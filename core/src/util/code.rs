pub const DEVICE_CODE_EXPIRY_MINUTES: i64 = 10;
pub const POLLING_INTERVAL_SECONDS: u64 = 1;

pub fn generate_device_code() -> String {
    use rand::Rng as _;
    let mut rng = rand::rng();
    (0..32)
        .map(|_| {
            let idx = rng.random_range(0..36);
            if idx < 10 {
                (b'0' + idx) as char
            } else {
                (b'a' + idx - 10) as char
            }
        })
        .collect()
}

pub fn generate_user_code() -> String {
    use rand::Rng as _;
    let mut rng = rand::rng();
    let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
    (0..6)
        .map(|_| chars[rng.random_range(0..chars.len())])
        .collect()
}
