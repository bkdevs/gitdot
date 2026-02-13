use crate::dto::{
    DiffChange, DiffHunk, DiffLine, DiffPair, RepositoryDiffResponse, RepositoryFileResponse,
    SyntaxHighlight,
};
use crate::error::DiffError;

pub trait DiffClient: Send + Sync + Clone + 'static {
    fn diff_files(
        &self,
        left: Option<&RepositoryFileResponse>,
        right: Option<&RepositoryFileResponse>,
    ) -> Result<RepositoryDiffResponse, DiffError>;
}

#[derive(Clone)]
pub struct DifftClient;

impl DifftClient {
    pub fn new() -> Self {
        Self {}
    }

    fn execute_difftastic(
        left_content: &str,
        right_content: &str,
        file_path: Option<&str>,
    ) -> Option<DifftasticOutput> {
        use std::io::Write;
        use tempfile::Builder;

        let extension = file_path
            .and_then(|p| std::path::Path::new(p).extension())
            .and_then(|ext| ext.to_str())
            .unwrap_or("txt");

        let mut left_file = Builder::new()
            .suffix(&format!(".{}", extension))
            .tempfile()
            .ok()?;
        let mut right_file = Builder::new()
            .suffix(&format!(".{}", extension))
            .tempfile()
            .ok()?;

        left_file.write_all(left_content.as_bytes()).ok()?;
        right_file.write_all(right_content.as_bytes()).ok()?;

        let output = std::process::Command::new("difft")
            .arg(left_file.path())
            .arg(right_file.path())
            .arg("--display")
            .arg("json")
            .env("DFT_UNSTABLE", "yes")
            .output()
            .ok()?;

        serde_json::from_slice(&output.stdout).ok()
    }
}

impl DiffClient for DifftClient {
    fn diff_files(
        &self,
        left: Option<&RepositoryFileResponse>,
        right: Option<&RepositoryFileResponse>,
    ) -> Result<RepositoryDiffResponse, DiffError> {
        let left_content = left.map(|f| f.content.as_str()).unwrap_or("");
        let right_content = right.map(|f| f.content.as_str()).unwrap_or("");
        let file_path = right
            .map(|f| f.path.as_str())
            .or_else(|| left.map(|f| f.path.as_str()));

        let output = Self::execute_difftastic(left_content, right_content, file_path)
            .ok_or_else(|| DiffError::DifftasticFailed("difftastic failed".to_string()))?;

        let hunks: Vec<DiffHunk> = output
            .chunks
            .unwrap_or_default()
            .into_iter()
            .map(|hunk| {
                hunk.into_iter()
                    .map(|line| DiffPair {
                        lhs: line.lhs.map(Into::into),
                        rhs: line.rhs.map(Into::into),
                    })
                    .collect()
            })
            .collect();

        let mut lines_added: u32 = 0;
        let mut lines_removed: u32 = 0;
        for hunk in &hunks {
            for pair in hunk {
                match (&pair.lhs, &pair.rhs) {
                    (None, Some(_)) => lines_added += 1,
                    (Some(_), None) => lines_removed += 1,
                    _ => {}
                }
            }
        }

        Ok(RepositoryDiffResponse {
            lines_added,
            lines_removed,
            hunks,
        })
    }
}

#[derive(serde::Deserialize)]
struct DifftasticOutput {
    chunks: Option<Vec<Vec<DifftLine>>>,
}

#[derive(serde::Deserialize)]
struct DifftLine {
    lhs: Option<DifftSide>,
    rhs: Option<DifftSide>,
}

#[derive(serde::Deserialize)]
struct DifftSide {
    line_number: u32,
    changes: Vec<DifftChange>,
}

impl From<DifftSide> for DiffLine {
    fn from(side: DifftSide) -> Self {
        Self {
            line_number: side.line_number,
            changes: side.changes.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(serde::Deserialize)]
struct DifftChange {
    start: u32,
    end: u32,
    content: String,
    highlight: DifftSyntaxHighlight,
}

impl From<DifftChange> for DiffChange {
    fn from(c: DifftChange) -> Self {
        Self {
            start: c.start,
            end: c.end,
            content: c.content,
            highlight: c.highlight.into(),
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum DifftSyntaxHighlight {
    Delimiter,
    Normal,
    String,
    Type,
    Comment,
    Keyword,
    TreeSitterError,
}

impl From<DifftSyntaxHighlight> for SyntaxHighlight {
    fn from(h: DifftSyntaxHighlight) -> Self {
        match h {
            DifftSyntaxHighlight::Delimiter => Self::Delimiter,
            DifftSyntaxHighlight::Normal => Self::Normal,
            DifftSyntaxHighlight::String => Self::String,
            DifftSyntaxHighlight::Type => Self::Type,
            DifftSyntaxHighlight::Comment => Self::Comment,
            DifftSyntaxHighlight::Keyword => Self::Keyword,
            DifftSyntaxHighlight::TreeSitterError => Self::TreeSitterError,
        }
    }
}
