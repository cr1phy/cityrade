use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    building::{Building, BuildingType},
    resources::{ResourceType, Resource},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Terrain {
    Plain,
    Forest,
    Mountain,
    Desert,
    Swamp,
    Water,
    Snow,
}

impl Terrain {
    pub fn display_name(&self) -> &str {
        match self {
            Terrain::Plain => "Равнина",
            Terrain::Forest => "Лес",
            Terrain::Mountain => "Горы",
            Terrain::Desert => "Пустыня",
            Terrain::Swamp => "Болото",
            Terrain::Water => "Вода",
            Terrain::Snow => "Снег",
        }
    }

    pub fn resource_modifier(&self) -> HashMap<ResourceType, f32> {
        let mut modifiers = HashMap::new();

        match self {
            Terrain::Plain => {
                modifiers.insert(ResourceType::Food, 1.2);
                modifiers.insert(ResourceType::Gold, 1.0);
            }
            Terrain::Forest => {
                modifiers.insert(ResourceType::Wood, 1.5);
                modifiers.insert(ResourceType::Food, 0.8);
            }
            Terrain::Mountain => {
                modifiers.insert(ResourceType::Stone, 1.5);
                modifiers.insert(ResourceType::Iron, 1.3);
                modifiers.insert(ResourceType::Crystal, 1.2);
                modifiers.insert(ResourceType::Food, 0.6);
            }
            Terrain::Desert => {
                modifiers.insert(ResourceType::Crystal, 1.3);
                modifiers.insert(ResourceType::Food, 0.5);
                modifiers.insert(ResourceType::Wood, 0.3);
            }
            Terrain::Swamp => {
                modifiers.insert(ResourceType::Wood, 1.1);
                modifiers.insert(ResourceType::Food, 0.7);
            }
            Terrain::Water => {
                modifiers.insert(ResourceType::Food, 1.3);
                modifiers.insert(ResourceType::Gold, 1.1);
            }
            Terrain::Snow => {
                modifiers.insert(ResourceType::Crystal, 1.4);
                modifiers.insert(ResourceType::Food, 0.4);
                modifiers.insert(ResourceType::Energy, 0.8);
            }
        }

        modifiers
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityStats {
    pub happiness: u32,      // Счастье населения (влияет на рост)
    pub defense: u32,        // Защита от нападений
    pub culture: u32,        // Культурный уровень (влияет на технологии)
    pub max_population: u32, // Максимальное население
    pub max_buildings: u32,  // Максимальное количество зданий
}

impl Default for CityStats {
    fn default() -> Self {
        CityStats {
            happiness: 50,
            defense: 10,
            culture: 0,
            max_population: 50,
            max_buildings: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub population: u32,
    pub buildings: HashMap<String, Building>,
    pub resources: Resource,
    pub stats: CityStats,
    pub terrain: Terrain,
    pub position: (i32, i32),
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl City {
    pub fn new(name: String, owner_id: String, terrain: Terrain, position: (i32, i32)) -> City {
        City {
            id: Uuid::new_v4().to_string(),
            name,
            owner_id,
            population: 10,
            buildings: HashMap::new(),
            resources: Resource::new(),
            stats: CityStats::default(),
            terrain,
            position,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        }
    }

    pub fn update(&mut self) {
        // Обновляем ресурсы на основе зданий
        self.update_resource_production();

        // Обновляем статистику
        self.update_stats();

        // Обновляем население
        self.update_population();

        // Обновляем временную метку
        self.last_updated = Utc::now();
    }

    pub fn update_resource_production(&mut self) {
        // Сбрасываем производство к нулю
        let mut production_rates = HashMap::new();
        for resource_type in [
            ResourceType::Gold,
            ResourceType::Wood,
            ResourceType::Stone,
            ResourceType::Food,
            ResourceType::Iron,
            ResourceType::Crystal,
            ResourceType::Energy,
        ] {
            // Базовое производство
            let base_rate = match resource_type {
                ResourceType::Gold => 5,
                ResourceType::Wood => 3,
                ResourceType::Stone => 2,
                ResourceType::Food => 8,
                _ => 0,
            };
            production_rates.insert(resource_type, base_rate);
        }

        // Добавляем производство от зданий
        for building in self.buildings.values() {
            for (resource, amount) in building.production_effect() {
                *production_rates.entry(resource).or_insert(0) += amount;
            }
        }

        // Применяем модификаторы местности
        let terrain_modifiers = self.terrain.resource_modifier();
        for (resource, modifier) in terrain_modifiers {
            if let Some(amount) = production_rates.get_mut(&resource) {
                *amount = (*amount as f32 * modifier) as i32;
            }
        }

        // Устанавливаем новые значения производства
        for (resource, rate) in production_rates {
            self.resources.set_production_rate(resource, rate);
        }

        // Применяем производство к текущим ресурсам
        self.resources.update_production();
    }

    pub fn update_stats(&mut self) {
        // Сбрасываем статистику к базовым значениям
        self.stats = CityStats::default();

        // Обновляем статистику на основе зданий
        for building in self.buildings.values() {
            match building.building_type {
                BuildingType::Residential => {
                    self.stats.max_population += 10 * building.level;
                }
                BuildingType::Temple => {
                    self.stats.happiness += 5 * building.level;
                    self.stats.culture += 3 * building.level;
                }
                BuildingType::Wall => {
                    self.stats.defense += 20 * building.level;
                }
                BuildingType::Laboratory => {
                    self.stats.culture += 5 * building.level;
                }
                BuildingType::Barracks => {
                    self.stats.defense += 10 * building.level;
                    self.stats.happiness -= 2 * building.level;
                }
                _ => {}
            }
        }

        // Увеличиваем максимальное количество зданий на основе населения
        self.stats.max_buildings = 5 + (self.population / 20);
    }

    pub fn update_population(&mut self) {
        // Рост населения зависит от счастья и наличия еды
        let food = self.resources.get(&ResourceType::Food);

        // Если еды недостаточно, уменьшаем население
        if food < self.population {
            self.decrease_population(1);
            self.stats.happiness = self.stats.happiness.saturating_sub(5);
        } else {
            // Иначе, увеличиваем население с вероятностью, зависящей от счастья
            let growth_chance = (self.stats.happiness as f32) / 100.0;

            if rand::random::<f32>() < growth_chance && self.population < self.stats.max_population
            {
                self.increase_population(1);
            }
        }
    }

    pub fn add_building(
        &mut self,
        building_type: BuildingType,
        name: String,
        position: (i32, i32),
    ) -> Result<String, String> {
        // Проверка, не превышено ли максимальное количество зданий
        if self.buildings.len() >= self.stats.max_buildings as usize {
            return Err("Достигнут предел количества зданий".to_string());
        }

        // Проверка, нет ли уже здания в этой позиции
        for building in self.buildings.values() {
            if building.position == position {
                return Err("В этой позиции уже есть здание".to_string());
            }
        }

        // Проверка, хватает ли ресурсов
        let costs = building_type.base_cost();
        if !self.resources.can_afford(&costs) {
            return Err("Недостаточно ресурсов".to_string());
        }

        // Снимаем ресурсы
        self.resources.pay(&costs);

        // Создаем новое здание
        let id = Uuid::new_v4().to_string();
        let building = Building::new(id.clone(), name, building_type, position);

        // Добавляем здание
        self.buildings.insert(id.clone(), building);

        Ok(id)
    }

    pub fn upgrade_building(&mut self, building_id: &str) -> Result<(), String> {
        // Проверяем, существует ли здание
        let building = match self.buildings.get(building_id) {
            Some(b) => b,
            None => return Err("Здание не найдено".to_string()),
        };

        // Проверяем, хватает ли ресурсов
        let costs = building.upgrade_cost();
        if !self.resources.can_afford(&costs) {
            return Err("Недостаточно ресурсов".to_string());
        }

        // Снимаем ресурсы
        self.resources.pay(&costs);

        // Улучшаем здание
        let building = self.buildings.get_mut(building_id).unwrap();
        building.upgrade();

        Ok(())
    }

    pub fn remove_building(&mut self, building_id: &str) -> Result<(), String> {
        // Проверяем, существует ли здание
        if !self.buildings.contains_key(building_id) {
            return Err("Здание не найдено".to_string());
        }

        // Удаляем здание
        self.buildings.remove(building_id);

        Ok(())
    }

    pub fn increase_population(&mut self, amount: u32) {
        self.population = (self.population + amount).min(self.stats.max_population);
    }

    pub fn decrease_population(&mut self, amount: u32) {
        self.population = self.population.saturating_sub(amount);
    }

    pub fn get_resource_report(&self) -> String {
        let mut report = format!("Ресурсы города {}:\n", self.name);

        for (resource, amount) in self.resources.get_all_resources() {
            let production = self.resources.get_production_rate(&resource);
            let production_str = if production > 0 {
                format!("+{}", production)
            } else if production < 0 {
                format!("{}", production)
            } else {
                "0".to_string()
            };

            report.push_str(&format!("{}: {} ({})\n", resource, amount, production_str));
        }

        report
    }

    pub fn get_buildings_report(&self) -> String {
        let mut report = format!(
            "Здания города {} ({}/{})\n",
            self.name,
            self.buildings.len(),
            self.stats.max_buildings
        );

        for building in self.buildings.values() {
            report.push_str(&format!(
                "- {}, уровень {}\n",
                building.name, building.level
            ));
        }

        report
    }

    pub fn get_stats_report(&self) -> String {
        format!(
            "Статистика города {}:\n\
             Население: {}/{}\n\
             Счастье: {}\n\
             Защита: {}\n\
             Культура: {}\n\
             Местность: {}\n\
             Основан: {}\n",
            self.name,
            self.population,
            self.stats.max_population,
            self.stats.happiness,
            self.stats.defense,
            self.stats.culture,
            self.terrain.display_name(),
            self.created_at.format("%d.%m.%Y")
        )
    }

    pub fn subtract_resources(&mut self, resource_type: &ResourceType, amount: u32) -> bool {
        let current = self.resources.get(resource_type);
        
        if current >= amount {
            self.resources.subtract(resource_type, amount);
            true
        } else {
            false
        }
    }

    pub fn add_resources(&mut self, resource_type: &ResourceType, amount: u32) {
        self.resources.add(resource_type, amount);
    }
}
