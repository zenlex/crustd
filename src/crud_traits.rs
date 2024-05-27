use async_trait::async_trait;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::PgPool;
use std::fmt::Debug;
use validator::Validate;

// ? This does not require FromRow as a trait bound but my sqlx implementations do for query_as...may pose a problem for generics - look at restructuring sqlx calls if possible to avoid unnecessary trait bounds.
#[async_trait]
pub trait CrudService {
    type T: Serialize + DeserializeOwned + Send + Sync; // Main Struct (Model)
    type P: Debug + Serialize + DeserializeOwned + Validate + Send + Sync; // Pending Struct (Create)
    type U: Serialize + DeserializeOwned + Validate + Send + Sync; // Update Struct

    async fn create(pool: &PgPool, data: Self::P) -> sqlx::Result<Self::T>;
    async fn get_all(pool: &PgPool) -> sqlx::Result<Vec<Self::T>>;
    async fn find(pool: &PgPool, id: i32) -> sqlx::Result<Self::T>;
    async fn update(pool: &PgPool, id: i32, data: Self::U) -> sqlx::Result<Self::T>;
    async fn delete(pool: &PgPool, id: i32) -> sqlx::Result<()>;
    async fn count(pool: &PgPool) -> sqlx::Result<i64>;
    async fn factory(pool: &PgPool) -> sqlx::Result<Self::T>;
}

#[async_trait]
pub trait CrudController: CrudService {
    //TODO: Add extractor for multipart/form-data for file uploads and update service (Use blog markdown uploads as test case)
    //TODO: remove the debug requirement and add tracing loggers instead
    async fn create<T>(
        State(pool): State<PgPool>,
        Json(data): Json<Self::P>,
    ) -> Result<Json<Self::T>, (StatusCode, String)> {
        match data.validate() {
            Ok(_) => {}
            Err(e) => {
                return Err((StatusCode::BAD_REQUEST, e.to_string()));
            }
        };
        match <Self as CrudService>::create(&pool, data).await {
            Ok(record) => Ok(Json(record)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    async fn index(State(pool): State<PgPool>) -> Result<Json<Vec<Self::T>>, (StatusCode, String)> {
        match <Self as CrudService>::get_all(&pool).await {
            Ok(users) => Ok(Json(users)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    async fn show(
        Path(id): Path<i32>,
        State(pool): State<PgPool>,
    ) -> Result<Json<Self::T>, (StatusCode, String)> {
        match <Self as CrudService>::find(&pool, id).await {
            Ok(user) => Ok(Json(user)),
            Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
        }
    }

    async fn update(
        State(pool): State<PgPool>,
        Path(id): Path<i32>,
        Json(data): Json<Self::U>,
    ) -> Result<Json<Self::T>, (StatusCode, String)> {
        match data.validate() {
            Ok(_) => {}
            Err(e) => {
                return Err((StatusCode::BAD_REQUEST, e.to_string()));
            }
        }
        match <Self as CrudService>::update(&pool, id, data).await {
            Ok(user) => Ok(Json(user)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    async fn store(
        State(pool): State<PgPool>,
        Json(data): Json<Self::P>,
    ) -> Result<Json<Self::T>, (StatusCode, String)> {
        match data.validate() {
            Ok(_) => {}
            Err(e) => return Err((StatusCode::BAD_REQUEST, e.to_string())),
        }
        match <Self as CrudService>::create(&pool, data).await {
            Ok(user) => Ok(Json(user)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    async fn destroy(
        Path(id): Path<i32>,
        State(pool): State<PgPool>,
    ) -> Result<(), (StatusCode, String)> {
        match <Self as CrudService>::delete(&pool, id).await {
            Ok(_) => Ok(()),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }
}

pub trait CrudRouter: CrudController
where
    Self: Send + Sync + 'static,
{
    fn crud_router(pool: PgPool) -> Router {
        Router::new()
            .route(
                "/",
                get(<Self as CrudController>::index).post(<Self as CrudController>::store),
            )
            .route(
                "/:id",
                get(<Self as CrudController>::show)
                    .put(<Self as CrudController>::update)
                    .delete(<Self as CrudController>::destroy),
            )
            .with_state(pool)
    }
}
