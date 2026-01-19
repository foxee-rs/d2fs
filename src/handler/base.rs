use crate::{
    common::{error::biz_error::BizError, util::id::UuidGenerator},
    model::result::base::BizResult,
};
use axum::{
    body::Body,
    extract::{Multipart, Path as PathVar, Query},
    http::{HeaderMap, header},
    response::{
        IntoResponse, Sse,
        sse::{Event, KeepAlive},
    },
};
use std::{collections::HashMap, convert::Infallible, path::Path, time::Duration};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncWriteExt, BufWriter},
};
use tokio_stream::{Stream, StreamExt};
use tokio_util::io::ReaderStream;
use tracing::debug;

pub struct BaseHandler;
impl BaseHandler {
    pub async fn greet() -> BizResult<String> {
        debug!("f[greet] begin");
        let reply = format!("Hey from {}", "d2fs");
        debug!("f[greet] end");
        BizResult::ok(reply)
    }

    pub async fn path(PathVar(key): PathVar<String>) -> impl IntoResponse {
        key
    }

    pub async fn query(Query(map): Query<HashMap<String, String>>) -> impl IntoResponse {
        format!("{:?}", map)
    }

    pub async fn headers(header_map: HeaderMap) -> impl IntoResponse {
        BizResult::ok(format!("{:?}", header_map))
    }

    pub async fn post_text(body: String) -> impl IntoResponse {
        BizResult::ok(body)
    }

    pub async fn download_file() -> Result<impl IntoResponse, BizError> {
        let filename = "Cargo.toml";
        let f = File::open(Path::new(filename)).await.map_err(|_| BizError::FILE_IO)?;
        let len = f.metadata().await.map_err(|_| BizError::FILE_IO)?.len().to_string();
        // let body = tokio::fs::read(p).await.map_err(|_| BizError::FILE_OP_FAILED)?;
        let body = Body::from_stream(ReaderStream::new(f));
        let mut headers = HeaderMap::new();
        // headers.insert(header::CONTENT_TYPE, "application/octet-stream".parse().unwrap());
        headers.insert(
            header::CONTENT_DISPOSITION,
            format!("attachment;filename={}", filename).parse().unwrap(),
        );
        headers.insert(header::CONTENT_LENGTH, len.parse().unwrap());
        Ok((headers, body))
    }

    pub async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
        while let Some(mut field) = multipart.next_field().await.unwrap() {
            if let Some(src_file_name) = field.file_name() {
                let dst_file_name = format!("{}_{}", UuidGenerator::next_v4(), src_file_name);
                let dst_dir = std::env::current_dir().unwrap().join("tmp");
                tokio::fs::create_dir_all(&dst_dir).await.expect("cant't mkdir");
                let dst = dst_dir.join(dst_file_name);
                let dst = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(dst)
                    .await
                    .unwrap();
                let mut writer = BufWriter::with_capacity(2 * 1024 * 1024, dst);
                while let Some(data) = field.chunk().await.unwrap() {
                    let _ = writer.write_all(&data).await;
                }
                let _ = writer.flush().await;
            }
        }
        BizResult::ok(())
    }

    pub async fn open_sse() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
        let stream = tokio_stream::iter(0..usize::MAX)
            .map(|_| {
                Event::default()
                    .data(format!(
                        "ServerMsg#{}",
                        OffsetDateTime::now_utc().format(&Rfc3339).unwrap()
                    ))
                    .event("sse-event-name")
                    .id("sse-event-id")
                    .comment("sse-event-comment")
            })
            .map(Ok)
            .throttle(Duration::from_secs(3));
        Sse::new(stream).keep_alive(KeepAlive::default().text("sse-keep-alive-msg"))
    }
}
