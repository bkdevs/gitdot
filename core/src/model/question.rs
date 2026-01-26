use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::User;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub number: i32,
    pub author_id: Uuid,
    pub repository_id: Uuid,
    pub title: String,
    pub body: String,
    pub upvote: i32,
    pub impression: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    /// Current user's vote on this question (1 = upvote, -1 = downvote, None = no vote)
    pub user_vote: Option<i16>,

    #[sqlx(json(nullable))]
    pub author: Option<User>,

    #[sqlx(json(nullable))]
    pub comments: Option<Vec<Comment>>,

    #[sqlx(json(nullable))]
    pub answers: Option<Vec<Answer>>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Answer {
    pub id: Uuid,
    pub question_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
    pub upvote: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    /// Current user's vote on this answer
    pub user_vote: Option<i16>,

    #[sqlx(json(nullable))]
    pub author: Option<User>,

    #[sqlx(json(nullable))]
    pub comments: Option<Vec<Comment>>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
    pub upvote: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    /// Current user's vote on this comment
    pub user_vote: Option<i16>,

    #[sqlx(json(nullable))]
    pub author: Option<User>,
}

#[derive(Debug, Clone)]
pub struct VoteResult {
    pub target_id: Uuid,
    pub new_score: i32,
    pub user_vote: Option<i16>,
}
