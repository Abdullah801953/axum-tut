use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;

// 1. Central Error Enum
enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized,
    Internal(String),
}

// 2. AppError -> HTTP Response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            AppError::Internal(msg) => {
                eprintln!("Internal error log: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".into(),
                )
            }
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

// 3. ParseIntError -> AppError conversion (`?` ke saath use hone ke liye)
impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::BadRequest(format!("Discount must be a valid number: {err}"))
    }
}

#[derive(Serialize)]
struct Product {
    id: u64,
    name: String,
    price: u64,
}

#[derive(Deserialize)]
struct OrderRequest {
    product_id: u64,
    quantity: u32,
    discount_str: String,
    auth_token: Option<String>,
}

#[derive(Serialize)]
struct OrderCreated {
    product_id: u64,
    final_amount: u64,
}

// GET /products/{id}
async fn get_product(Path(id): Path<u64>) -> Result<Json<Product>, AppError> {
    if id == 999 {
        return Err(AppError::Internal("Warehouse DB timeout".into()));
    }
    if id == 0 || id > 50 {
        return Err(AppError::NotFound(format!("Product #{id} not found")));
    }

    Ok(Json(Product {
        id,
        name: format!("Product #{id}"),
        price: 1000,
    }))
}

// POST /orders
async fn place_order(
    Json(input): Json<OrderRequest>,
) -> Result<(StatusCode, Json<OrderCreated>), AppError> {
    if input.auth_token.as_deref() != Some("user-token-123") {
        return Err(AppError::Unauthorized);
    }

    if input.quantity == 0 {
        return Err(AppError::BadRequest("Quantity must be at least 1".into()));
    }

    // `?` ParseIntError ko AppError::BadRequest me convert karega agar input number na ho
    let discount: u64 = input.discount_str.trim().parse()?;
    if discount > 70 {
        return Err(AppError::BadRequest("Max discount allowed is 70%".into()));
    }

    let subtotal = 1000 * u64::from(input.quantity);
    let final_amount = subtotal - (subtotal * discount / 100);

    Ok((
        StatusCode::CREATED,
        Json(OrderCreated {
            product_id: input.product_id,
            final_amount,
        }),
    ))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/products/{id}", get(get_product))
        .route("/orders", post(place_order));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}