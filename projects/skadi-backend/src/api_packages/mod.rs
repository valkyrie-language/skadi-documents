use crate::api_statistics::HomeStatistics;
use aide::axum::IntoApiResponse;
use axum::Json;
use chrono::{DateTime, Local};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
pub struct PackageQueryByName {
    organization: String,
    name: String,
}

#[derive(Serialize, JsonSchema)]
pub struct PackageInfo {
    organization: String,
    name: String,
    version: String,
    description: String,
    author: String,
    license: String,
    repository: String,
    downloads: u64,
    last_update: DateTime<Local>,
}

pub async fn find_package(query: PackageQueryByName) -> impl IntoApiResponse {
    Json(PackageInfo {
        organization: query.organization,
        name: query.name,
        version: "0.1.0".to_string(),
        description: "description".to_string(),
        author: "author".to_string(),
        license: "license".to_string(),
        repository: "repository".to_string(),
        downloads: 1234,
        last_update: DateTime::default(),
    })
}
