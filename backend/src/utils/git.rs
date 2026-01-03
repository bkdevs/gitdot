// repo is expected to contain .git suffix
static REPO_SUFFIX: &str = ".git";

pub fn normalize_repo_name(repo_name: &str) -> String {
    format!(
        "{}{}",
        repo_name.strip_suffix(REPO_SUFFIX).unwrap_or(repo_name),
        REPO_SUFFIX
    )
}
