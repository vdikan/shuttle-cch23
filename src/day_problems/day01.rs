use axum::{
    http::StatusCode,
    extract::Path,
    response::{IntoResponse, Response},
};

pub async fn day_01(Path(args): Path<String>) -> Response {
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
