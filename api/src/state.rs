use std::env;

use anyhow::Context;
use axum::extract::FromRef;
use sqlx::PgPool;

use crate::schema::{create_schema, HopQuerySchema};

#[derive(FromRef, Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub async fn try_new() -> anyhow::Result<Self> {
        let connection_string = build_connection_string()?;
        let pool = PgPool::connect(&connection_string)
            .await
            .context("failed to connect to postgres")?;

        Ok(Self { pool })
    }
}

fn build_connection_string() -> anyhow::Result<String> {
    let pg_host = env::var("POSTGRES_HOST").unwrap_or("localhost".to_string());
    let connection_string =
        env::var("DATABASE_URL").context("connection string not found in environment")?;

    if pg_host.eq("localhost") {
        Ok(connection_string)
    } else {
        Ok(connection_string.replace("localhost", &pg_host))
    }
}
