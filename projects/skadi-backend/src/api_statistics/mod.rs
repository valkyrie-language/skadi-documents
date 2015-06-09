use crate::SkadiError;
use aide::{axum::IntoApiResponse, openapi::StatusCode};
use axum::{response::IntoResponse, Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// We'll need to derive `JsonSchema` for
// all types that appear in the api documentation.
#[derive(Serialize, JsonSchema)]
pub struct HomeStatistics {
    name: String,
}

pub async fn home_statistics() -> impl IntoApiResponse {
    Json(HomeStatistics { name: "".to_string() })
}
