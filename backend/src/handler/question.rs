mod create_answer;
mod create_comment;
mod create_question;
mod get_question;
mod get_questions;
mod update_answer;
mod update_comment;
mod update_question;

use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::app::AppState;

use create_answer::create_answer;
use create_comment::create_comment;
use create_question::create_question;
use get_question::get_question;
use get_questions::get_questions;
use update_answer::update_answer;
use update_comment::update_comment;
use update_question::update_question;

pub fn create_question_router() -> Router<AppState> {
    Router::new()
        .route("/question", post(create_question))
        .route(
            "/question/{question_id}",
            get(get_question).patch(update_question),
        )
        .route("/questions", get(get_questions))
        .route("/answer", post(create_answer))
        .route("/answer/{answer_id}", patch(update_answer))
        .route("/comment", post(create_comment))
        .route("/comment/{comment_id}", patch(update_comment))
}
