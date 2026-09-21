use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use data_model::{NewUser, User};
use log::{error, info};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};

#[derive(Debug)]
struct AppState {
    next_id: AtomicU64,
    users: Mutex<HashMap<u64, User>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let state = Arc::new(AppState {
        next_id: AtomicU64::new(1),
        users: Mutex::new(HashMap::new()),
    });

    let app = Router::new()
        .route("/health", get(health))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .with_state(state);

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    info!("api-server listening on http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}

async fn health(State(state): State<Arc<AppState>>) -> Json<Value> {
    let count = state.users.lock().unwrap().len();
    Json(json!({ "status": "ok", "user_count": count }))
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewUser>,
) -> (StatusCode, Json<Value>) {
    if let Err(e) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        );
    }

    let id = state.next_id.fetch_add(1, Ordering::Relaxed);
    let user = User::from_new(payload, id).unwrap();
    state.users.lock().unwrap().insert(user.id, user.clone());

    (StatusCode::CREATED, Json(json!(user)))
}

async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> (StatusCode, Json<Value>) {
    let users = state.users.lock().unwrap();
    match users.get(&id) {
        Some(u) => (StatusCode::OK, Json(json!(u))),
        None => {
            error!("user {id} not found");
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "not found" })),
            )
        }
    }
}
