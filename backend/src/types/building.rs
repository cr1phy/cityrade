use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BuildingPurpose {
    Residential,
    Public,
    Industrial,
    Agricultural,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BuildingStatus {
    Planning,
    UnderConstruction,
    InUse,
    NeedsRepair,
    Abandoned,
    Demolished,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub id: u64,
    pub name: String,
    pub purpose: BuildingPurpose,
    pub status: BuildingStatus,
    pub size: (u32, u32),
    pub coordinates: Option<(i32, i32)>,
}

impl Building {
    pub fn new(id: u64, name: &str, purpose: BuildingPurpose, size: (u32, u32)) -> Self {
        Self {
            id,
            name: name.to_string(),
            purpose,
            status: BuildingStatus::Planning,
            size,
            coordinates: None,
        }
    }
}