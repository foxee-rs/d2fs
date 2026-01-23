use crate::handler::FsHandler;
use axum::{Router, routing::post};

pub struct FsRouter;
impl FsRouter {
    pub fn routes() -> Router {
        Router::new().nest(
            "/fs",
            Router::new()
                .route("/ls", post(FsHandler::ls))
                .route("/mkdir", post(FsHandler::mkdir))
                .route("/touch", post(FsHandler::touch))
                .route("/rm", post(FsHandler::rm))
                .route("/mv", post(FsHandler::mv))
                .route("/read", post(FsHandler::read))
                .route("/write", post(FsHandler::write)),
        )
    }
}
