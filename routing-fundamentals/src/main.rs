use axum::{
    extract::Path,
    routing::{get, post},
    Router,
};

// 1. Basic Handlers
async fn home_handler()->&'static str{
    "welcome to home page (GET /)"
}

async fn about_handler() -> &'static str {
    "This is the About Page (GET /about)"
}

// 2. Handlers for Chained Methods (/users)
async fn list_users_handler() -> &'static str {
    "List of all users (GET /users)"
}

async fn create_user_handler() -> &'static str {
    "User created successfully (POST /users)"
}

// 3. Path Parameter Handler (Axum 0.8 syntax: {id})
async fn get_user_by_id_handler(Path(id): Path<u32>) -> String {
    format!("Fetching user profile for ID: {} (GET /users/{{id}})", id)
}

// 4. Nested Router Handler (/api/v1/status)
async fn api_status_handler() -> &'static str {
    "API v1 is running smoothly (GET /api/v1/status)"
}

// 5. Merged Health Routes Function
fn health_routes() -> Router {
    Router::new()
        .route("/health", get(|| async { "Server is healthy! (GET /health)" }))
        .route("/ping", get(|| async { "pong! (GET /ping)" }))
}

#[tokio::main]
async fn main(){
    // --- A. Sub-router for API nesting ---
    let v1_router = Router::new()
        .route("/status", get(api_status_handler));

    // --- B. Main Router with Routing Fundamentals ---
    let app=Router::new()
    // 1. Basic Routes
    .route("/",get(home_handler))
    .route("/about",get(about_handler))

    // 2. Multiple Methods on One Route (Chaining)
        .route("/users", get(list_users_handler).post(create_user_handler))

    // 3. Path Parameters (Axum 0.8 style: {id} instead of :id)
    .route("/users/{id}", get(get_user_by_id_handler))

    // 4. Nested Routers using .nest()
    .nest("/api/v1",v1_router)

    // 5. Merged Routers using .merge() (Yeh bina kisi prefix ke direct paths add kar deta hai)
    .merge(health_routes());

    // Bind and start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server started successfully at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();

}