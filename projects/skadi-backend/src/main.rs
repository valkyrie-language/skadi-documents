use skadi::{SkadiError, SkadiService};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), SkadiError> {
    let service = SkadiService::load(&Path::new("skadi.toml")).await?;
    service.serve().await
}
