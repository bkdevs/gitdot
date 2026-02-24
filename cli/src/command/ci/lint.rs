use itertools::Either;
use tombi_config::TomlVersion;
use tombi_diagnostic::printer::{Pretty, Print};
use tombi_linter::{LintOptions, Linter};
use tombi_schema_store::SchemaStore;

pub async fn lint() -> anyhow::Result<()> {
    let config_path = crate::util::ci::find_config().await?;
    if !config_path.exists() {
        anyhow::bail!(
            "{} not found. Run `gitdot ci init` to create one.",
            config_path.display()
        );
    }
    let source = tokio::fs::read_to_string(&config_path).await?;

    let schema_store = SchemaStore::new();
    let options = LintOptions::default();
    let linter = Linter::new(
        TomlVersion::V1_1_0,
        &options,
        Some(Either::Right(config_path.as_path())),
        &schema_store,
    );

    match linter.lint(&source).await {
        Ok(()) => {
            println!("{}: no issues found", config_path.display());
            Ok(())
        }
        Err(diagnostics) => {
            let mut printer = Pretty::default();
            for d in &diagnostics {
                d.clone().with_source_file(&config_path).print(&mut printer);
            }
            anyhow::bail!("{}: {} issue(s) found", config_path.display(), diagnostics.len())
        }
    }
}
