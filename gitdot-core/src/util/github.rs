pub fn get_github_clone_url(access_token: &str, full_name: &str) -> String {
    format!(
        "https://x-access-token:{}@github.com/{}.git",
        access_token, full_name
    )
}
