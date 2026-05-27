use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{
    client::{EmailClient, SmtpClient, TokenClient, TokenClientImpl},
    dto::{AddUserEmailRequest, UserEmailResponse, VerifyUserEmailRequest},
    error::{ConflictError, DatabaseError, EmailVerificationError, OptionNotFoundExt},
    repository::{
        EmailVerificationRepository, EmailVerificationRepositoryImpl, UserRepository,
        UserRepositoryImpl,
    },
    util::{
        auth::{NOREPLY_EMAIL, get_verify_email_email},
        crypto::hash_string,
    },
};

#[async_trait]
pub trait EmailVerificationService: Send + Sync + 'static {
    /// Adds an email to the user's account in an unverified state and emails a
    /// verification code. Idempotent for the same user's unverified row:
    /// re-calling acts as a resend (issues a new code, invalidating prior ones).
    async fn add_email(
        &self,
        request: AddUserEmailRequest,
    ) -> Result<UserEmailResponse, EmailVerificationError>;

    /// Verifies a code previously emailed for `request.email`. On success the
    /// `user_emails` row flips to verified and the code is marked used. Scoped
    /// to the caller — a code only verifies the email row belonging to
    /// `request.user_id`.
    async fn verify_email(
        &self,
        request: VerifyUserEmailRequest,
    ) -> Result<UserEmailResponse, EmailVerificationError>;
}

#[derive(Debug, Clone)]
pub struct EmailVerificationServiceImpl<UR, ER, EC, TC>
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
    EmailVerificationServiceImpl<
        UserRepositoryImpl,
        EmailVerificationRepositoryImpl,
        SmtpClient,
        TokenClientImpl,
    >
{
    pub fn new(
        user_repo: UserRepositoryImpl,
        email_verification_repo: EmailVerificationRepositoryImpl,
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

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<UR, ER, EC, TC> EmailVerificationService for EmailVerificationServiceImpl<UR, ER, EC, TC>
where
    UR: UserRepository,
    ER: EmailVerificationRepository,
    EC: EmailClient,
    TC: TokenClient,
{
    async fn add_email(
        &self,
        request: AddUserEmailRequest,
    ) -> Result<UserEmailResponse, EmailVerificationError> {
        let email = request.email.as_ref();

        // Resolve the user_email row: insert a new unverified row, or reuse the
        // caller's existing unverified row (resend). Conflict when the email
        // already belongs to another user, or to this user but verified.
        let user_email_id = match self.user_repo.get_email_owner(email).await? {
            Some((id, owner_id, is_verified)) => {
                if owner_id != request.user_id || is_verified {
                    return Err(ConflictError::new("email", email).into());
                }
                id
            }
            None => match self.user_repo.create_email(request.user_id, email).await {
                Ok(_) => self
                    .user_repo
                    .get_email_owner(email)
                    .await?
                    .map(|(id, _, _)| id)
                    .or_not_found("user_email", email)?,
                Err(DatabaseError::Other(e))
                    if e.as_database_error().and_then(|db| db.code()).as_deref()
                        == Some("23505") =>
                {
                    return Err(ConflictError::new("email", email).into());
                }
                Err(e) => return Err(e.into()),
            },
        };

        self.email_verification_repo
            .invalidate_codes_for_email(user_email_id)
            .await?;

        let code = self.token_client.generate_readable_code();
        let code_hash = hash_string(&code);
        let expires_at = Utc::now()
            + Duration::seconds(self.token_client.get_auth_code_expiry_in_seconds() as i64);
        self.email_verification_repo
            .create_code(user_email_id, &code_hash, expires_at)
            .await?;

        let (subject, html) = get_verify_email_email(&code);
        self.email_client
            .send_email(NOREPLY_EMAIL, email, &subject, &html)
            .await?;

        let row = self
            .user_repo
            .list_emails(request.user_id)
            .await?
            .into_iter()
            .find(|e| e.email == email)
            .or_not_found("user_email", email)?;
        Ok(row.into())
    }

    async fn verify_email(
        &self,
        request: VerifyUserEmailRequest,
    ) -> Result<UserEmailResponse, EmailVerificationError> {
        let email = request.email.as_ref();

        // Locate the caller's row for this email. Any mismatch (not present, or
        // owned by another user) collapses to InvalidCode so we don't leak
        // ownership information.
        let user_email_id = match self.user_repo.get_email_owner(email).await? {
            Some((id, owner_id, _)) if owner_id == request.user_id => id,
            _ => return Err(EmailVerificationError::InvalidCode),
        };

        let code_hash = hash_string(request.code.as_ref());
        let code = self
            .email_verification_repo
            .get_code_by_hash(&code_hash)
            .await?
            .ok_or(EmailVerificationError::InvalidCode)?;

        if code.user_email_id != user_email_id
            || code.used_at.is_some()
            || code.expires_at < Utc::now()
        {
            return Err(EmailVerificationError::InvalidCode);
        }

        self.email_verification_repo
            .mark_code_used_and_verify_email(code.id, user_email_id)
            .await?;

        let row = self
            .user_repo
            .list_emails(request.user_id)
            .await?
            .into_iter()
            .find(|e| e.email == email)
            .or_not_found("user_email", email)?;
        Ok(row.into())
    }
}
