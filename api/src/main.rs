mod inputs;
mod mutations;
mod queries;
mod router;
mod schema;
mod state;

use anyhow::Context;
use router::build_hop_query_routes;
use schema::create_schema;
use state::AppState;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context("failed to initialize environment variables")?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("environment initialized, building application dependencies");

    let port = std::env::var("PORT")
        .context("port was not found in the environment")?
        .parse::<u16>()
        .context("port is not a valid u16")?;

    info!("initializing connection to database and graphql schema");

    let state = AppState::try_new().await?;
    let schema = create_schema();

    info!("postgres connection and schema initialized");

    let app = build_hop_query_routes(state, schema);
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("failed to bind to port {port}")?;

    info!("starting server on port {port}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
