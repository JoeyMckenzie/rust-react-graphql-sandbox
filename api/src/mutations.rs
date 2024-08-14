use async_graphql::{Context, Object, ID};

use crate::{
    inputs::{CreateBeerInput, CreateBreweryInput, CreateReviewInput},
    schema::{Beer, BeerIngredient, Brewery, Review},
};

pub struct MutationHandler;

#[Object]
impl MutationHandler {
    async fn create_brewery(
        &self,
        ctx: &Context<'_>,
        input: CreateBreweryInput,
    ) -> anyhow::Result<Brewery> {
        todo!()
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
