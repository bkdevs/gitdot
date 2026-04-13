use std::{collections::HashSet, sync::LazyLock};

pub const GITDOT_SERVER_ID: &str = "gitdot-server";
pub const S2_SERVER_ID: &str = "s2-server";

pub const NOREPLY_EMAIL: &str = "gitdot <noreply@gitdot.io>";

const CODE_TEMPLATE: &str = include_str!("../../templates/code.html");

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

pub fn get_auth_email(code: &str) -> (String, String) {
    (
        format!("Your gitdot code: {}", code),
        CODE_TEMPLATE.replace("{{code}}", code),
    )
}
