use axum::http::StatusCode;

pub async fn hello_world() -> &'static str {
    "Hello, cruel world\n"
}

pub async fn internal_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}
