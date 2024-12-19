use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub id: Uuid,
    pub name: String,
    pub sector_id: Uuid,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Building {
    pub fn new(name: &str, sector_id: Uuid, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            id: Uuid::now_v7(),
            name: name.to_string(),
            sector_id,
            x,
            y,
            width,
            height,
        }
    }
}
