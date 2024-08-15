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
    pub name: String,
    pub brewery_id: ID,
    pub style_id: ID,
    pub abv: f64,
    pub ibu: Option<i32>,
    pub description: Option<String>,
    pub is_seasonal: bool,
}

#[derive(InputObject)]
pub struct CreateReviewInput {
    beer_id: ID,
    user_name: String,
    rating: i32,
    comment: Option<String>,
}
