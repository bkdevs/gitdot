//! Define newtype structs that can be shared across domains.

use nutype::nutype;

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 100),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub struct OwnerName(String);

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 100),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub struct RepositoryName(String);
