#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod api_statistics;
mod configs;
mod errors;

pub use crate::errors::{Result, SkadiError};
// Replace some of the `axum::` types with `aide::axum::` ones.
use aide::{
    axum::{
        routing::get, ApiRouter,
        IntoApiResponse,
    },
    openapi::{Info, OpenApi},
};
use axum::{Extension, Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Debug)]
pub struct SkadiService {
    pg: Pool<Postgres>,
}

impl SkadiService {
    pub async fn serve(self) -> Result<()> {
        let app = ApiRouter::new()
            // Change `route` to `api_route` for the route
            // we'd like to expose in the documentation.
            .api_route("/home/statistics", get(api_statistics::home_statistics))
            // We'll serve our generated document here.
            .route("/api.json", get(open_api));

        let mut api =
            OpenApi { info: Info { description: Some("an example API".to_string()), ..Info::default() }, ..OpenApi::default() };

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

        axum::serve(
            listener,
            app
                // Generate the documentation.
                .finish_api(&mut api)
                // Expose the documentation to the handlers.
                .layer(Extension(api))
                .into_make_service(),
        )
        .await?;
        Ok(())
    }
}
/// Export the generated OpenAPI schema.
pub async fn open_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}
