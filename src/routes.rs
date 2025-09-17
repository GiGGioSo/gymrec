use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    extract::State,
    Router,
};
use std::sync::{Arc, Mutex};

use crate::recording;

pub struct AppState {
    tester: i32,
    current_recording: Option<recording::RecordingInfo>
}

// -------------------- Route Handlers --------------------

pub async fn start_recording(
    State(state): State<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    println!("Received /start request. Starting recording...");

    let mut s = state.lock().expect("state mutex was poisoned");

    s.current_recording = Some(recording::start_recording().unwrap());
    s.tester = 42;
    (StatusCode::OK, "Recording started")
}

pub async fn stop_recording(
    State(state): State<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    println!("Received /stop request. Stopping recording...");

    let recording_info = {
        let mut s = state.lock().expect("state mutex was poisoned");
        s.tester = 69;
        s.current_recording.take()
    };

    if let Some(rec_info) = recording_info {
        let _ = recording::stop_recording(rec_info).await;
    }
    (StatusCode::OK, "Recording stopped")
}

pub async fn list_videos(
    State(state): State<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    println!("Received /videos request. Listing videos...");

    let s = state.lock().expect("state mutex was poisoned");
    println!("tester: {}", s.tester);

    (StatusCode::OK, "Video list (placeholder)")
}

// -------------------- Router --------------------

pub fn create_router() -> Router {
    let shared_state = Arc::new(Mutex::new(AppState {
        tester: 10,
        current_recording: None
    }));
    Router::new()
        .route("/api/start", post(start_recording))
        .route("/api/stop", post(stop_recording))
        .route("/api/videos", get(list_videos))
        .with_state(shared_state)
}
