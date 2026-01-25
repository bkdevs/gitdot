use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::User;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub author_id: Uuid,
    pub repository_id: Uuid,
    pub title: String,
    pub body: String,
    pub upvote: i32,
    pub impression: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

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

    #[sqlx(json(nullable))]
    pub author: Option<User>,
}
