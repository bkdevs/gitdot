use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CiConfig {
    pub pull_request: PullRequestConfig,
    pub push_to_main: PushToMainConfig,
    pub tasks: Vec<TaskConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PullRequestConfig {
    pub tasks: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PushToMainConfig {
    pub tasks: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskConfig {
    pub name: String,
    pub script: String,
    pub runs_after: Option<Vec<String>>,
}
