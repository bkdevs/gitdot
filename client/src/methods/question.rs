use anyhow::Result;
use uuid::Uuid;

use gitdot_api::endpoint::question::{
    create_answer::{CreateAnswerRequest, CreateAnswerResponse},
    create_answer_comment::{CreateAnswerCommentRequest, CreateAnswerCommentResponse},
    create_question::{CreateQuestionRequest, CreateQuestionResponse},
    create_question_comment::{CreateQuestionCommentRequest, CreateQuestionCommentResponse},
    get_question::{GetQuestionRequest, GetQuestionResponse},
    get_questions::{GetQuestionsRequest, GetQuestionsResponse},
    update_answer::{UpdateAnswerRequest, UpdateAnswerResponse},
    update_comment::{UpdateCommentRequest, UpdateCommentResponse},
    update_question::{UpdateQuestionRequest, UpdateQuestionResponse},
    vote_answer::{VoteAnswerRequest, VoteAnswerResponse},
    vote_comment::{VoteCommentRequest, VoteCommentResponse},
    vote_question::{VoteQuestionRequest, VoteQuestionResponse},
};

use crate::client::GitdotClient;

impl GitdotClient {
    pub async fn create_question(
        &self,
        owner: &str,
        repo: &str,
        request: CreateQuestionRequest,
    ) -> Result<CreateQuestionResponse> {
        self.post(format!("repository/{}/{}/question", owner, repo), request)
            .await
    }

    pub async fn get_question(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
        request: GetQuestionRequest,
    ) -> Result<GetQuestionResponse> {
        self.get(
            format!("repository/{}/{}/question/{}", owner, repo, number),
            request,
        )
        .await
    }

    pub async fn get_questions(
        &self,
        owner: &str,
        repo: &str,
        request: GetQuestionsRequest,
    ) -> Result<GetQuestionsResponse> {
        self.get(format!("repository/{}/{}/questions", owner, repo), request)
            .await
    }

    pub async fn update_question(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
        request: UpdateQuestionRequest,
    ) -> Result<UpdateQuestionResponse> {
        self.patch(
            format!("repository/{}/{}/question/{}", owner, repo, number),
            request,
        )
        .await
    }

    pub async fn create_answer(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
        request: CreateAnswerRequest,
    ) -> Result<CreateAnswerResponse> {
        self.post(
            format!("repository/{}/{}/question/{}/answer", owner, repo, number),
            request,
        )
        .await
    }

    pub async fn update_answer(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
        answer_id: Uuid,
        request: UpdateAnswerRequest,
    ) -> Result<UpdateAnswerResponse> {
        self.patch(
            format!(
                "repository/{}/{}/question/{}/answer/{}",
                owner, repo, number, answer_id
            ),
            request,
        )
        .await
    }

    pub async fn create_answer_comment(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
        answer_id: Uuid,
        request: CreateAnswerCommentRequest,
    ) -> Result<CreateAnswerCommentResponse> {
        self.post(
            format!(
                "repository/{}/{}/question/{}/answer/{}/comment",
                owner, repo, number, answer_id
            ),
            request,
        )
        .await
    }

    pub async fn create_question_comment(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
        request: CreateQuestionCommentRequest,
    ) -> Result<CreateQuestionCommentResponse> {
        self.post(
            format!("repository/{}/{}/question/{}/comment", owner, repo, number),
            request,
        )
        .await
    }

    pub async fn update_comment(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
        comment_id: Uuid,
        request: UpdateCommentRequest,
    ) -> Result<UpdateCommentResponse> {
        self.patch(
            format!(
                "repository/{}/{}/question/{}/comment/{}",
                owner, repo, number, comment_id
            ),
            request,
        )
        .await
    }

    pub async fn vote_question(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
        request: VoteQuestionRequest,
    ) -> Result<VoteQuestionResponse> {
        self.post(
            format!("repository/{}/{}/question/{}/vote", owner, repo, number),
            request,
        )
        .await
    }

    pub async fn vote_answer(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
        answer_id: Uuid,
        request: VoteAnswerRequest,
    ) -> Result<VoteAnswerResponse> {
        self.post(
            format!(
                "repository/{}/{}/question/{}/answer/{}/vote",
                owner, repo, number, answer_id
            ),
            request,
        )
        .await
    }

    pub async fn vote_comment(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
        comment_id: Uuid,
        request: VoteCommentRequest,
    ) -> Result<VoteCommentResponse> {
        self.post(
            format!(
                "repository/{}/{}/question/{}/comment/{}/vote",
                owner, repo, number, comment_id
            ),
            request,
        )
        .await
    }
}
