mod create_answer;
mod create_answer_comment;
mod create_question;
mod create_question_comment;
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
use create_answer_comment::create_answer_comment;
use create_question::create_question;
use create_question_comment::create_question_comment;
use get_question::get_question;
use get_questions::get_questions;
use update_answer::update_answer;
use update_comment::update_comment;
use update_question::update_question;

pub fn create_question_router() -> Router<AppState> {
    Router::new()
        .route("/repository/{owner}/{repo}/question", post(create_question))
        .route(
            "/repository/{owner}/{repo}/question/{number}",
            get(get_question).patch(update_question),
        )
        .route("/repository/{owner}/{repo}/questions", get(get_questions))
        .route(
            "/repository/{owner}/{repo}/question/{number}/answer",
            post(create_answer),
        )
        .route(
            "/repository/{owner}/{repo}/question/{number}/answer/{answer_id}",
            patch(update_answer),
        )
        .route(
            "/repository/{owner}/{repo}/question/{number}/comment",
            post(create_question_comment),
        )
        .route(
            "/repository/{owner}/{repo}/question/{number}/answer/{answer_id}/comment",
            post(create_answer_comment),
        )
        .route(
            "/repository/{owner}/{repo}/question/{number}/comment/{comment_id}",
            patch(update_comment),
        )
}
