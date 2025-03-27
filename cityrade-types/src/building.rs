use crate::resources::{ResourceType, Resource};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildingType {
    Residential, // Увеличивает лимит населения
    Farm,        // Производит еду
    LumberMill,  // Производит дерево
    Mine,        // Производит камень и железо
    Market,      // Увеличивает доход золота
    Barracks,    // Тренирует войска
    PowerPlant,  // Производит энергию
    Laboratory,  // Исследования
    Temple,      // Повышает счастье населения
    WaterMill,   // Увеличивает производство ресурсов
    Wall,        // Защита города
    Workshop,    // Улучшает производство предметов
    CrystalMine, // Производит кристаллы
}

impl BuildingType {
    pub fn display_name(&self) -> &str {
        match self {
            BuildingType::Residential => "Жилой дом",
            BuildingType::Farm => "Ферма",
            BuildingType::LumberMill => "Лесопилка",
            BuildingType::Mine => "Шахта",
            BuildingType::Market => "Рынок",
            BuildingType::Barracks => "Казармы",
            BuildingType::PowerPlant => "Электростанция",
            BuildingType::Laboratory => "Лаборатория",
            BuildingType::Temple => "Храм",
            BuildingType::WaterMill => "Водяная мельница",
            BuildingType::Wall => "Стена",
            BuildingType::Workshop => "Мастерская",
            BuildingType::CrystalMine => "Кристальная шахта",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            BuildingType::Residential => "Увеличивает максимальное население города",
            BuildingType::Farm => "Производит еду для населения",
            BuildingType::LumberMill => "Добывает дерево из окрестных лесов",
            BuildingType::Mine => "Добывает камень и железо из недр земли",
            BuildingType::Market => "Увеличивает доход золота в городе",
            BuildingType::Barracks => "Позволяет тренировать военные отряды",
            BuildingType::PowerPlant => "Вырабатывает энергию для города",
            BuildingType::Laboratory => "Открывает новые технологии",
            BuildingType::Temple => "Повышает счастье и мораль населения",
            BuildingType::WaterMill => "Увеличивает общую продуктивность",
            BuildingType::Wall => "Защищает город от нападений",
            BuildingType::Workshop => "Улучшает производство и ремесло",
            BuildingType::CrystalMine => "Добывает редкие магические кристаллы",
        }
    }

    pub fn base_cost(&self) -> Vec<(ResourceType, u32)> {
        match self {
            BuildingType::Residential => vec![(ResourceType::Wood, 50), (ResourceType::Stone, 30)],
            BuildingType::Farm => vec![(ResourceType::Wood, 30), (ResourceType::Gold, 20)],
            BuildingType::LumberMill => vec![
                (ResourceType::Wood, 20),
                (ResourceType::Stone, 50),
                (ResourceType::Gold, 30),
            ],
            BuildingType::Mine => vec![
                (ResourceType::Wood, 40),
                (ResourceType::Stone, 20),
                (ResourceType::Gold, 50),
            ],
            BuildingType::Market => vec![
                (ResourceType::Wood, 60),
                (ResourceType::Stone, 40),
                (ResourceType::Gold, 100),
            ],
            BuildingType::Barracks => vec![
                (ResourceType::Wood, 80),
                (ResourceType::Stone, 100),
                (ResourceType::Iron, 50),
            ],
            BuildingType::PowerPlant => vec![
                (ResourceType::Stone, 150),
                (ResourceType::Iron, 80),
                (ResourceType::Gold, 200),
            ],
            BuildingType::Laboratory => vec![
                (ResourceType::Stone, 100),
                (ResourceType::Crystal, 30),
                (ResourceType::Gold, 250),
            ],
            BuildingType::Temple => vec![
                (ResourceType::Stone, 200),
                (ResourceType::Wood, 100),
                (ResourceType::Gold, 150),
                (ResourceType::Crystal, 20),
            ],
            BuildingType::WaterMill => vec![
                (ResourceType::Wood, 120),
                (ResourceType::Stone, 80),
                (ResourceType::Gold, 100),
            ],
            BuildingType::Wall => vec![(ResourceType::Stone, 300), (ResourceType::Iron, 100)],
            BuildingType::Workshop => vec![
                (ResourceType::Wood, 150),
                (ResourceType::Stone, 100),
                (ResourceType::Iron, 50),
                (ResourceType::Gold, 120),
            ],
            BuildingType::CrystalMine => vec![
                (ResourceType::Stone, 200),
                (ResourceType::Iron, 150),
                (ResourceType::Gold, 300),
            ],
        }
    }

    pub fn production_effect(&self, level: u32) -> Vec<(ResourceType, i32)> {
        let base_effect = match self {
            BuildingType::Residential => vec![(ResourceType::Population, 10 + level as i32 * 5)],
            BuildingType::Farm => vec![(ResourceType::Food, 10 + level as i32 * 3)],
            BuildingType::LumberMill => vec![(ResourceType::Wood, 8 + level as i32 * 2)],
            BuildingType::Mine => vec![
                (ResourceType::Stone, 5 + level as i32),
                (ResourceType::Iron, 2 + (level as i32) / 2),
            ],
            BuildingType::Market => vec![(ResourceType::Gold, 15 + level as i32 * 5)],
            BuildingType::Barracks => vec![
                (ResourceType::Gold, -(10 + level as i32 * 2)),
                (ResourceType::Food, -(5 + level as i32)),
            ],
            BuildingType::PowerPlant => vec![(ResourceType::Energy, 20 + level as i32 * 10)],
            BuildingType::Laboratory => vec![
                (ResourceType::Gold, -(20 + level as i32 * 5)),
                (ResourceType::Energy, -(5 + level as i32 * 2)),
            ],
            BuildingType::Temple => vec![(ResourceType::Gold, -(10 + level as i32 * 3))],
            BuildingType::WaterMill => vec![
                (ResourceType::Food, 5 + level as i32),
                (ResourceType::Wood, 5 + level as i32),
            ],
            BuildingType::Wall => vec![],
            BuildingType::Workshop => vec![
                (ResourceType::Gold, 10 + level as i32 * 3),
                (ResourceType::Energy, -(3 + level as i32)),
            ],
            BuildingType::CrystalMine => vec![
                (ResourceType::Crystal, 1 + level as i32 / 3),
                (ResourceType::Energy, -(10 + level as i32 * 2)),
            ],
        };
        base_effect
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub id: String,
    pub name: String,
    pub building_type: BuildingType,
    pub level: u32,
    pub position: (i32, i32), // Координаты на карте города
}

impl Building {
    pub fn new(
        id: String,
        name: String,
        building_type: BuildingType,
        position: (i32, i32),
    ) -> Building {
        Building {
            id,
            name,
            building_type,
            level: 1,
            position,
        }
    }

    pub fn upgrade(&mut self) {
        self.level += 1;
    }

    pub fn get_info(&self) -> String {
        format!(
            "{} ({}), Уровень: {}\nТип: {}\nОписание: {}",
            self.name,
            self.id,
            self.level,
            self.building_type.display_name(),
            self.building_type.description()
        )
    }

    pub fn upgrade_cost(&self) -> Vec<(ResourceType, u32)> {
        let base_costs = self.building_type.base_cost();
        let multiplier = (1.5f32).powi(self.level as i32);

        base_costs
            .iter()
            .map(|(rt, amount)| (rt.clone(), (*amount as f32 * multiplier) as u32))
            .collect()
    }

    pub fn production_effect(&self) -> Vec<(ResourceType, i32)> {
        self.building_type.production_effect(self.level)
    }

    pub fn apply_production_to_resources(&self, resources: &mut Resource) {
        for (resource, amount) in self.production_effect() {
            if amount > 0 {
                resources.add(&resource, amount as u32);
            } else if amount < 0 {
                let _ = resources.subtract(&resource, (-amount) as u32);
            }
        }
    }
}
