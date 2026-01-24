use crate::{
    common::error::biz_error::BizError,
    model::{
        dto::fs::FsRoot,
        param::fs::{FsLsParam, FsMkdirParam, FsMvParam, FsRmParam, FsTouchParam},
        result::{base::BizResult, fs::FsLsResult},
    },
};
use axum::{Extension, Json, response::IntoResponse};
use walkdir::WalkDir;

pub struct FsHandler;
impl FsHandler {
    pub async fn ls0(Json(_param): Json<FsLsParam>) -> BizResult<FsLsResult> {
        let d = FsLsResult {
            path: vec![
                String::from("Document"),
                String::from("Download"),
                String::from("Music"),
                String::from("Video"),
            ],
        };
        BizResult::ok(d)
    }

    pub async fn ls(
        Extension(fs_root): Extension<FsRoot>,
        Json(param): Json<FsLsParam>,
    ) -> Result<BizResult<FsLsResult>, BizError> {
        let p = param.path.trim_start_matches('/').to_string();
        let fs_root = fs_root.path.clone();

        let entries: Result<Vec<String>, BizError> = tokio::task::spawn_blocking(move || {
            let dst = fs_root.join(p);
            let dst = std::fs::canonicalize(&dst)
                .map_err(|_| BizError::FILE_IO.concat_msg(format!("path not found: {}", param.path)))?;
            if !dst.starts_with(&fs_root) {
                return Err(BizError::FILE_IO.concat_msg("illegal path"));
            }
            if !dst.is_dir() {
                return Err(BizError::FILE_IO.concat_msg("isn't directory"));
            }
            let mut result = Vec::<String>::new();
            let walker = WalkDir::new(dst).min_depth(1).max_depth(1).sort_by_file_name();
            for entry in walker {
                match entry {
                    Ok(e) => {
                        let name = e.file_name().to_string_lossy().to_string();
                        result.push(name);
                    }
                    Err(_) => continue,
                }
            }
            Ok(result)
        })
        .await
        .map_err(|_e| BizError::FILE_IO)?;
        entries.map(|data| BizResult::ok(FsLsResult { path: data }))
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
