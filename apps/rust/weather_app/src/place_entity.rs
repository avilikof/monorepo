use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Coordinates {
    latitude: f64,
    longitude: f64,
}

// Struct for Place
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    code: String,
    name: String,
    administrative_division: String,
    country_code: String,
    coordinates: Coordinates,
}

impl Place {
    pub fn as_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self)
    }
}
