use crate::handler::FsHandler;
use axum::{Router, routing::post};

pub struct FsRouter;
impl FsRouter {
    pub fn routes() -> Router {
        Router::new()
            .route("/ls", post(FsHandler::ls))
            .route("/mkdir", post(FsHandler::mkdir))
            .route("/touch", post(FsHandler::touch))
            .route("/read", post(FsHandler::read))
            .route("/write", post(FsHandler::write))
            .route("/rm", post(FsHandler::ls))
            .route("/mv", post(FsHandler::mv))
    }
}
