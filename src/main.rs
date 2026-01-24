#![allow(dead_code)]
#![allow(async_fn_in_trait)]

use crate::{
    common::{
        cfg::http::singleton_http_client,
        middleware::request_id::check_request_id,
        util::{app, fs::init_fsroot},
    },
    job::scheduler::launch_job,
    model::dto::fs::FsRoot,
    router::{BaseRouter, FsRouter},
};
use axum::{Extension, Router, extract::DefaultBodyLimit, middleware};
use reqwest::StatusCode;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowCredentials, AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    request_id::PropagateRequestIdLayer,
    services::ServeDir,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::{Level, Span, info};

mod common;
mod handler;
mod job;
mod model;
mod router;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_thread_names(true)
        .with_timer(tracing_subscriber::fmt::time::OffsetTime::local_rfc_3339().expect("can't get local offset"))
        .init();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn(|| {
            static TOKIO_WORKER_ID: AtomicU64 = AtomicU64::new(0);
            let id = TOKIO_WORKER_ID.fetch_add(1, Ordering::Relaxed);
            format!("tokio-worker-{id}")
        })
        .build()
        .expect("can't create tokio runtime");
    rt.block_on(init());
}
pub async fn init() {
    let fsroot = init_fsroot().await.expect("can't init fsroot");
    // cors
    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .allow_methods(AllowMethods::mirror_request())
        .allow_headers(AllowHeaders::mirror_request())
        .allow_credentials(AllowCredentials::yes())
        .max_age(Duration::from_secs(3600));
    // router
    let app_router = Router::new()
        .merge(BaseRouter::routes())
        .merge(FsRouter::routes())
        .fallback(app::handler_404)
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(check_request_id))
                .layer(TraceLayer::new_for_http().make_span_with(|_: &axum::http::Request<_>| Span::none()))
                .layer(cors_layer.clone())
                .layer(PropagateRequestIdLayer::x_request_id())
                .layer(TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(30),
                ))
                .layer(DefaultBodyLimit::max(500 * 1024 * 1024))
                .layer(CompressionLayer::new())
                .layer(Extension(FsRoot { path: fsroot.clone() }))
                .layer(Extension(singleton_http_client())),
        );
    let res_router = Router::new()
        .nest_service("/fs", ServeDir::new(fsroot))
        .fallback(app::handler_404)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors_layer),
        );
    let app_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 8080);
    let res_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 8081);
    info!("app listening on {}", app_addr);
    info!("res listening on {}", res_addr);
    tokio::spawn(launch_job());
    tokio::join!(app::serve(app_router, app_addr), app::serve(res_router, res_addr),);
}
