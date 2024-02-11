use axum::{
    // body::Body,
    // http::StatusCode,
    // response::{IntoResponse, Response},
    routing::{get}, // post
    Json, Router,
};
use diesel::prelude::*;
use portier::models::Account;

async fn list_accounts() -> Json<Vec<Account>> {

    use portier::schema::accounts::dsl::{accounts};

    let connection = &mut portier::establish_connection();
    let results = accounts
        .select(Account::as_select())
        .load(connection)
        .expect("Error loading posts");

    Json(results)
}

#[tokio::main]
async fn main() {
    // Define Routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/api/accounts", get(list_accounts));

    println!("Running on http://localhost:3000");
    // Start Server
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
