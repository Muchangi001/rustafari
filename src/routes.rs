// Description: This file contains the routes for the Axum web server.
// It defines the API endpoints for user management and connection handling.
// It uses Axum for routing and Serde for JSON serialization/deserialization.
// It also includes error handling for various operations.
// It is designed to be modular and reusable, with a focus on clean code and separation of concerns.
use axum::{extract::{Path, State},response::IntoResponse, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::graph::{CommunityGraph, ConnectionType, User};
use crate::errors::{AppError};

type AppState = Arc<Mutex<CommunityGraph>>;

pub fn routes() -> Router {
    let graph = Arc::new(Mutex::new(CommunityGraph::new()));
    Router::new()
        .route("/users", post(add_user))
        .route("/users/:username", get(get_user))
        .route("/connections", post(connect_users))
        .route("/users/:username/recommendations", get(get_recommendations))
        .route("/interests/:interest/users", get(find_users_by_interest))
        .with_state(graph)
}

#[derive(Deserialize)]
struct NewUser {
    username: String,
    bio: Option<String>,
    interests: Vec<String>,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: Some(data),
        }
    }

    fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: None,
        }
    }
}

async fn add_user(
    State(state): State<AppState>, 
    Json(payload): Json<NewUser>
) -> impl axum::response::IntoResponse {
    let user = User::new(
        payload.username.clone(),
        payload.bio,
        payload.interests,
    );
    
    let result = state.lock()
        .map_err(|e| AppError::InternalError(e.to_string()))
        .and_then(|mut graph| graph.add_user(user));
    
    match result {
        Ok(_) => Json(ApiResponse::success(
            format!("User {} added successfully", payload.username),
            payload.username
        )).into_response(),
        Err(err) => err.into_response(),
    }
}

#[derive(Deserialize)]
struct ConnectPayload {
    from: String,
    to: String,
    kind: ConnectionType,
    tags: Vec<String>,
    since: String,
}

async fn connect_users(
    State(state): State<AppState>, 
    Json(payload): Json<ConnectPayload>
) -> impl axum::response::IntoResponse {
    let result = state.lock()
        .map_err(|e| AppError::InternalError(e.to_string()))
        .and_then(|mut graph| {
            graph.connect_users(
                &payload.from, 
                &payload.to, 
                payload.kind, 
                payload.tags, 
                payload.since
            )
        });
    
    match result {
        Ok(_) => Json(ApiResponse::success(
            format!("Connected {} to {}", payload.from, payload.to),
            format!("{}:{}", payload.from, payload.to)
        )).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn get_user(
    State(state): State<AppState>, 
    Path(username): Path<String>
) -> impl axum::response::IntoResponse {
    let result = state.lock()
        .map_err(|e| AppError::InternalError(e.to_string()))
        .and_then(|graph| graph.get_user(&username).map(|user| user.clone()));
    
    match result {
        Ok(user) => Json(ApiResponse::success(
            format!("User {} found", username),
            user
        )).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn get_recommendations(
    State(state): State<AppState>,
    Path(username): Path<String>
) -> impl axum::response::IntoResponse {
    let result = state.lock()
        .map_err(|e| AppError::InternalError(e.to_string()))
        .and_then(|graph| graph.recommend_connections(&username));
    
    match result {
        Ok(recommendations) => Json(ApiResponse::success(
            format!("Found {} recommendations for {}", recommendations.len(), username),
            recommendations
        )).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn find_users_by_interest(
    State(state): State<AppState>,
    Path(interest): Path<String>
) -> impl axum::response::IntoResponse {
    let result = state.lock()
        .map_err(|e| AppError::InternalError(e.to_string()))
        .map(|graph| {
            let users = graph.find_users_by_interest(&interest);
            users.into_iter().cloned().collect::<Vec<_>>()
        });
    
    match result {
        Ok(users) => Json(ApiResponse::success(
            format!("Found {} users interested in {}", users.len(), interest),
            users
        )).into_response(),
        Err(err) => err.into_response(),
    }
}