use crate::SkadiError;
use aide::{axum::IntoApiResponse, openapi::StatusCode};
use axum::{response::IntoResponse, Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// We'll need to derive `JsonSchema` for
// all types that appear in the api documentation.
#[derive(Serialize, JsonSchema)]
pub struct HomeStatistics {
    all_users: u64,
    all_packages: u64,
    all_downloads: u64,
}

pub async fn home_statistics() -> impl IntoApiResponse {
    Json(HomeStatistics { all_users: 1, all_packages: 2, all_downloads: 3 })
}
