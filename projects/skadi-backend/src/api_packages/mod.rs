use crate::api_statistics::HomeStatistics;
use aide::axum::IntoApiResponse;
use axum::Json;
use chrono::{DateTime, Local};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, JsonSchema)]
pub struct PackageQueryByName {
    organization: String,
    name: String,
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
        version: "0.1.0".to_string(),
        summary: "summary".to_string(),
        documentation: r#"

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
