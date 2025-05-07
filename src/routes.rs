// Description: This file contains the routes for the Axum web server.
// It defines the API endpoints for user management and connection handling.
// It uses Axum for routing and Serde for JSON serialization/deserialization.
// It also includes error handling for various operations.
// It is designed to be modular and reusable, with a focus on clean code and separation of concerns.
use axum::{extract::{Path, State},response::{Html, IntoResponse}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::graph::{CommunityGraph, ConnectionType, User};
use crate::errors::AppError;

type AppState = Arc<Mutex<CommunityGraph>>;

pub fn routes() -> Router {
    let graph = Arc::new(Mutex::new(CommunityGraph::new()));
    Router::new()
        .route("/", get(root))
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

async fn root() -> Html<String> {
    Html(format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>🦀 Rustafari</title>
    <style>
        :root {{
            --rust-orange: #e6531c;
            --rust-orange-light: #f74c00;
            --rust-dark: #262625;
            --rust-darker: #1a1a19;
            --rust-text: #e8e8e8;
        }}
        
        body {{
            font-family: 'Fira Code', monospace, system-ui, -apple-system, sans-serif;
            background-color: var(--rust-dark);
            color: var(--rust-text);
            margin: 0;
            padding: 0;
            line-height: 1.6;
        }}
        
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
        }}
        
        header {{
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 1rem 0;
            border-bottom: 2px solid var(--rust-orange);
            margin-bottom: 2rem;
        }}
        
        h1 {{
            color: var(--rust-orange);
            font-size: 2.5rem;
            margin: 0;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }}
        
        .logo {{
            font-size: 3rem;
        }}
        
        .tagline {{
            font-style: italic;
            color: #aaa;
            margin-bottom: 2rem;
        }}
        
        .features {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 1.5rem;
            margin: 2rem 0;
        }}
        
        .feature-card {{
            background-color: var(--rust-darker);
            border-left: 4px solid var(--rust-orange);
            padding: 1.5rem;
            border-radius: 0 4px 4px 0;
        }}
        
        .feature-card h3 {{
            color: var(--rust-orange-light);
            margin-top: 0;
        }}
        
        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 2rem 0;
        }}
        
        th, td {{
            padding: 0.75rem;
            text-align: left;
            border-bottom: 1px solid #444;
        }}
        
        th {{
            background-color: var(--rust-darker);
            color: var(--rust-orange);
        }}
        
        tr:hover {{
            background-color: #333;
        }}
        
        pre {{
            background-color: var(--rust-darker);
            padding: 1rem;
            border-radius: 4px;
            overflow-x: auto;
            border-left: 4px solid var(--rust-orange);
        }}
        
        code {{
            color: var(--rust-orange-light);
            font-family: 'Fira Code', monospace;
        }}
        
        .cta {{
            background-color: var(--rust-orange);
            color: white;
            padding: 0.75rem 1.5rem;
            border: none;
            border-radius: 4px;
            font-weight: bold;
            cursor: pointer;
            font-size: 1rem;
            margin-top: 1rem;
            display: inline-block;
            text-decoration: none;
        }}
        
        .cta:hover {{
            background-color: var(--rust-orange-light);
        }}
        
        footer {{
            margin-top: 3rem;
            padding-top: 1rem;
            border-top: 1px solid #444;
            text-align: center;
            color: #777;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1><span class="logo">🦀</span> Rustafari</h1>
        </header>
        
        <p class="tagline">A Vanilla Rust Community Platform</p>
        
        <p>Rustafari connects Rust developers based on their interests, mentorship needs, and collaboration opportunities.</p>
        
        <div class="features">
            <div class="feature-card">
                <h3>User Profiles</h3>
                <p>Create profiles with interests and bio information</p>
            </div>
            <div class="feature-card">
                <h3>Smart Connections</h3>
                <p>Connect as mentors, collaborators, followers, or project buddies</p>
            </div>
            <div class="feature-card">
                <h3>Interest Matching</h3>
                <p>Find other Rust developers with similar interests</p>
            </div>
        </div>
        
        <h2>API Endpoints</h2>
        
        <table>
            <thead>
                <tr>
                    <th>Endpoint</th>
                    <th>Method</th>
                    <th>Description</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td><code>/users</code></td>
                    <td>POST</td>
                    <td>Add a new user</td>
                </tr>
                <tr>
                    <td><code>/users/:username</code></td>
                    <td>GET</td>
                    <td>Get a specific user's profile</td>
                </tr>
                <tr>
                    <td><code>/connections</code></td>
                    <td>POST</td>
                    <td>Create a connection between users</td>
                </tr>
                <tr>
                    <td><code>/users/:username/recommendations</code></td>
                    <td>GET</td>
                    <td>Get connection recommendations for a user</td>
                </tr>
                <tr>
                    <td><code>/interests/:interest/users</code></td>
                    <td>GET</td>
                    <td>Find users with a specific interest</td>
                </tr>
            </tbody>
        </table>
        
        <h2>Getting Started</h2>
        
        <h3>Prerequisites</h3>
        <ul>
            <li>Rust (latest stable version)</li>
            <li>Cargo</li>
        </ul>
        
        <h3>Installation</h3>
        <pre><code>git clone https://github.com/your-username/rustafari.git
cd rustafari
cargo build --release
cargo run --release</code></pre>
        
        <p>The server will start at <code>http://127.0.0.1:3000</code></p>
        
        <a href="https://github.com/Muchangi001/rustafari" class="cta">Get Started on GitHub</a>
        
        <footer>
            <p>This project is licensed under the MIT License</p>
        </footer>
    </div>
</body>
</html>
    "#))
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