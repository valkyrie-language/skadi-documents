use crate::{SkadiError, SkadiService};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::{path::Path, time::Duration};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SkadiConfig {
    database_url: String,
}

impl SkadiService {
    pub async fn load(toml: &Path) -> Result<Self, SkadiError> {
        let s = match std::fs::read_to_string(toml) {
            Ok(o) => o,
            Err(_) => Err(SkadiError::from())?,
        };
        let config = toml::from_str::<SkadiConfig>(&s)?;
        config.instantiation().await
    }
}

impl SkadiConfig {
    pub async fn instantiation(self) -> Result<SkadiService, SkadiError> {
        let pool =
            PgPoolOptions::new().max_connections(5).acquire_timeout(Duration::from_secs(3)).connect(&self.database_url).await?;
        Ok(SkadiService { pg: pool })
    }
}
