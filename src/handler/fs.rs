use crate::{
    common::error::biz_error::BizError,
    model::{
        param::fs::{FsLsParam, FsMkdirParam, FsMvParam, FsRmParam, FsTouchParam},
        result::{base::BizResult, fs::FsLsResult},
    },
};
use axum::{Json, response::IntoResponse};

pub struct FsHandler;
impl FsHandler {
    pub async fn ls(Json(_param): Json<FsLsParam>) -> BizResult<FsLsResult> {
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
