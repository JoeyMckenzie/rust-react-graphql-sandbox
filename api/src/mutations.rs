use std::sync::Arc;

use async_graphql::{Context, Object, ID};

use crate::{
    inputs::{CreateBeerInput, CreateBreweryInput, CreateReviewInput},
    schema::{Beer, BeerIngredient, Brewery, Review},
    state::AppState,
};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_brewery(
        &self,
        ctx: &Context<'_>,
        input: CreateBreweryInput,
    ) -> anyhow::Result<Brewery> {
        let state = ctx.data::<Arc<AppState>>().unwrap();

        let brewery = sqlx::query_as!(
            Brewery,
            r#"
                INSERT INTO breweries (name, location, year_established, description, website)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING *;
            "#,
            input.name,
            input.location,
            input.year_established,
            input.description,
            input.website,
        )
        .fetch_one(&state.pool)
        .await?;

        Ok(brewery)
    }

    async fn create_beer(&self, ctx: &Context<'_>, input: CreateBeerInput) -> anyhow::Result<Beer> {
        // Implement beer creation logic
        todo!()
    }

    async fn create_review(
        &self,
        ctx: &Context<'_>,
        input: CreateReviewInput,
    ) -> anyhow::Result<Review> {
        // Implement review creation logic
        todo!()
    }

    async fn add_ingredient_to_beer(
        &self,
        ctx: &Context<'_>,
        beer_id: ID,
        ingredient_id: ID,
        amount: Option<String>,
    ) -> anyhow::Result<BeerIngredient> {
        todo!()
    }
}
