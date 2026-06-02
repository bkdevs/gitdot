use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{
    client::{TokenClient, TokenClientImpl},
    dto::{
        AuthorizeDeviceRequest, DeviceCodeRequest, DeviceCodeResponse, PollTokenRequest,
        TokenResponse,
    },
    error::{DeviceError, InputError, OptionNotFoundExt},
    model::{DeviceAuthorizationStatus, TokenType},
    repository::{
        DeviceRepository, PgDeviceRepository, PgTokenRepository, PgUserRepository, TokenRepository,
        UserRepository,
    },
    util::crypto::hash_string,
};

/// Implements the OAuth 2.0 device authorization flow. A client requests a
/// device/user code pair, the user approves the user code in a browser, and
/// the client polls until it can exchange the device code for an access token.
#[async_trait]
pub trait DeviceService: Send + Sync + 'static {
    /// Starts a device authorization: generates a device code (returned in the
    /// clear, stored only as a hash), a human-readable user code, and persists
    /// a pending authorization that expires after the configured device-code
    /// window. Returns the codes alongside the verification URL, expiry, and
    /// the recommended polling interval.
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, DeviceError>;

    /// Polls the status of a pending authorization by device code and, once
    /// authorized, issues a personal access token for the approving user.
    ///
    /// A successful authorization consumes the request: the device
    /// authorization is expired after the token is created, so the token can
    /// be retrieved only once.
    ///
    /// # Errors
    /// - [`DeviceError::NotFound`] — no authorization matches the device code
    /// - [`DeviceError::TokenPending`] — user has not yet approved the user code
    /// - [`DeviceError::TokenExpired`] — the device code lapsed before approval
    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, DeviceError>;

    /// Approves a pending authorization by binding the user code to the
    /// approving user, unblocking the client's next `poll_token` call.
    ///
    /// # Errors
    /// - [`DeviceError::NotFound`] — no pending authorization matches the user code
    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), DeviceError>;
}

#[derive(Debug, Clone)]
pub struct DeviceServiceImpl<DR, TR, UR, TC>
where
    DR: DeviceRepository,
    TR: TokenRepository,
    UR: UserRepository,
    TC: TokenClient,
{
    device_repo: DR,
    token_repo: TR,
    user_repo: UR,
    token_client: TC,
}

impl DeviceServiceImpl<PgDeviceRepository, PgTokenRepository, PgUserRepository, TokenClientImpl> {
    pub fn new(
        device_repo: PgDeviceRepository,
        token_repo: PgTokenRepository,
        user_repo: PgUserRepository,
        token_client: TokenClientImpl,
    ) -> Self {
        Self {
            device_repo,
            token_repo,
            user_repo,
            token_client,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<DR, TR, UR, TC> DeviceService for DeviceServiceImpl<DR, TR, UR, TC>
where
    DR: DeviceRepository,
    TR: TokenRepository,
    UR: UserRepository,
    TC: TokenClient,
{
    async fn request_device_code(
        &self,
        request: DeviceCodeRequest,
    ) -> Result<DeviceCodeResponse, DeviceError> {
        let (device_code, device_code_hash) = self.token_client.generate_high_entropic_code();
        let (user_code, user_code_hash) = self.token_client.generate_readable_code();
        let expiry_secs = self.token_client.get_device_code_expiry_in_seconds();
        let expires_at = Utc::now() + Duration::seconds(expiry_secs as i64);

        self.device_repo
            .create_device_authorization(
                &device_code_hash,
                &user_code_hash,
                &request.client_id,
                expires_at,
            )
            .await?;

        Ok(DeviceCodeResponse {
            device_code,
            user_code,
            verification_url: request.verification_url,
            expires_in: expiry_secs,
            interval: self.token_client.get_polling_interval_in_seconds(),
        })
    }

    async fn poll_token(&self, request: PollTokenRequest) -> Result<TokenResponse, DeviceError> {
        let device_code_hash = hash_string(&request.device_code);
        let device_auth = self
            .device_repo
            .get_device_authorization_by_device_code_hash(&device_code_hash)
            .await?
            .or_not_found("device_code", &device_code_hash)?;

        if device_auth.expires_at < Utc::now()
            && device_auth.status == DeviceAuthorizationStatus::Pending
        {
            self.device_repo
                .expire_device_authorization(device_auth.id)
                .await?;
            return Err(DeviceError::TokenExpired("device_code".into()));
        }

        match device_auth.status {
            DeviceAuthorizationStatus::Pending => {
                Err(DeviceError::TokenPending("device_code".into()))
            }
            DeviceAuthorizationStatus::Expired => {
                Err(DeviceError::TokenExpired("device_code".into()))
            }
            DeviceAuthorizationStatus::Authorized => {
                let user_id = device_auth
                    .user_id
                    .ok_or(InputError::new("user_id", "missing"))?;

                let user = self
                    .user_repo
                    .get_by_id(user_id)
                    .await?
                    .or_not_found("user", user_id)?;
                let user_email = user
                    .primary_email()
                    .or_not_found("user_email", user_id)?
                    .email
                    .clone();

                let (access_token, token_hash) = self
                    .token_client
                    .generate_access_token(&TokenType::Personal);

                self.token_repo
                    .create_token(
                        user_id,
                        &device_auth.client_id,
                        &token_hash,
                        TokenType::Personal,
                    )
                    .await?;

                self.device_repo
                    .expire_device_authorization(device_auth.id)
                    .await?;

                Ok(TokenResponse {
                    access_token,
                    user_name: user.name,
                    user_email,
                })
            }
        }
    }

    async fn authorize_device(&self, request: AuthorizeDeviceRequest) -> Result<(), DeviceError> {
        let user_code_hash = hash_string(request.user_code.as_ref());
        self.device_repo
            .authorize_device(&user_code_hash, request.user_id)
            .await?
            .or_not_found("user_code", &user_code_hash)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use chrono::{Duration, Utc};
    use uuid::Uuid;

    use super::{DeviceService, DeviceServiceImpl};
    use crate::{
        dto::{AuthorizeDeviceRequest, DeviceCodeRequest, PollTokenRequest},
        error::DeviceError,
        model::{AccessToken, DeviceAuthorizationStatus, TokenType},
        service::{
            test_client::MockTokenClient,
            test_common::{create_device_authorization, create_user, create_user_email},
            test_repository::{MockDeviceRepository, MockTokenRepository, MockUserRepository},
        },
    };

    type Service = DeviceServiceImpl<
        MockDeviceRepository,
        MockTokenRepository,
        MockUserRepository,
        MockTokenClient,
    >;

    fn create_service(
        device_repo: MockDeviceRepository,
        token_repo: MockTokenRepository,
        user_repo: MockUserRepository,
    ) -> Service {
        DeviceServiceImpl {
            device_repo,
            token_repo,
            user_repo,
            token_client: MockTokenClient::default(),
        }
    }

    fn poll_request() -> PollTokenRequest {
        PollTokenRequest::new("device-code".to_string(), "gitdot-cli".to_string())
    }

    mod request_device_code {
        use super::*;

        #[tokio::test]
        async fn returns_codes_and_persists() {
            let mut device_repo = MockDeviceRepository::new();
            device_repo
                .expect_create_device_authorization()
                .times(1)
                .returning(|_, _, _, _| {
                    Ok(create_device_authorization(
                        DeviceAuthorizationStatus::Pending,
                    ))
                });

            let service = create_service(
                device_repo,
                MockTokenRepository::new(),
                MockUserRepository::new(),
            );

            let response = service
                .request_device_code(DeviceCodeRequest::new(
                    "gitdot-cli".to_string(),
                    "https://gitdot.dev/device".to_string(),
                ))
                .await
                .unwrap();

            // Raw codes from the token client are returned in the clear; the
            // verification URL is echoed; expiry/interval come from the client.
            assert_eq!(response.device_code, "refresh-token-raw");
            assert_eq!(response.user_code, "ABC234");
            assert_eq!(response.verification_url, "https://gitdot.dev/device");
            assert_eq!(response.expires_in, 600);
            assert_eq!(response.interval, 1);
        }

        #[tokio::test]
        async fn propagates_repo_error() {
            let mut device_repo = MockDeviceRepository::new();
            device_repo
                .expect_create_device_authorization()
                .returning(|_, _, _, _| {
                    Err(crate::error::DatabaseError::Other(sqlx::Error::RowNotFound))
                });

            let service = create_service(
                device_repo,
                MockTokenRepository::new(),
                MockUserRepository::new(),
            );

            let err = service
                .request_device_code(DeviceCodeRequest::new(
                    "gitdot-cli".to_string(),
                    "https://gitdot.dev/device".to_string(),
                ))
                .await
                .unwrap_err();

            assert!(matches!(err, DeviceError::DatabaseError(_)));
        }
    }

    mod poll_token {
        use super::*;

        #[tokio::test]
        async fn unknown_device_code_is_not_found() {
            let mut device_repo = MockDeviceRepository::new();
            device_repo
                .expect_get_device_authorization_by_device_code_hash()
                .returning(|_| Ok(None));

            let service = create_service(
                device_repo,
                MockTokenRepository::new(),
                MockUserRepository::new(),
            );

            let err = service.poll_token(poll_request()).await.unwrap_err();
            assert!(matches!(err, DeviceError::NotFound(_)));
        }

        #[tokio::test]
        async fn pending_is_token_pending() {
            let mut device_repo = MockDeviceRepository::new();
            device_repo
                .expect_get_device_authorization_by_device_code_hash()
                .returning(|_| {
                    Ok(Some(create_device_authorization(
                        DeviceAuthorizationStatus::Pending,
                    )))
                });

            // No `expect_expire_device_authorization`: an unexpired pending auth
            // must not be touched (mockall panics on an unexpected call).
            let service = create_service(
                device_repo,
                MockTokenRepository::new(),
                MockUserRepository::new(),
            );

            let err = service.poll_token(poll_request()).await.unwrap_err();
            assert!(matches!(err, DeviceError::TokenPending(_)));
        }

        #[tokio::test]
        async fn pending_but_expired_marks_expired() {
            let mut auth = create_device_authorization(DeviceAuthorizationStatus::Pending);
            auth.expires_at = Utc::now() - Duration::minutes(1);

            let mut device_repo = MockDeviceRepository::new();
            device_repo
                .expect_get_device_authorization_by_device_code_hash()
                .returning(move |_| Ok(Some(auth.clone())));
            device_repo
                .expect_expire_device_authorization()
                .times(1)
                .returning(|_| Ok(()));

            let service = create_service(
                device_repo,
                MockTokenRepository::new(),
                MockUserRepository::new(),
            );

            let err = service.poll_token(poll_request()).await.unwrap_err();
            assert!(matches!(err, DeviceError::TokenExpired(_)));
        }

        #[tokio::test]
        async fn expired_status_is_token_expired() {
            let mut device_repo = MockDeviceRepository::new();
            device_repo
                .expect_get_device_authorization_by_device_code_hash()
                .returning(|_| {
                    Ok(Some(create_device_authorization(
                        DeviceAuthorizationStatus::Expired,
                    )))
                });

            let service = create_service(
                device_repo,
                MockTokenRepository::new(),
                MockUserRepository::new(),
            );

            let err = service.poll_token(poll_request()).await.unwrap_err();
            assert!(matches!(err, DeviceError::TokenExpired(_)));
        }

        #[tokio::test]
        async fn authorized_issues_token() {
            let user = {
                let mut user = create_user("alice");
                let mut email = create_user_email(user.id, "alice@example.com");
                email.is_primary = true;
                user.emails = vec![email];
                user
            };
            let user_id = user.id;

            let mut auth = create_device_authorization(DeviceAuthorizationStatus::Authorized);
            auth.user_id = Some(user_id);
            let auth_id = auth.id;

            let mut device_repo = MockDeviceRepository::new();
            device_repo
                .expect_get_device_authorization_by_device_code_hash()
                .returning(move |_| Ok(Some(auth.clone())));
            // The authorization is consumed: expired right after the token is issued.
            device_repo
                .expect_expire_device_authorization()
                .withf(move |id| *id == auth_id)
                .times(1)
                .returning(|_| Ok(()));

            let mut user_repo = MockUserRepository::new();
            user_repo
                .expect_get_by_id()
                .returning(move |_| Ok(Some(user.clone())));

            // Capture the token-creation args to assert the principal, client,
            // and token type without relying on mockall reference-matcher quirks.
            let created = Arc::new(Mutex::new(None));
            let recorder = created.clone();
            let mut token_repo = MockTokenRepository::new();
            token_repo.expect_create_token().times(1).returning(
                move |principal_id, client_id, token_hash, token_type| {
                    *recorder.lock().unwrap() =
                        Some((principal_id, client_id.to_string(), token_type.clone()));
                    Ok(AccessToken {
                        id: Uuid::new_v4(),
                        principal_id,
                        client_id: client_id.to_string(),
                        token_hash: token_hash.to_string(),
                        token_type,
                        created_at: Utc::now(),
                        last_used_at: None,
                    })
                },
            );

            let service = create_service(device_repo, token_repo, user_repo);
            let response = service.poll_token(poll_request()).await.unwrap();

            assert_eq!(response.access_token, "access-token-raw");
            assert_eq!(response.user_name, "alice");
            assert_eq!(response.user_email, "alice@example.com");

            let (principal_id, client_id, token_type) = created.lock().unwrap().clone().unwrap();
            assert_eq!(principal_id, user_id);
            assert_eq!(client_id, "gitdot-cli");
            assert_eq!(token_type, TokenType::Personal);
        }

        #[tokio::test]
        async fn authorized_without_user_is_input_error() {
            // Default authorized fixture has `user_id: None`.
            let mut device_repo = MockDeviceRepository::new();
            device_repo
                .expect_get_device_authorization_by_device_code_hash()
                .returning(|_| {
                    Ok(Some(create_device_authorization(
                        DeviceAuthorizationStatus::Authorized,
                    )))
                });

            let service = create_service(
                device_repo,
                MockTokenRepository::new(),
                MockUserRepository::new(),
            );

            let err = service.poll_token(poll_request()).await.unwrap_err();
            assert!(matches!(err, DeviceError::Input(_)));
        }

        #[tokio::test]
        async fn authorized_user_missing_is_not_found() {
            let mut auth = create_device_authorization(DeviceAuthorizationStatus::Authorized);
            auth.user_id = Some(Uuid::new_v4());

            let mut device_repo = MockDeviceRepository::new();
            device_repo
                .expect_get_device_authorization_by_device_code_hash()
                .returning(move |_| Ok(Some(auth.clone())));

            let mut user_repo = MockUserRepository::new();
            user_repo.expect_get_by_id().returning(|_| Ok(None));

            let service = create_service(device_repo, MockTokenRepository::new(), user_repo);
            let err = service.poll_token(poll_request()).await.unwrap_err();
            assert!(matches!(err, DeviceError::NotFound(_)));
        }

        #[tokio::test]
        async fn authorized_without_primary_email_is_not_found() {
            // `create_user` has no emails, so there is no primary email.
            let user = create_user("alice");
            let mut auth = create_device_authorization(DeviceAuthorizationStatus::Authorized);
            auth.user_id = Some(user.id);

            let mut device_repo = MockDeviceRepository::new();
            device_repo
                .expect_get_device_authorization_by_device_code_hash()
                .returning(move |_| Ok(Some(auth.clone())));

            let mut user_repo = MockUserRepository::new();
            user_repo
                .expect_get_by_id()
                .returning(move |_| Ok(Some(user.clone())));

            let service = create_service(device_repo, MockTokenRepository::new(), user_repo);
            let err = service.poll_token(poll_request()).await.unwrap_err();
            assert!(matches!(err, DeviceError::NotFound(_)));
        }
    }

    mod authorize_device {
        use super::*;

        #[tokio::test]
        async fn success() {
            let mut device_repo = MockDeviceRepository::new();
            device_repo
                .expect_authorize_device()
                .times(1)
                .returning(|_, _| {
                    Ok(Some(create_device_authorization(
                        DeviceAuthorizationStatus::Authorized,
                    )))
                });

            let service = create_service(
                device_repo,
                MockTokenRepository::new(),
                MockUserRepository::new(),
            );

            service
                .authorize_device(AuthorizeDeviceRequest::new("ABC234", Uuid::new_v4()).unwrap())
                .await
                .unwrap();
        }

        #[tokio::test]
        async fn no_match_is_not_found() {
            let mut device_repo = MockDeviceRepository::new();
            device_repo
                .expect_authorize_device()
                .returning(|_, _| Ok(None));

            let service = create_service(
                device_repo,
                MockTokenRepository::new(),
                MockUserRepository::new(),
            );

            let err = service
                .authorize_device(AuthorizeDeviceRequest::new("ABC234", Uuid::new_v4()).unwrap())
                .await
                .unwrap_err();

            assert!(matches!(err, DeviceError::NotFound(_)));
        }

        #[tokio::test]
        async fn invalid_user_code_is_input_error() {
            // The request constructor rejects malformed user codes before the
            // service is ever called.
            let err = AuthorizeDeviceRequest::new("bad", Uuid::new_v4()).unwrap_err();
            assert!(matches!(err, DeviceError::Input(_)));
        }
    }
}
