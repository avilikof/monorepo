use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Coordinates {
    latitude: f64,
    longitude: f64,
}

// Struct for Place
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub code: String,
    pub name: String,
    pub administrative_division: String,
    pub country_code: String,
    pub coordinates: Coordinates,
}

impl Place {
    pub fn as_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self)
    }
}
