#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod api_sts;
mod configs;
mod errors;

pub use crate::errors::{Result, SkadiError, SkadiErrorKind};
// Replace some of the `axum::` types with `aide::axum::` ones.
use aide::{
    axum::{
        routing::{get, post}, ApiRouter,
        IntoApiResponse,
    },
    openapi::{Info, OpenApi},
};
use axum::{Extension, Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

pub struct SkadiService {
    pg: Pool<Postgres>,
}

impl SkadiService {
    pub async fn serve(self) -> Result<()> {
        let app = ApiRouter::new()
            // Change `route` to `api_route` for the route
            // we'd like to expose in the documentation.
            .api_route("/home/statistics", post(api_sts::home_statistics))
            // We'll serve our generated document here.
            .route("/api.json", get(serve_api));

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

// Note that this clones the document on each request.
// To be more efficient, we could wrap it into an Arc,
// or even store it as a serialized string.
pub async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}
