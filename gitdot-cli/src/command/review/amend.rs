use crate::{
    config::UserConfig,
    git::GitWrapper,
    util::review::{clear_review_branch, load_review_branch},
};

pub async fn amend_review(_config: UserConfig, git: &GitWrapper) -> anyhow::Result<()> {
    let original_hash = git.rev_parse("HEAD").await?;

    git.add_all().await?;
    git.commit_amend_no_edit().await?;

    let checkout_branch = load_review_branch(git).await?;
    if let Some(branch) = &checkout_branch {
        let new_hash = git.rev_parse("HEAD").await?;
        git.rebase_onto(&new_hash, &original_hash, branch).await?;
        clear_review_branch(git).await?;
        println!("Amended and rebased onto {}", branch);
    } else {
        println!("Amended commit");
    }

    Ok(())
}
