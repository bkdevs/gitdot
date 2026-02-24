use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BuildConfig {
    pub builds: Vec<BuildSpec>,
    pub tasks: Vec<TaskConfig>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTrigger {
    PullRequest,
    PushToMain,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BuildSpec {
    pub trigger: BuildTrigger,
    pub tasks: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskConfig {
    pub name: String,
    pub command: String,
    pub runs_after: Option<Vec<String>>,
}
