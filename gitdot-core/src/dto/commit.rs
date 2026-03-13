mod create_commits;
mod get_commit;
mod get_commits;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{
    Commit, CommitDiff, CommitDiffChange, CommitDiffLine, CommitDiffPair, CommitDiffSyntaxHighlight,
};

use super::{DiffChange, DiffLine, DiffPair, SyntaxHighlight};

pub use create_commits::CreateCommitsRequest;
pub use get_commit::GetCommitRequest;
pub use get_commits::GetCommitsRequest;

#[derive(Debug, Clone)]
pub struct CommitResponse {
    pub id: Uuid,
    pub author_id: Option<Uuid>,
    pub git_author_name: String,
    pub git_author_email: String,
    pub repo_id: Uuid,
    pub ref_name: String,
    pub sha: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub diffs: Vec<CommitDiff>,
}

#[derive(Debug, Clone)]
pub struct CommitsResponse {
    pub commits: Vec<CommitResponse>,
    pub has_next: bool,
}

impl From<DiffPair> for CommitDiffPair {
    fn from(p: DiffPair) -> Self {
        Self {
            lhs: p.lhs.map(Into::into),
            rhs: p.rhs.map(Into::into),
        }
    }
}

impl From<DiffLine> for CommitDiffLine {
    fn from(l: DiffLine) -> Self {
        Self {
            line_number: l.line_number,
            changes: l.changes.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<DiffChange> for CommitDiffChange {
    fn from(c: DiffChange) -> Self {
        Self {
            start: c.start,
            end: c.end,
            content: c.content,
            highlight: c.highlight.into(),
        }
    }
}

impl From<SyntaxHighlight> for CommitDiffSyntaxHighlight {
    fn from(h: SyntaxHighlight) -> Self {
        match h {
            SyntaxHighlight::Delimiter => Self::Delimiter,
            SyntaxHighlight::Normal => Self::Normal,
            SyntaxHighlight::String => Self::String,
            SyntaxHighlight::Type => Self::Type,
            SyntaxHighlight::Comment => Self::Comment,
            SyntaxHighlight::Keyword => Self::Keyword,
            SyntaxHighlight::TreeSitterError => Self::TreeSitterError,
        }
    }
}

impl From<Commit> for CommitResponse {
    fn from(commit: Commit) -> Self {
        Self {
            id: commit.id,
            author_id: commit.author_id,
            git_author_name: commit.git_author_name,
            git_author_email: commit.git_author_email,
            repo_id: commit.repo_id,
            ref_name: commit.ref_name,
            sha: commit.sha,
            message: commit.message,
            created_at: commit.created_at,
            diffs: commit.diffs,
        }
    }
}
