use axum::routing::post;
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};

mod handlers;

#[tokio::main]
async fn main() {
    // init tracing
    tracing_subscriber::fmt::init();

    // cors
    let cors = CorsLayer::new().allow_origin(Any);

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
        .route("/", get(hello))
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

    tracing::debug!("listening on {}", "0.0.0.0:3000");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "Hello world again!"
}
