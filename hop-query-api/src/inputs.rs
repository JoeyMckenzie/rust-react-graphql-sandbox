use async_graphql::{InputObject, ID};

#[derive(InputObject)]
pub struct CreateBreweryInput {
    pub name: String,
    pub location: String,
    pub year_established: Option<i32>,
    pub description: Option<String>,
    pub website: Option<String>,
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
