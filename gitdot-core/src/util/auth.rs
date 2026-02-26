use std::{collections::HashSet, sync::LazyLock};

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
