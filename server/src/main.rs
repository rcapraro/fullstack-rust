use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};

use http::header::CONTENT_TYPE;
use http::Method;

mod handlers;

#[tokio::main]
async fn main() {
    // init tracing
    tracing_subscriber::fmt::init();

    // cors
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    // add postgres
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error creating pool");

    sqlx::migrate!("db/migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    // build our application with a single route
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/api/products", post(handlers::create_product))
        .route("/api/products", get(handlers::get_all_products))
        .route(
            "/api/products/:id",
            get(handlers::get_one_product)
                .delete(handlers::delete_product)
                .put(handlers::update_product),
        )
        .with_state(pool)
        .layer(cors);

    tracing::info!("listening on {}", "0.0.0.0:3000");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "Hello world!"
}
