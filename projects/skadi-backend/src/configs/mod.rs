use crate::{SkadiError, SkadiService};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::{path::Path, time::Duration};

impl SkadiService {
    pub async fn load(toml: &Path) -> crate::Result<Self, SkadiError> {
        let config = toml::from_str::<SkadiConfig>(toml.to_str().unwrap())?;
        config.instantiation().await
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SkadiConfig {
    database_url: String,
}

impl SkadiConfig {
    pub async fn instantiation(self) -> crate::Result<SkadiService, SkadiError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(url)
            .await
            .expect("can't connect to database");
        Ok(SkadiService {})
    }
}
