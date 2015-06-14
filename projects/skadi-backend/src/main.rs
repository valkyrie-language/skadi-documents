use skadi::{SkadiError, SkadiService};

#[tokio::main]
async fn main() -> Result<(), SkadiError> {
    let here = std::env::current_dir()?;
    let service = SkadiService::load(&here.join("skadi.toml")).await?;
    service.serve().await
}
