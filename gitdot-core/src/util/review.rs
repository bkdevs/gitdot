pub const MAGIC_REF_PREFIX: &str = "refs/for";

pub fn get_target_ref(branch: &str) -> String {
    format!("refs/heads/{}", branch)
}

pub fn get_head_ref(review_number: i32) -> String {
    format!("refs/reviews/{}/head", review_number)
}

pub fn get_revision_ref(review_number: i32, diff_position: i32, revision_number: i32) -> String {
    format!(
        "refs/reviews/{}/diffs/{}/revisions/{}",
        review_number, diff_position, revision_number
    )
}

pub fn get_current_ref(review_number: i32, diff_position: i32) -> String {
    format!(
        "refs/reviews/{}/diffs/{}/current",
        review_number, diff_position
    )
}
