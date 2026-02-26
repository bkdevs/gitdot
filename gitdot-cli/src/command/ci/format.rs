use crate::util::ci;

use itertools::Either;
use tombi_config::TomlVersion;
use tombi_diagnostic::printer::{Pretty, Print};
use tombi_formatter::{FormatOptions, Formatter};
use tombi_schema_store::SchemaStore;

pub async fn format() -> anyhow::Result<()> {
    let config_path = ci::find_config().await?;
    if !config_path.exists() {
        anyhow::bail!(
            "{} not found. Run `gitdot ci init` to create one.",
            config_path.display()
        );
    }
    let source = tokio::fs::read_to_string(&config_path).await?;

    let schema_store = SchemaStore::new();
    let options = FormatOptions::default();
    let formatter = Formatter::new(
        TomlVersion::V1_1_0,
        &options,
        Some(Either::Right(config_path.as_path())),
        &schema_store,
    );

    match formatter.format(&source).await {
        Ok(formatted) => {
            if formatted == source {
                println!("No changes made.");
            } else {
                tokio::fs::write(&config_path, &formatted).await?;
                println!(".gitdot-ci.toml formatted");
            }
            Ok(())
        }
        Err(diagnostics) => {
            let mut printer = Pretty::default();
            for d in &diagnostics {
                d.clone().with_source_file(&config_path).print(&mut printer);
            }
            anyhow::bail!("failed to format: {} error(s)", diagnostics.len())
        }
    }
}
