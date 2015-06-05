use axum::Json;
use schemars::JsonSchema;
use serde::Deserialize;
use crate::{ SkadiError};


// We'll need to derive `JsonSchema` for
// all types that appear in the api documentation.
#[derive(Deserialize, JsonSchema)]
pub struct HomeStatistics {
    name: String,
}

pub async fn home_statistics() -> Result<HomeStatistics, SkadiError> {
    Ok(HomeStatistics { name: "".to_string() })
}

