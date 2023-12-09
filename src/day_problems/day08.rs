use axum::{
    http::StatusCode,
    extract::Path,
    response::{IntoResponse, Response},
};
use reqwest;
use serde_json::Value;
use anyhow::Result;

async fn get_poke_weight(index: i32) -> Result<f64> {
    let endpoint = format!("https://pokeapi.co/api/v2/pokemon/{}/", index);
    let resp = reqwest::get(endpoint).await?.text().await?;
    let v: Value = serde_json::from_str(resp.as_str())?;
    let w = v["weight"].as_f64().unwrap() * 0.1_f64;
    Ok(w)
}

async fn get_poke_momentum(index: i32) -> Result<f64> {
    let w = get_poke_weight(index).await?;
    let g: f64 = 9.825;
    let m = f64::sqrt(20_f64 * g) * w; // h=10m due to the task description
    Ok(m)
}

pub async fn day08_weight(Path(index): Path<i32>) -> Response {
    if let Ok(w) = get_poke_weight(index).await {
        format!("{}", w).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn day08_momentum(Path(index): Path<i32>) -> Response {
    if let Ok(w) = get_poke_momentum(index).await {
        format!("{}", w).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use super::get_poke_weight;
    use super::get_poke_momentum;

    #[tokio::test]
    async fn test_poke_weight() -> Result<()> {
        let w = get_poke_weight(25).await?;
        assert_eq!(w, 6_f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_poke_momentum() -> Result<()> {
        let m = get_poke_momentum(25).await?;
        assert_eq!(m, 84.10707461325713_f64);
        Ok(())
    }
}
