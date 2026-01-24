use crate::{
    common::error::biz_error::BizError,
    model::{
        dto::fs::FsRoot,
        param::fs::{FsLsParam, FsMkdirParam, FsMvParam, FsRmParam, FsTouchParam},
        result::{
            base::BizResult,
            fs::{FsItem, FsKind, FsLsResult},
        },
    },
};
use axum::{Extension, Json, response::IntoResponse};
use tokio::fs;

pub struct FsHandler;
impl FsHandler {
    pub async fn ls(
        Extension(fsroot): Extension<FsRoot>,
        Json(param): Json<FsLsParam>,
    ) -> Result<BizResult<FsLsResult>, BizError> {
        let p = param.path.trim_start_matches('/');
        let fsroot = fsroot.path.clone();
        let dst = fsroot.join(p);
        let dst = fs::canonicalize(&dst)
            .await
            .map_err(|_| BizError::FILE_IO.concat_msg(format!("path not found: {}", param.path)))?;
        if !dst.starts_with(&fsroot) {
            return Err(BizError::FILE_IO.concat_msg("illegal path"));
        }
        if !fs::metadata(&dst).await.map_err(|_| BizError::FILE_IO)?.is_dir() {
            return Err(BizError::FILE_IO.concat_msg("isn't directory"));
        }
        let mut read_dir = fs::read_dir(&dst)
            .await
            .map_err(|e| BizError::FILE_IO.concat_msg(e.to_string()))?;
        let mut item_vec = Vec::new();
        while let Some(entry) = read_dir
            .next_entry()
            .await
            .map_err(|e| BizError::FILE_IO.concat_msg(e.to_string()))?
        {
            if let Ok(metadata) = entry.metadata().await {
                let name = entry.file_name().to_string_lossy().to_string();
                let kind = if metadata.is_dir() { FsKind::Dir } else { FsKind::File };
                item_vec.push(FsItem {
                    kind,
                    name,
                    size: metadata.len(),
                });
            }
        }
        item_vec.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(BizResult::ok(FsLsResult { item_vec }))
    }

    pub async fn mkdir(Json(_param): Json<FsMkdirParam>) -> Result<impl IntoResponse, BizError> {
        Ok(BizResult::ok(()))
    }

    pub async fn touch(Json(_param): Json<FsTouchParam>) -> Result<impl IntoResponse, BizError> {
        Ok(BizResult::ok(()))
    }

    pub async fn mv(Json(_param): Json<FsMvParam>) -> Result<impl IntoResponse, BizError> {
        Ok(BizResult::ok(()))
    }

    pub async fn rm(Json(_param): Json<FsRmParam>) -> Result<impl IntoResponse, BizError> {
        Ok(BizResult::ok(()))
    }

    pub async fn read() -> BizResult<String> {
        BizResult::ok(String::from("read"))
    }

    pub async fn write() -> BizResult<String> {
        BizResult::ok(String::from("write"))
    }
}
