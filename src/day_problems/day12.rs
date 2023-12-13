use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use tokio::time::Instant;

pub type Day12State = Arc<Mutex<HashMap<String, Instant>>>;

pub async fn save(Path(elem): Path<String>, State(state): State<Day12State>) -> StatusCode {
    state.lock().unwrap().insert(elem, Instant::now());
    StatusCode::OK
}

pub async fn load(Path(elem): Path<String>, State(state): State<Day12State>) -> Json<u64> {
    state
        .lock()
        .unwrap()
        .get(&elem)
        .map_or(Json(0), |x| Json(x.elapsed().as_secs()))
}
