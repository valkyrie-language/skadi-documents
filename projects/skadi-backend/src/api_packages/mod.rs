use crate::api_statistics::HomeStatistics;
use aide::axum::IntoApiResponse;
use axum::Json;
use chrono::{DateTime, Local};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, JsonSchema)]
pub struct PackageQueryByName {
    /// The owner of the package.
    organization: String,
    /// The name of the package.
    name: String,
    /// The specific version of the package to query.
    version: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct VersionQueryByName {
    /// The owner of the package.
    organization: String,
    /// The name of the package.
    name: String,
    /// The specific version of the package to query.
    sorter: Option<VersionSorter>,
}

#[derive(Deserialize, JsonSchema)]
pub enum VersionSorter {
    Version,
    Time,
    Downloads,
}

#[derive(Serialize, JsonSchema)]
pub struct PackageVersion {
    version: String,
    update_user: String,
    update_time: DateTime<Local>,
}

#[derive(Serialize, JsonSchema)]
pub struct PackageDetail {
    package_id: Uuid,
    organization: String,
    name: String,
    version: String,
    summary: String,
    documentation: String,
    author: String,
    license: String,
    repository: String,
    downloads: u64,
    update_time: DateTime<Local>,
    update_user: Uuid,
}

pub async fn find_package(query: Json<PackageQueryByName>) -> impl IntoApiResponse {
    Json(PackageDetail {
        package_id: Uuid::now_v7(),
        organization: query.0.organization,
        name: query.0.name,
        version: query.0.version.unwrap_or("0.1.0".to_string()),
        summary: "summary".to_string(),
        documentation: r#"
## markdown

```ts
const markdown = "test"
```

$\frac{1}{2}$ + $$\frac{1}{2}$$

        "#
        .to_string(),
        author: "author".to_string(),
        license: "license".to_string(),
        repository: "repository".to_string(),
        downloads: 1234,
        update_time: DateTime::default(),
        update_user: Uuid::now_v7(),
    })
}

pub async fn list_package_version(query: Json<PackageQueryByName>) -> impl IntoApiResponse {
    Json(vec![
        PackageVersion { version: "1.0".to_string(), update_user: "bb".to_string(), update_time: Default::default() },
        PackageVersion { version: "2.0".to_string(), update_user: "aa".to_string(), update_time: Default::default() },
    ])
}
