use axum::{
    routing::get,
    Router,
    http::StatusCode,
    extract::Path,
    response::{IntoResponse, Response},
};

async fn hello_world() -> &'static str {
    "Hello, cruel world\n"
}

async fn internal_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn day_01(Path(args): Path<String>) -> Response {
    let parsed: Result<Vec<_>, _> = args.split('/').map(|s| s.parse::<i32>()).collect();
    match parsed {
        Ok(v) => {
            if v.len() <= 20 {
                format!("{}", v.iter().copied().reduce(|a, b| a ^ b).unwrap().pow(3))
                    .into_response()
            } else {
                StatusCode::NOT_FOUND.into_response()
            }
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(internal_error))
        .route("/1/*args", get(day_01));

    Ok(router.into())
}
