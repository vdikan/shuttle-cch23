use axum::{routing::get, routing::post, Router};
use tower_http::services::ServeDir;

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
        .nest_service("/11/assets", ServeDir::new("assets"));

    Ok(router.into())
}
