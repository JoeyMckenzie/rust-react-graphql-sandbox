use async_graphql::{EmptySubscription, Schema, SimpleObject, ID};
use time::OffsetDateTime;

use crate::{mutations::MutationHandler, queries::QueryRoot};

#[derive(SimpleObject)]
pub struct Brewery {
    pub id: ID,
    pub name: String,
    pub location: String,
    pub year_established: Option<i32>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(SimpleObject)]
pub struct BeerStyle {
    id: ID,
    name: String,
    description: Option<String>,
    created_at: OffsetDateTime,
}

#[derive(SimpleObject)]
pub struct Beer {
    id: ID,
    name: String,
    brewery_id: ID,
    style_id: ID,
    abv: f64,
    ibu: Option<i32>,
    description: Option<String>,
    is_seasonal: bool,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

#[derive(SimpleObject)]
pub struct Ingredient {
    id: ID,
    name: String,
    ingredient_type: String,
    description: Option<String>,
    created_at: OffsetDateTime,
}

#[derive(SimpleObject)]
pub struct BeerIngredient {
    beer_id: ID,
    ingredient_id: ID,
    amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct Review {
    id: ID,
    beer_id: ID,
    user_name: String,
    rating: i32,
    comment: Option<String>,
    created_at: OffsetDateTime,
}

pub type HopQuerySchema = Schema<QueryRoot, MutationHandler, EmptySubscription>;

// Create the schema
pub fn create_schema() -> HopQuerySchema {
    Schema::build(QueryRoot, MutationHandler, EmptySubscription).finish()
}
