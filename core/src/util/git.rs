pub const REPO_SUFFIX: &str = ".git";
pub const DEFAULT_BRANCH: &str = "main";
pub const EMPTY_TREE_REF: &str = "EMPTY_TREE_REF";
pub const DEFAULT_PREVIEW_LINES: u32 = 100;

/// include git hook scripts in the binary during compilation
pub const POST_RECEIVE_SCRIPT: &str = include_str!("../../../scripts/post-receive");

/// server-side git hook types
pub enum GitHookType {
    PreReceive,
    PostReceive,
    Update,
}

impl GitHookType {
    pub fn as_str(&self) -> &str {
        match self {
            GitHookType::PreReceive => "pre-receive",
            GitHookType::PostReceive => "post-receive",
            GitHookType::Update => "update",
        }
    }
}
