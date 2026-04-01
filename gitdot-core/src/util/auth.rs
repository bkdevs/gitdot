use std::{collections::HashSet, sync::LazyLock};

pub const GITDOT_SERVER_ID: &str = "gitdot-server";
pub const S2_SERVER_ID: &str = "s2-server";

// TODO: make configurable
pub const NOREPLY_EMAIL: &str = "gitdot <noreply@gitdot.io>";
pub const SITE_URL: &str = "https://gitdot.io";

const SIGNUP_TEMPLATE: &str = include_str!("../../templates/signup.html");
const LOGIN_TEMPLATE: &str = include_str!("../../templates/login.html");

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

pub fn get_auth_email(is_signup: bool, code: &str) -> (String, String) {
    if is_signup {
        let link = format!("{SITE_URL}/auth/verify?code={code}&redirect=%2Fonboarding");
        (
            "Confirm your gitdot email".to_string(),
            SIGNUP_TEMPLATE.replace("{{link}}", &link),
        )
    } else {
        let link = format!("{SITE_URL}/auth/verify?code={code}");
        (
            "Login to gitdot".to_string(),
            LOGIN_TEMPLATE.replace("{{link}}", &link),
        )
    }
}
