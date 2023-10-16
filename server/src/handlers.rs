use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;
use tracing::info;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    id: i32,
    name: String,
    price: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewProduct {
    name: String,
    price: i32,
}

pub async fn create_product(
    State(pool): State<PgPool>,
    Json(product): Json<NewProduct>,
) -> Result<Json<NewProduct>, (StatusCode, String)> {
    info!(product.name, product.price, "inserting new product with");
    sqlx::query("insert into products (name, price) values ($1, $2)")
        .bind(&product.name)
        .bind(product.price)
        .execute(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {}", err),
            )
        })?;

    Ok(Json(product))
}

pub async fn get_all_products(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    info!("getting all products");
    let result = sqlx::query_as("select * from products")
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {}", err),
            )
        })?;

    Ok(Json(result))
}

pub async fn get_one_product(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Product>, (StatusCode, String)> {
    info!(id, "getting product by");
    let result = sqlx::query_as("select id, name, price from products where id=$1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {}", err),
            ),
        })?;

    Ok(Json(result))
}

pub async fn delete_product(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, String)> {
    info!(id, "deleting product with");
    sqlx::query("delete from products where id=$1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {}", err),
            ),
        })?;

    Ok(Json(
        json!({"message": format!("product with id {} deleted successfully!", id)}),
    ))
}

pub async fn update_product(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(product): Json<NewProduct>,
) -> Result<Json<Value>, (StatusCode, String)> {
    info!(id, "updating product with");
    sqlx::query("update products set name=$1, price=$2 where id=$3")
        .bind(&product.name)
        .bind(product.price)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {}", err),
            ),
        })?;

    Ok(Json(
        json!({"message": format!("product with id {} updated successfully!", id)}),
    ))
}
