use axum::{routing::get, Router};

mod day_problems;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(day_problems::day_m1::hello_world))
        .route("/-1/error", get(day_problems::day_m1::internal_error))
        .route("/1/*args", get(day_problems::day01::day_01));

    Ok(router.into())
}
