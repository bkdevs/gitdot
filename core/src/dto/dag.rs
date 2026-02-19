mod create_dag;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::dag::Dag;

pub use create_dag::CreateDagRequest;

#[derive(Debug, Clone)]
pub struct DagResponse {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub task_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Dag> for DagResponse {
    fn from(dag: Dag) -> Self {
        Self {
            id: dag.id,
            repo_owner: dag.repo_owner,
            repo_name: dag.repo_name,
            task_ids: dag.task_ids,
            created_at: dag.created_at,
            updated_at: dag.updated_at,
        }
    }
}
