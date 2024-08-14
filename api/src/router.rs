use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, response::Html, routing::get, Extension, Router};

use crate::{schema::HopQuerySchema, state::AppState};

pub fn build_hop_query_routes(state: AppState, schema: HopQuerySchema) -> Router {
    let state = Arc::new(state);
    Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema))
        .with_state(state)
}

async fn graphql_handler(
    schema: Extension<HopQuerySchema>,
    State(state): State<Arc<AppState>>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema
        .execute(request.into_inner().data(state))
        .await
        .into()
}

async fn graphql_playground() -> Html<String> {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
