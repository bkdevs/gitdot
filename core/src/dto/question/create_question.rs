use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::QuestionError,
};

#[derive(Debug, Clone)]
pub struct CreateQuestionRequest {
    pub author_id: Uuid,
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub title: String,
    pub body: String,
}

impl CreateQuestionRequest {
    pub fn new(
        author_id: Uuid,
        owner: &str,
        repo: &str,
        title: String,
        body: String,
    ) -> Result<Self, QuestionError> {
        Ok(Self {
            author_id,
            owner: OwnerName::try_new(owner)
                .map_err(|e| QuestionError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| QuestionError::InvalidOwnerName(e.to_string()))?,
            title,
            body,
        })
    }

    pub fn get_repo_path(&self) -> String {
        format!("{}/{}", self.owner.as_ref(), self.repo.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_request() {
        let author_id = Uuid::new_v4();
        let request = CreateQuestionRequest::new(
            author_id,
            "johndoe",
            "my-repo",
            "Question title".to_string(),
            "Question body".to_string(),
        )
        .unwrap();

        assert_eq!(request.author_id, author_id);
        assert_eq!(request.owner.as_ref(), "johndoe");
        assert_eq!(request.repo.as_ref(), "my-repo");
        assert_eq!(request.title, "Question title");
        assert_eq!(request.body, "Question body");
    }

    #[test]
    fn get_repo_path_formats_correctly() {
        let author_id = Uuid::new_v4();
        let request = CreateQuestionRequest::new(
            author_id,
            "johndoe",
            "my-repo",
            "Title".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        assert_eq!(request.get_repo_path(), "johndoe/my-repo");
    }

    #[test]
    fn sanitizes_owner_and_repo_to_lowercase() {
        let author_id = Uuid::new_v4();
        let request = CreateQuestionRequest::new(
            author_id,
            "JohnDoe",
            "MyRepo",
            "Title".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        assert_eq!(request.get_repo_path(), "johndoe/myrepo");
    }

    #[test]
    fn rejects_invalid_owner() {
        let author_id = Uuid::new_v4();
        let result = CreateQuestionRequest::new(
            author_id,
            "invalid@owner",
            "my-repo",
            "Title".to_string(),
            "Body".to_string(),
        );

        assert!(matches!(result, Err(QuestionError::InvalidOwnerName(_))));
    }

    #[test]
    fn rejects_invalid_repo() {
        let author_id = Uuid::new_v4();
        let result = CreateQuestionRequest::new(
            author_id,
            "johndoe",
            "invalid/repo",
            "Title".to_string(),
            "Body".to_string(),
        );

        assert!(matches!(result, Err(QuestionError::InvalidOwnerName(_))));
    }
}
