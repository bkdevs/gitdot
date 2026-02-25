use octocrab::models::Repository;

#[derive(Debug, Clone)]
pub struct GitHubRepositoryResponse {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub private: bool,
    pub default_branch: String,
}

impl From<Repository> for GitHubRepositoryResponse {
    fn from(r: Repository) -> Self {
        Self {
            id: r.id.into_inner(),
            name: r.name,
            full_name: r.full_name.unwrap_or_default(),
            description: r.description,
            private: r.private.unwrap_or(false),
            default_branch: r.default_branch.unwrap_or_else(|| "main".to_string()),
        }
    }
}

pub type ListGitHubInstallationRepositoriesResponse = Vec<GitHubRepositoryResponse>;
