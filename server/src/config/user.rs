use omnistat_core::types::latitude::Latitude;
use omnistat_core::types::longitude::Longitude;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigUser {
    pub latitude: Latitude,
    pub longitude: Longitude,
}
