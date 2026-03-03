use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use s2_sdk::{
    S2,
    types::{BasinName, CreateBasinInput, CreateStreamInput, ErrorResponse, S2Error, StreamName},
};
use uuid::Uuid;

use crate::dto::{GITDOT_SERVER_ID, JwtClaims, S2_SERVER_ID};

#[async_trait]
pub trait S2Client: Send + Sync + Clone + 'static {
    async fn create_stream(&self, owner: &str, repo: &str, task_id: Uuid)
    -> Result<String, String>;
}

#[derive(Debug, Clone)]
pub struct S2ClientImpl {
    s2: S2,
    gitdot_private_key: String,
}

impl S2ClientImpl {
    pub fn new(server_url: &str, gitdot_private_key: String) -> Self {
        let s2 = S2::from_url(server_url).expect("valid S2 server URL");
        Self {
            s2,
            gitdot_private_key,
        }
    }

    fn issue_internal_jwt(&self) -> Result<String, String> {
        let now = Utc::now().timestamp() as usize;
        let claims = JwtClaims {
            iss: GITDOT_SERVER_ID.to_string(),
            aud: vec![S2_SERVER_ID.to_string()],
            sub: GITDOT_SERVER_ID.to_string(),
            iat: now,
            exp: now + 15,
        };

        let encoding_key = EncodingKey::from_ed_pem(self.gitdot_private_key.as_bytes())
            .map_err(|e| e.to_string())?;

        encode(&Header::new(Algorithm::EdDSA), &claims, &encoding_key).map_err(|e| e.to_string())
    }
}

#[async_trait]
impl S2Client for S2ClientImpl {
    async fn create_stream(
        &self,
        owner: &str,
        repo: &str,
        task_id: Uuid,
    ) -> Result<String, String> {
        let jwt = self.issue_internal_jwt()?;
        let s2 = self.s2.with_auth(&jwt);

        let basin_name_str = format!("{}-{}", owner.to_lowercase(), repo.to_lowercase());
        let basin_name: BasinName = basin_name_str
            .parse()
            .map_err(|_| format!("invalid basin name: {basin_name_str}"))?;

        match s2
            .create_basin(CreateBasinInput::new(basin_name.clone()))
            .await
        {
            Ok(_) => {}
            Err(S2Error::Server(ErrorResponse { code, .. }))
                if code == "resource_already_exists" => {}
            Err(e) => return Err(e.to_string()),
        }

        let stream_name_str = format!("task/{task_id}");
        let stream_name: StreamName = stream_name_str
            .parse()
            .map_err(|_| format!("invalid stream name: {stream_name_str}"))?;

        s2.basin(basin_name)
            .create_stream(CreateStreamInput::new(stream_name))
            .await
            .map_err(|e| e.to_string())?;

        Ok(format!(
            "s2://{}-{}/task/{task_id}",
            owner.to_lowercase(),
            repo.to_lowercase()
        ))
    }
}
