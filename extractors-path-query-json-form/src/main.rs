use axum::{
    Router,
    extract::{Json, Path, Query},
    routing::{get, post},
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Pagination {
    page: Option<u32>,
    per_page: Option<u32>,
}

// 2. JSON Body ke liye struct
#[derive(Deserialize, Debug)]
struct CreateUser {
    name: String,
    email: String,
}

// --- Handlers using Extractors ---

// A. Single Path Parameter Handler (GET /users/{id})
async fn get_user(Path(id): Path<u64>) -> String {
    format!("User Profile -> ID: #{id}")
}

// B. Multiple Path Parameters Handler (GET /posts/{post_id}/comments/{comment_id})
async fn get_comment(Path((post_id, comment_id)): Path<(u64, u64)>) -> String {
    format!("Post ID: {post_id}, Comment ID: {comment_id}")
}

// C. Query Parameters Handler (GET /items?page=2&per_page=50)
async fn list_items(Query(pagination): Query<Pagination>) -> String {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(20);
    format!("Displaying Page {page} with {per_page} items per page.")
}

// D. JSON Body Handler (POST /users)
async fn create_user(Json(input): Json<CreateUser>) -> String {
    format!(
        "User Created Successfully! Name: {}, Email: {}",
        input.name, input.email
    )
}

#[tokio::main]
async fn main() {
    // Router definition mapping routes to extractors handlers
    let app = Router::new()
        .route("/users/{id}", get(get_user))
        .route("/posts/{post_id}/comments/{comment_id}", get(get_comment))
        .route("/items", get(list_items))
        .route("/users", post(create_user));OLMESARTAN 40MG + HCTZ 12.5MG + AMLODIPINE 5MG OFLOXA ROUND (10X10 - ALU)

    // Bind and start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Extractors demo server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
