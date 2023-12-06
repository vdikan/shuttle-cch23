use axum::{routing::get, routing::post, Router};

mod day_problems;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(day_problems::day_m1::hello_world))
        .route("/-1/error", get(day_problems::day_m1::internal_error))
        .route("/1/*args", get(day_problems::day01::day_01))
        .route("/4/strength", post(day_problems::day04::day_04_strength))
        .route("/4/contest", post(day_problems::day04::day_04_contest))
        .route("/6", post(day_problems::day06::day06));

    Ok(router.into())
}
