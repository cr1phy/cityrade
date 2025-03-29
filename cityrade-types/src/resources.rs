// cityrade-types/src/resources.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::default::Default;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Food,
    Wood,
    Stone,
    Iron,
    Gold,
    Coal,
    Tools,
    LuxuryGoods,
    Population,
    Energy,
    Crystal,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResourceType::Gold => write!(f, "Золото"),
            ResourceType::Wood => write!(f, "Дерево"),
            ResourceType::Stone => write!(f, "Камень"),
            ResourceType::Food => write!(f, "Еда"),
            ResourceType::Iron => write!(f, "Железо"),
            ResourceType::Crystal => write!(f, "Кристаллы"),
            ResourceType::Population => write!(f, "Население"),
            ResourceType::Energy => write!(f, "Энергия"),
            ResourceType::Coal => write!(f, "Уголь"),
            ResourceType::Tools => write!(f, "Инструменты"),
            ResourceType::LuxuryGoods => write!(f, "Роскошные товары"),
        }
    }
}

impl Default for ResourceType {
    fn default() -> Self {
        ResourceType::Food
    }
}

impl FromStr for ResourceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "food" => Ok(ResourceType::Food),
            "wood" => Ok(ResourceType::Wood),
            "stone" => Ok(ResourceType::Stone),
            "iron" => Ok(ResourceType::Iron),
            "gold" => Ok(ResourceType::Gold),
            "coal" => Ok(ResourceType::Coal),
            "tools" => Ok(ResourceType::Tools),
            "luxurygoods" => Ok(ResourceType::LuxuryGoods),
            "population" => Ok(ResourceType::Population),
            "energy" => Ok(ResourceType::Energy),
            "crystal" => Ok(ResourceType::Crystal),
            _ => Err(format!("Неизвестный тип ресурса: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingType {
    Farm,
    Mine,
    Sawmill,
    House,
    Market,
    Bank,
    School,
    PowerPlant,
    Laboratory,
    CrystalMine,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BuildingEffect {
    ProductionBoost,
    EfficiencyIncrease,
    StorageCapacity,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Resource {
    resources: HashMap<ResourceType, u32>,
    production_rate: HashMap<ResourceType, i32>,
}

impl Resource {
    pub fn new() -> Self {
        let mut resources = HashMap::new();
        let mut production_rate = HashMap::new();

        // Стандартные ресурсы
        resources.insert(ResourceType::Gold, 100);
        resources.insert(ResourceType::Wood, 100);
        resources.insert(ResourceType::Stone, 50);
        resources.insert(ResourceType::Food, 200);
        resources.insert(ResourceType::Iron, 0);
        resources.insert(ResourceType::Crystal, 0);
        resources.insert(ResourceType::Population, 10);
        resources.insert(ResourceType::Energy, 50);

        // Начальная скорость производства
        production_rate.insert(ResourceType::Gold, 5);
        production_rate.insert(ResourceType::Wood, 8);
        production_rate.insert(ResourceType::Stone, 3);
        production_rate.insert(ResourceType::Food, 10);
        production_rate.insert(ResourceType::Iron, 0);
        production_rate.insert(ResourceType::Coal, 0);
        production_rate.insert(ResourceType::Tools, 0);
        production_rate.insert(ResourceType::LuxuryGoods, 0);

        Resource {
            resources,
            production_rate,
        }
    }

    pub fn with_values(values: HashMap<ResourceType, u32>) -> Self {
        let mut resources = Resource::new();
        for (resource_type, amount) in values {
            resources.set(resource_type, amount);
        }
        resources
    }

    pub fn get(&self, resource: &ResourceType) -> u32 {
        *self.resources.get(resource).unwrap_or(&0)
    }

    pub fn set(&mut self, resource: ResourceType, amount: u32) {
        self.resources.insert(resource, amount);
    }

    pub fn add(&mut self, resource: &ResourceType, amount: u32) {
        let current = self.get(resource);
        self.resources.insert(resource.clone(), current + amount);
    }

    pub fn subtract(&mut self, resource: &ResourceType, amount: u32) -> bool {
        let current = self.get(resource);
        if current >= amount {
            self.resources.insert(resource.clone(), current - amount);
            true
        } else {
            false
        }
    }

    pub fn get_production_rate(&self, resource: &ResourceType) -> i32 {
        *self.production_rate.get(resource).unwrap_or(&0)
    }

    pub fn set_production_rate(&mut self, resource: ResourceType, rate: i32) {
        self.production_rate.insert(resource, rate);
    }

    pub fn update_production(&mut self) {
        for (resource, rate) in self.production_rate.clone() {
            if rate > 0 {
                self.add(&resource, rate as u32);
            } else if rate < 0 {
                // Игнорируем результат вычитания - если ресурсов не хватает,
                // они просто становятся равными нулю
                let _ = self.subtract(&resource, (-rate) as u32);
            }
        }
    }

    pub fn can_afford(&self, costs: &[(ResourceType, u32)]) -> bool {
        costs
            .iter()
            .all(|(resource, amount)| self.get(resource) >= *amount)
    }

    pub fn pay(&mut self, costs: &[(ResourceType, u32)]) -> bool {
        if self.can_afford(costs) {
            for (resource, amount) in costs {
                self.subtract(resource, *amount);
            }
            true
        } else {
            false
        }
    }

    pub fn get_all_resources(&self) -> Vec<(ResourceType, u32)> {
        self.resources
            .iter()
            .map(|(rt, &amount)| (rt.clone(), amount))
            .collect()
    }

    pub fn get_all_production_rates(&self) -> Vec<(ResourceType, i32)> {
        self.production_rate
            .iter()
            .map(|(rt, &rate)| (rt.clone(), rate))
            .collect()
    }
}
