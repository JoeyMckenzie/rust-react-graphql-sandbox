use async_graphql::{EmptySubscription, Schema, SimpleObject, ID};
use sqlx::types::BigDecimal;
use time::OffsetDateTime;

use crate::{mutations::MutationRoot, queries::QueryRoot};

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
    pub id: ID,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}

#[derive(SimpleObject)]
pub struct Beer {
    pub id: ID,
    pub name: String,
    pub brewery_id: ID,
    pub style_id: ID,
    pub abv: BigDecimal,
    pub ibu: Option<i32>,
    pub description: Option<String>,
    pub is_seasonal: Option<bool>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(SimpleObject)]
pub struct Ingredient {
    pub id: ID,
    pub name: String,
    pub ingredient_type: String,
    pub description: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}

#[derive(SimpleObject)]
pub struct BeerIngredient {
    beer_id: ID,
    ingredient_id: ID,
    amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct Review {
    pub id: ID,
    pub beer_id: ID,
    pub user_name: String,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}

pub type HopQuerySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

// Create the schema
pub fn create_schema() -> HopQuerySchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}
