use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;

use crate::types::building::{Building, BuildingPurpose};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShopItem {
    pub id: u64,
    pub name: String,
    pub purpose: BuildingPurpose,
    pub size: (u32, u32),
    pub cost: f64,
}

impl ShopItem {
    pub fn to_building(&self) -> Building {
        Building {
            id: self.id,
            name: self.name.clone(),
            purpose: self.purpose.clone(),
            size: self.size,
            coordinates: None,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Shop {
    pub items: Vec<ShopItem>,
}

impl Shop {
    pub fn new_from_json(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Unable to read store JSON");
        let items: Vec<ShopItem> = from_str(&data).expect("Unable to parse store JSON");
        Shop { items }
    }

    pub fn list_items(&self) -> &Vec<ShopItem> {
        &self.items
    }

    pub fn purchase(&self, item_id: u64, player_money: &mut f64) -> Option<Building> {
        self.items.iter().find(|item| item.id == item_id).and_then(|item| {
            if *player_money >= item.cost {
                *player_money -= item.cost;
                Some(item.to_building())
            } else {
                None
            }
        })
    }
}
