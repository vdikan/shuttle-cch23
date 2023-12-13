use axum::{routing::get, routing::post, Router};
use tower_http::services::ServeDir;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::Instant;

mod day_problems;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(day_problems::day_m1::hello_world))
        .route("/-1/error", get(day_problems::day_m1::internal_error))
        .route("/1/*args", get(day_problems::day01::day_01))
        .route("/4/strength", post(day_problems::day04::day_04_strength))
        .route("/4/contest", post(day_problems::day04::day_04_contest))
        .route("/6", post(day_problems::day06::day06))
        .route("/7/decode", get(day_problems::day07::day07_decode))
        .route("/8/weight/:index", get(day_problems::day08::day08_weight))
        .route("/8/drop/:index", get(day_problems::day08::day08_momentum))
        .route("/11/red_pixels", post(day_problems::day11::day11_reds))
        .route("/12/save/:string", post(day_problems::day12::save))
        .route("/12/load/:string", get(day_problems::day12::load))
        .nest_service("/11/assets", ServeDir::new("assets"))
        .with_state(Arc::new(Mutex::new(HashMap::<String, Instant>::new())));

    Ok(router.into())
}
