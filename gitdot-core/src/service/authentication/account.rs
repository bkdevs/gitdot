use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{
    client::{EmailClient, SmtpClient, TokenClient, TokenClientImpl},
    dto::{AddUserEmailRequest, UserEmailResponse, VerifyUserEmailRequest},
    error::{AccountError, ConflictError},
    repository::{
        EmailVerificationRepository, PgEmailVerificationRepository, PgUserRepository,
        UserRepository,
    },
    util::{
        auth::{NOREPLY_EMAIL, get_code_email},
        crypto::hash_string,
    },
};

/// Manages a user's account emails: adding secondary emails, issuing
/// emailed verification codes, and verifying them. Each code is single-use,
/// time-limited, and scoped to the requesting user.
#[async_trait]
pub trait AccountService: Send + Sync + 'static {
    /// Emails a verification code for the address without persisting anything to
    /// `core.user_emails` — the row is only created once the code is verified.
    /// Any prior active codes for the address are invalidated, so calling this
    /// again simply re-issues a fresh code. Fails with `Conflict` if the email
    /// is already verified by any user.
    async fn add_email(&self, request: AddUserEmailRequest) -> Result<(), AccountError>;

    /// Verifies a code previously emailed for `request.email`. On success the
    /// verified `user_emails` row is created and the code is marked used. Scoped
    /// to the caller — a code only verifies the address for `request.user_id`.
    async fn verify_email(
        &self,
        request: VerifyUserEmailRequest,
    ) -> Result<UserEmailResponse, AccountError>;
}

#[derive(Debug, Clone)]
pub struct AccountServiceImpl<UR, ER, EC, TC>
where
    UR: UserRepository,
    ER: EmailVerificationRepository,
    EC: EmailClient,
    TC: TokenClient,
{
    user_repo: UR,
    email_verification_repo: ER,
    email_client: EC,
    token_client: TC,
}

impl
    AccountServiceImpl<PgUserRepository, PgEmailVerificationRepository, SmtpClient, TokenClientImpl>
{
    pub fn new(
        user_repo: PgUserRepository,
        email_verification_repo: PgEmailVerificationRepository,
        email_client: SmtpClient,
        token_client: TokenClientImpl,
    ) -> Self {
        Self {
            user_repo,
            email_verification_repo,
            email_client,
            token_client,
        }
    }
}

impl<UR, ER, EC, TC> AccountServiceImpl<UR, ER, EC, TC>
where
    UR: UserRepository,
    ER: EmailVerificationRepository,
    EC: EmailClient,
    TC: TokenClient,
{
    async fn issue_and_send_code(
        &self,
        user_id: uuid::Uuid,
        email: &str,
    ) -> Result<(), AccountError> {
        self.email_verification_repo
            .invalidate_codes_for_email(user_id, email)
            .await?;

        let (code, code_hash) = self.token_client.generate_readable_code();
        let expires_at = Utc::now()
            + Duration::seconds(self.token_client.get_auth_code_expiry_in_seconds() as i64);
        self.email_verification_repo
            .create_code(user_id, email, &code_hash, expires_at)
            .await?;

        let (subject, html) = get_code_email(&code);
        self.email_client
            .send_email(NOREPLY_EMAIL, email, &subject, &html)
            .await?;

        Ok(())
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<UR, ER, EC, TC> AccountService for AccountServiceImpl<UR, ER, EC, TC>
where
    UR: UserRepository,
    ER: EmailVerificationRepository,
    EC: EmailClient,
    TC: TokenClient,
{
    async fn add_email(&self, request: AddUserEmailRequest) -> Result<(), AccountError> {
        let email = request.email.as_ref();

        if self.user_repo.is_email_taken(email).await? {
            return Err(ConflictError::new("email", email).into());
        }

        self.issue_and_send_code(request.user_id, email).await?;

        Ok(())
    }

    async fn verify_email(
        &self,
        request: VerifyUserEmailRequest,
    ) -> Result<UserEmailResponse, AccountError> {
        let email = request.email.as_ref();

        let code_hash = hash_string(request.code.as_ref());
        let code = self
            .email_verification_repo
            .get_code_by_hash(&code_hash)
            .await?
            .ok_or(AccountError::InvalidCode)?;

        if code.user_id != request.user_id
            || code.email != email
            || code.used_at.is_some()
            || code.expires_at < Utc::now()
        {
            return Err(AccountError::InvalidCode);
        }

        let row = match self
            .email_verification_repo
            .mark_code_used_and_add_email(code.id, request.user_id, email)
            .await
        {
            Ok(row) => row,
            Err(e) if e.is_unique_violation() => {
                return Err(ConflictError::new("email", email).into());
            }
            Err(e) => return Err(e.into()),
        };

        Ok(row.into())
    }
}
