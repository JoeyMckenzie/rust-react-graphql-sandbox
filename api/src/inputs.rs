use async_graphql::{InputObject, ID};

#[derive(InputObject)]
pub struct CreateBreweryInput {
    name: String,
    location: String,
    year_established: Option<i32>,
    description: Option<String>,
    website: Option<String>,
}

#[derive(InputObject)]
pub struct CreateBeerInput {
    name: String,
    brewery_id: ID,
    style_id: ID,
    abv: f64,
    ibu: Option<i32>,
    description: Option<String>,
    is_seasonal: bool,
}

#[derive(InputObject)]
pub struct CreateReviewInput {
    beer_id: ID,
    user_name: String,
    rating: i32,
    comment: Option<String>,
}
