use std::sync::Arc;

use async_graphql::{Context, Object, ID};
use tracing::info;

use crate::{
    schema::{Beer, BeerStyle, Brewery, Ingredient, Review},
    state::AppState,
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn brewery(&self, ctx: &Context<'_>, id: ID) -> anyhow::Result<Brewery> {
        let state = ctx.data::<Arc<AppState>>().unwrap();
        let id = id.0.parse::<i32>()?;

        info!("searching for brewery {id}");

        let brewery = sqlx::query_as!(
            Brewery,
            r#"
                SELECT *
                FROM breweries
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&state.pool)
        .await?;

        Ok(brewery)
    }

    async fn breweries(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Brewery>> {
        let state = ctx.data::<Arc<AppState>>().unwrap();

        info!("retrieving all breweries");

        let breweries = sqlx::query_as!(
            Brewery,
            r#"
                SELECT *
                FROM breweries
            "#
        )
        .fetch_all(&state.pool)
        .await?;

        Ok(breweries)
    }

    async fn beer(&self, ctx: &Context<'_>, id: ID) -> anyhow::Result<Beer> {
        let state = ctx.data::<Arc<AppState>>().unwrap();
        let id = id.0.parse::<i32>()?;

        info!("searching for beer {id}");

        let beer = sqlx::query_as!(
            Beer,
            r#"
                SELECT *
                FROM beers
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&state.pool)
        .await?;

        Ok(beer)
    }

    async fn beers(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Beer>> {
        let state = ctx.data::<Arc<AppState>>().unwrap();

        info!("retrieving all beers");

        let beers = sqlx::query_as!(
            Beer,
            r#"
                SELECT *
                FROM beers
            "#
        )
        .fetch_all(&state.pool)
        .await?;

        Ok(beers)
    }

    async fn beer_style(&self, ctx: &Context<'_>, id: ID) -> anyhow::Result<BeerStyle> {
        let state = ctx.data::<Arc<AppState>>().unwrap();
        let id = id.0.parse::<i32>()?;

        info!("searching for beer style {id}");

        let style = sqlx::query_as!(
            BeerStyle,
            r#"
                SELECT *
                FROM beer_styles
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&state.pool)
        .await?;

        Ok(style)
    }

    async fn beer_styles(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<BeerStyle>> {
        // Implement beer styles fetching logic
        todo!()
    }

    async fn ingredient(&self, ctx: &Context<'_>, id: ID) -> anyhow::Result<Ingredient> {
        // Implement ingredient fetching logic
        todo!()
    }

    async fn ingredients(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Ingredient>> {
        // Implement ingredients fetching logic
        todo!()
    }

    async fn reviews_for_beer(
        &self,
        ctx: &Context<'_>,
        beer_id: ID,
    ) -> anyhow::Result<Vec<Review>> {
        todo!()
    }
}
