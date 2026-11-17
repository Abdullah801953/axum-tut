use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct User{
    id:u64,
    name:String,
}

// 1. Plain Text Response
async fn plain_handler() -> &'static str {
    "Hello, Plain Text!"
}

// 2. Status Code Only Response
async fn no_content_handler() -> StatusCode {
    StatusCode::NO_CONTENT
}

// 3. JSON Response
async fn json_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "message": "Hello JSON", "status": "success" }))
}

// 4. HTML Response
async fn html_handler() -> Html<&'static str> {
    Html("<h1>Hello from HTML Response!</h1>")
}

// 5. Tuple Response (StatusCode + JSON Body)
async fn created_handler() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::CREATED,
        Json(serde_json::json!({ "id": 1, "created": true })),
    )
}

// 6. Custom Response Enum using IntoResponse (Bade APIs ke liye best practice)
enum ApiResponse {
    Ok,
    JsonUsers(Vec<User>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::Ok => StatusCode::OK.into_response(),
            Self::JsonUsers(users) => (StatusCode::OK, Json(users)).into_response(),
        }
    }
}

async fn custom_enum_handler() -> ApiResponse {
    let users = vec![
        User { id: 1, name: "Alice".into() },
        User { id: 2, name: "Bob".into() },
    ];
    ApiResponse::JsonUsers(users)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/text", get(plain_handler))
        .route("/nocontent", get(no_content_handler))
        .route("/json", get(json_handler))
        .route("/html", get(html_handler))
        .route("/created", get(created_handler))
        .route("/users", get(custom_enum_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Response demo server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

