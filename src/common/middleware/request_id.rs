use crate::common::{constant::http::HEADER_REQUEST_ID, error::biz_error::BizError, util::id::UlidGenerator};
use axum::{http::HeaderValue, middleware::Next, response::Response};
use tracing::{Instrument, info_span, trace};

pub async fn check_request_id(mut req: axum::extract::Request, next: Next) -> Result<Response, BizError> {
    trace!("[f]check_request_id begin");
    // complete x-request-id
    let rayid = req
        .headers()
        .get(HEADER_REQUEST_ID)
        .and_then(|v| v.to_str().ok())
        .filter(|s| is_valid(s))
        .map(|s| s.to_owned())
        .unwrap_or_else(UlidGenerator::next);
    req.headers_mut().insert(
        HEADER_REQUEST_ID,
        HeaderValue::from_str(&rayid).expect("x-request-id must be a valid HTTP header value"),
    );
    // make request span
    let span = info_span!(
        "request",
        method = %req.method(),
        uri = %req.uri(),
        version = ?req.version(),
        rayid = %rayid
    );
    async move {
        // call next service
        let response = next.run(req).await;
        // do something with `response`...
        trace!("finished build request context");
        Ok(response)
    }
    .instrument(span)
    .await
}

fn is_valid(s: &str) -> bool {
    const THRESHOLD: usize = 1024;
    !s.is_empty() && s.len() <= THRESHOLD && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}
