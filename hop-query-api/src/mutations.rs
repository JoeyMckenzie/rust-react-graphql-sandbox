use std::sync::Arc;

use anyhow::anyhow;
use async_graphql::{Context, Object};
use bigdecimal::{BigDecimal, FromPrimitive};

use crate::{
    inputs::{CreateBeerInput, CreateBreweryInput},
    schema::{Beer, Brewery},
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
        let state = ctx.data::<Arc<AppState>>().unwrap();

        match BigDecimal::from_f64(input.abv) {
            Some(abv) => {
                let beer = sqlx::query_as!(
                    Beer,
                    r#"
                        INSERT INTO beers (name, brewery_id, style_id, abv, ibu, description, is_seasonal)
                        VALUES ($1, $2, $3, $4, $5, $6, $7)
                        RETURNING *;
                    "#,
                    input.name,
                    input.brewery_id.0.parse::<i32>()?,
                    input.style_id.0.parse::<i32>()?,
                    abv,
                    input.ibu,
                    input.description,
                    input.is_seasonal,
                )
                .fetch_one(&state.pool)
                .await?;

                Ok(beer)
            }
            None => Err(anyhow!("abv is not valid")),
        }
    }
}
