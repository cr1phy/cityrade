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

pub const HOUSE: Building = Building {
    id: 1,
    name: "House".to_string(),
    purpose: BuildingPurpose::Residential,
    status: BuildingStatus::Planning,
    size: (2, 2),
    coordinates: None,
};

pub const FARM: Building = Building {
    id: 2,
    name: "Farm".to_string(),
    purpose: BuildingPurpose::Agricultural,
    status: BuildingStatus::Planning,
    size: (3, 3),
    coordinates: None,
};

pub const FACTORY: Building = Building {
    id: 3,
    name: "Factory".to_string(),
    purpose: BuildingPurpose::Industrial,
    status: BuildingStatus::Planning,
    size: (4, 4),
    coordinates: None,
};

pub const PARK: Building = Building {
    id: 4,
    name: "Park".to_string(),
    purpose: BuildingPurpose::Public,
    status: BuildingStatus::Planning,
    size: (3, 3),
    coordinates: None,
};
