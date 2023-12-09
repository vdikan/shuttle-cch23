use axum::{
    http::{StatusCode, HeaderValue, HeaderMap},
    response::{IntoResponse, Response},
};
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};

fn decode_cookie(cookie: &HeaderValue) -> Result<String> {
    let encoded_s = cookie.to_str()?.replace("recipe=", "");
    let decoded_s = general_purpose::STANDARD.decode(encoded_s)?;
    Ok(String::from_utf8(decoded_s)?)
}

pub async fn day07_decode(headers: HeaderMap) -> Response {
    if let Some(cookie) = headers.get("Cookie") {
        if let Ok(s) = decode_cookie(cookie) {
            s.into_response()
        } else {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    } else {
        StatusCode::BAD_REQUEST.into_response()
    }
}

// #[cfg(test)]
// mod tests {
//     use anyhow::Result;

//     #[tokio::test]
//     async fn test_test() -> Result<()> {
//         Ok(())
//     }
// }
