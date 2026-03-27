mod create_webhook;
mod delete_webhook;
mod get_webhook;
mod list_webhooks;
mod update_webhook;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use create_webhook::create_webhook;
use delete_webhook::delete_webhook;
use get_webhook::get_webhook;
use list_webhooks::list_webhooks;
use update_webhook::update_webhook;

pub fn create_webhook_router() -> Router<AppState> {
    Router::new()
        .route("/repository/{owner}/{repo}/webhook", post(create_webhook))
        .route("/repository/{owner}/{repo}/webhooks", get(list_webhooks))
        .route(
            "/repository/{owner}/{repo}/webhook/{webhook_id}",
            get(get_webhook)
                .patch(update_webhook)
                .delete(delete_webhook),
        )
}
