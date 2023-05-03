use axum::{extract::TypedHeader, headers::UserAgent};
use once_cell::sync::Lazy;
use reqwest::header::HeaderValue;

static DEFAULT_USER_AGENT: Lazy<HeaderValue> =
    Lazy::new(|| HeaderValue::from_static(concat!("Readable/", env!("CARGO_PKG_VERSION"))));

pub fn forwarded_agent(ua_header: &Option<TypedHeader<UserAgent>>) -> HeaderValue {
    match ua_header {
        Some(TypedHeader(ua)) => {
            HeaderValue::from_str(ua.as_str()).unwrap_or_else(|_| DEFAULT_USER_AGENT.clone())
        }
        None => DEFAULT_USER_AGENT.clone(),
    }
}

/// get current date and time as UTC
/// and format as: 1 December, 2017 12:00:00
pub fn get_time() -> String {
    let now = chrono::Local::now();
    now.format("%A, %B %e, %Y, %H:%M:%S").to_string()
}
