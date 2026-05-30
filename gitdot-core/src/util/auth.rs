use std::{collections::HashSet, sync::LazyLock};

use rustrict::CensorStr;

use crate::error::ConflictError;

pub const GITDOT_SERVER_ID: &str = "gitdot-server";
pub const S2_SERVER_ID: &str = "s2-server";

pub const NOREPLY_EMAIL: &str = "gitdot <noreply@gitdot.io>";

const CODE_TEMPLATE: &str = include_str!("../../templates/email/code.html");

/// Reserved usernames that cannot be used for user or organization names.
static RESERVED_NAMES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        // Frontend routes
        "api",
        "company",
        "decisions",
        "faq",
        "home",
        "login",
        "oauth",
        "onboarding",
        "privacy",
        "releases",
        "terms",
        "weeks",
        // Redirect sources
        "beta",
        "signup",
        // Common reserved
        "admin",
    ])
});

/// Validates a user or organization name, rejecting reserved and offensive
/// names. `entity` labels the conflict (e.g. `"organization"`, `"user name"`).
pub fn validate_name(entity: &'static str, name: &str) -> Result<(), ConflictError> {
    if is_reserved_name(name) {
        return Err(ConflictError::new(entity, format!("{name} is reserved")));
    }
    if is_offensive_name(name) {
        return Err(ConflictError::new(entity, format!("{name} is not allowed")));
    }
    Ok(())
}

pub fn is_reserved_name(name: &str) -> bool {
    RESERVED_NAMES.contains(name.to_lowercase().as_str())
}

pub fn is_offensive_name(name: &str) -> bool {
    name.is_inappropriate()
}

pub fn get_code_email(code: &str) -> (String, String) {
    (
        format!("Your gitdot code: {}", code),
        CODE_TEMPLATE.replace("{{code}}", code),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserved_names_are_detected_case_insensitively() {
        assert!(is_reserved_name("admin"));
        assert!(is_reserved_name("ADMIN"));
        assert!(!is_reserved_name("johndoe"));
    }

    #[test]
    fn offensive_names_are_flagged() {
        assert!(is_offensive_name("fuck"));
        assert!(is_offensive_name("f-u-c-k"));
    }

    #[test]
    fn validate_name_rejects_reserved_and_offensive_but_allows_benign() {
        assert!(validate_name("organization", "admin").is_err());
        assert!(validate_name("user name", "fuck").is_err());
        assert!(validate_name("organization", "acme-corp").is_ok());
    }

    #[test]
    fn benign_names_are_allowed() {
        assert!(!is_offensive_name("johndoe"));
        assert!(!is_offensive_name("user123"));
        assert!(!is_offensive_name("my-org"));
        assert!(!is_offensive_name("acme_corp"));
    }
}
