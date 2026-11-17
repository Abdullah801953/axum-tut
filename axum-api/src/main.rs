use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // 1. Router define karna
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
 
    // 2. TCP listener bind karna port 3000 par
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    
    // 3. Server start karna
    axum::serve(listener, app).await.unwrap();
}