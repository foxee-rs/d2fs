use crate::handler::BaseHandler;
use axum::{
    Router,
    routing::{get, post},
};

pub struct BaseRouter;
impl BaseRouter {
    pub fn routes() -> Router {
        Router::new()
            .route("/", get(BaseHandler::greet))
            .route("/path/{key}", get(BaseHandler::path))
            .route("/query", get(BaseHandler::query))
            .route("/headers", get(BaseHandler::headers))
            .route("/post-text", post(BaseHandler::post_text))
            .route("/download-file", get(BaseHandler::download_file))
            .route("/upload-file", post(BaseHandler::upload_file))
            .route("/open-sse", get(BaseHandler::open_sse))
    }
}
