use aide::axum::ApiRouter;
use aide::openapi::{Info, OpenApi};
use axum::Extension;
use skadi::{SkadiError, SkadiService};

#[tokio::main]
async fn main() -> Result<(), SkadiError> {
    let service = SkadiService::load("skadi.toml")?;
    service.serve().await
}
