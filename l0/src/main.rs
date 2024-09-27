use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use strum_macros::EnumString;
use tokio::sync::Mutex;

#[derive(Default)]
struct AppState {
    pub orders: Mutex<HashMap<usize, CreateOrder>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState::default());

    let app = Router::new()
        .route("/order/new", post(create_order))
        .route("/order", get(get_order))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Create new order and return its id
async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateOrder>,
) -> (StatusCode, String) {
    let mut orders = state.orders.lock().await;
    let id = orders.len();
    orders.insert(id, payload);

    (StatusCode::CREATED, format!("{id}"))
}

/// Get order by its id
// TODO: maybe use `order_uid` instead
async fn get_order(
    State(state): State<Arc<AppState>>,
    id: String,
) -> (StatusCode, Json<Option<CreateOrder>>) {
    let orders = state.orders.lock().await;
    let id: usize = id.parse::<usize>().unwrap();

    let Some(order) = orders.get(&id) else {
        return (StatusCode::NOT_FOUND, Json(None));
    };

    return (StatusCode::OK, Json(Some(order.clone())));
}

// TODO: add validation: CreateOrder -> Order
#[derive(Deserialize, Serialize, Default, Clone)]
struct CreateOrder {
    order_uid: String,
    track_number: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: Locale,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: usize,
    date_created: String,
    oof_shard: String,
}

#[derive(Deserialize, Serialize, Default, Clone)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

#[derive(Deserialize, Serialize, Default, Clone)]
struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    amount: usize,
    payment_dt: usize,
    bank: String,
    delivery_cost: usize,
    goods_total: usize,
    custom_fee: usize,
}

#[derive(Deserialize, Serialize, Default, Clone)]
struct Item {
    chrt_id: usize,
    track_number: String,
    price: usize,
    rid: String,
    name: String,
    sale: usize,
    size: String,
    total_price: usize,
    nm_id: usize,
    brand: String,
    status: usize,
}

#[derive(Deserialize, Serialize, Default, Clone, EnumString)]
#[strum(ascii_case_insensitive)]
enum Locale {
    #[default]
    ru,
    en,
}
