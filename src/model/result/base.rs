use crate::common::error::biz_error::BizError;
use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BizResult<T: Serialize> {
    pub code: Cow<'static, str>,
    pub msg: Cow<'static, str>,
    pub data: Option<T>,
}

impl<T: Serialize> BizResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: BizError::OK.code,
            msg: BizError::OK.msg,
            data: Some(data),
        }
    }
}

impl From<BizError> for BizResult<()> {
    fn from(value: BizError) -> Self {
        Self {
            code: value.code,
            msg: value.msg,
            data: None,
        }
    }
}

impl<T> IntoResponse for BizResult<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
