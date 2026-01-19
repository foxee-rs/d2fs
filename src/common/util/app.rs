use axum::{Router, http::StatusCode, response::IntoResponse};
use std::net::SocketAddr;
use tokio::{net::TcpListener, signal};

pub async fn serve(router: Router, addr: SocketAddr) {
    let _ = axum::serve(TcpListener::bind(addr).await.unwrap(), router)
        .with_graceful_shutdown(shutdown())
        .await;
}

pub async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

/// 404 handler
pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "How ! 404.")
}
