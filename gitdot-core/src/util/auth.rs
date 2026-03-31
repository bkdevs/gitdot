use std::{collections::HashSet, sync::LazyLock};

// TODO: make configurable
pub const NOREPLY_EMAIL: &str = "gitdot <noreply@gitdot.io>";
pub const SITE_URL: &str = "https://gitdot.io";

/// Reserved usernames that cannot be used for user or organization names.
static RESERVED_NAMES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        // Frontend routes
        "login",
        "signup",
        "home",
        "settings",
        "search",
        "notifications",
        "auth",
        "oauth",
        "week",
        "beta",
        // Common reserved
        "admin",
    ])
});

pub fn is_reserved_name(name: &str) -> bool {
    RESERVED_NAMES.contains(name.to_lowercase().as_str())
}

// TODO: move html to resources
pub fn get_auth_email(is_signup: bool, code: &str) -> (String, String) {
    if is_signup {
        let link = format!("{SITE_URL}/auth/verify?code={code}&redirect=%2Fonboarding");
        (
            "Confirm your gitdot email".to_string(),
            format!(
                "<p>\n\
                 Click to continue: <a href=\"{link}\">confirm</a>\n\
                 </p>\n\
                 \n\
                 <p>\n\
                 \u{2014}gitdot\n\
                 </p>"
            ),
        )
    } else {
        let link = format!("{SITE_URL}/auth/verify?code={code}");
        (
            "Login to gitdot".to_string(),
            format!(
                "<p>\n\
                 Click to continue: <a href=\"{link}\">login</a>\n\
                 </p>\n\
                 \n\
                 <p>\n\
                 \u{2014}gitdot\n\
                 </p>"
            ),
        )
    }
}
