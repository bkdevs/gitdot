use async_trait::async_trait;
use s2_sdk::{
    S2,
    types::{BasinName, CreateBasinInput, CreateStreamInput, ErrorResponse, S2Error, StreamName},
};
use uuid::Uuid;

#[async_trait]
pub trait S2Client: Send + Sync + Clone + 'static {
    async fn create_stream(&self, owner: &str, repo: &str, task_id: Uuid)
    -> Result<String, String>;
}

#[derive(Debug, Clone)]
pub struct S2ClientImpl {
    s2: S2,
}

impl S2ClientImpl {
    pub fn new(server_url: &str) -> Self {
        let s2 = S2::from_url(server_url).expect("valid S2 server URL");
        Self { s2 }
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
        let basin_name_str = format!("{}.{}", owner.to_lowercase(), repo.to_lowercase());
        let basin_name: BasinName = basin_name_str
            .parse()
            .map_err(|_| format!("invalid basin name: {basin_name_str}"))?;

        match self
            .s2
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

        self.s2
            .basin(basin_name)
            .create_stream(CreateStreamInput::new(stream_name))
            .await
            .map_err(|e| e.to_string())?;

        Ok(format!(
            "s2://{}.{}/task/{task_id}",
            owner.to_lowercase(),
            repo.to_lowercase()
        ))
    }
}
