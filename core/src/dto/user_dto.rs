use nutype::nutype;

#[derive(Debug, Clone)]
pub struct FindUserByNameRequest {
    pub name: UserName,
}

impl FindUserByNameRequest {
    pub fn new(user_name: String) -> Self {
        Self {
            name: UserName::try_new(user_name).unwrap(),
        }
    }
}

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 100),
    derive(
        Debug,
        Clone,
        Serialize,
        Deserialize,
        PartialEq,
        Eq,
        TryFrom,
        AsRef,
        Deref,
    )
)]
pub struct UserName(String);
