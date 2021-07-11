use actix_web::{web, HttpResponse, ResponseError};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct MyError(String); // <-- needs debug and display

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A validation error occured on the input.")
    }
}

impl ResponseError for MyError {} // <-- key

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name="Adding a new subscriber",
    skip(form, pool),
    fields(
        email = %form.email,
        name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> Result<HttpResponse, MyError> {
    insert_subscriber(&pool, &form).await.map_err(|e| MyError(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Saving new subscriber details in the database", skip(form, pool))]
async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
