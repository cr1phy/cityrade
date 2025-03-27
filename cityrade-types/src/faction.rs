// cityrade-types/src/faction.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::resources::ResourceType;

/// Специализации фракций, определяющие их основные бонусы
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionSpecialization {
    /// Фокус на торговле и коммерции
    Trade,
    /// Фокус на промышленности и добыче ресурсов
    Industry,
    /// Фокус на сельском хозяйстве и экологии
    Agriculture,
    /// Фокус на науке и исследованиях
    Technology,
    /// Фокус на военной мощи
    Military,
    /// Без явной специализации
    Balanced,
}

impl Default for FactionSpecialization {
    fn default() -> Self {
        FactionSpecialization::Balanced
    }
}

/// Бонус или штраф для определенного аспекта игры
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactionBonus {
    /// Модификатор производства ресурса (процентное изменение)
    ResourceProduction(ResourceType, i32),
    /// Модификатор стоимости строительства (процентное изменение)
    BuildingCost(i32),
    /// Модификатор скорости исследований (процентное изменение)
    ResearchSpeed(i32),
    /// Модификатор торговых сделок (процентное изменение)
    TradeDeals(i32),
    /// Модификатор дипломатических отношений (абсолютное изменение)
    DiplomaticInfluence(i32),
    /// Модификатор скорости строительства (процентное изменение)
    BuildingSpeed(i32),
    /// Модификатор для военной мощи (процентное изменение)
    MilitaryStrength(i32),
    /// Модификатор для прироста населения (процентное изменение)
    PopulationGrowth(i32),
    /// Доступ к уникальным зданиям
    UniqueBuildings(Vec<String>),
    /// Доступ к уникальным технологиям
    UniqueTechnologies(Vec<String>),
}

/// Представляет фракцию в игре
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub id: String,
    pub name: String,
    pub description: String,
    pub specialization: FactionSpecialization,
    pub bonuses: Vec<FactionBonus>,
    pub colors: (u8, u8, u8), // RGB представление цветов фракции
    pub emblem: String,      // Путь к файлу с эмблемой или символьное представление
    pub cities: Vec<String>, // ID городов, принадлежащих фракции
    pub is_player_faction: bool, // Является ли фракция игрока
}

impl Faction {
    pub fn new(id: String, name: String, specialization: FactionSpecialization) -> Self {
        let description = match specialization {
            FactionSpecialization::Trade => "Торговый альянс, специализирующийся на коммерции и дипломатии",
            FactionSpecialization::Industry => "Промышленная гильдия, специализирующаяся на производстве и добыче ресурсов",
            FactionSpecialization::Agriculture => "Орден натуралистов, специализирующийся на сельском хозяйстве и экологии",
            FactionSpecialization::Technology => "Технократы, специализирующиеся на высоких технологиях и инновациях",
            FactionSpecialization::Military => "Военная коалиция, специализирующаяся на обороне и производстве оружия",
            FactionSpecialization::Balanced => "Без явной специализации, сбалансированная фракция",
        }.to_string();

        let default_bonuses = match specialization {
            FactionSpecialization::Trade => vec![
                FactionBonus::TradeDeals(15),
                FactionBonus::DiplomaticInfluence(10),
                FactionBonus::MilitaryStrength(-10),
            ],
            FactionSpecialization::Industry => vec![
                FactionBonus::ResourceProduction(ResourceType::Iron, 20),
                FactionBonus::ResourceProduction(ResourceType::Stone, 20),
                FactionBonus::BuildingSpeed(15),
                FactionBonus::PopulationGrowth(-10),
            ],
            FactionSpecialization::Agriculture => vec![
                FactionBonus::ResourceProduction(ResourceType::Food, 25),
                FactionBonus::PopulationGrowth(15),
                FactionBonus::MilitaryStrength(-15),
            ],
            FactionSpecialization::Technology => vec![
                FactionBonus::ResearchSpeed(20),
                FactionBonus::BuildingCost(-10),
                FactionBonus::ResourceProduction(ResourceType::Crystal, 15),
            ],
            FactionSpecialization::Military => vec![
                FactionBonus::MilitaryStrength(25),
                FactionBonus::ResourceProduction(ResourceType::Iron, 10),
                FactionBonus::DiplomaticInfluence(-15),
            ],
            FactionSpecialization::Balanced => vec![
                FactionBonus::ResourceProduction(ResourceType::Gold, 5),
                FactionBonus::ResourceProduction(ResourceType::Food, 5),
                FactionBonus::ResearchSpeed(5),
                FactionBonus::BuildingSpeed(5),
            ],
        };

        let colors = match specialization {
            FactionSpecialization::Trade => (218, 165, 32),      // Gold
            FactionSpecialization::Industry => (139, 69, 19),    // Brown
            FactionSpecialization::Agriculture => (0, 128, 0),   // Green
            FactionSpecialization::Technology => (65, 105, 225), // Royal Blue
            FactionSpecialization::Military => (128, 0, 0),      // Maroon
            FactionSpecialization::Balanced => (128, 128, 128),  // Gray
        };

        Faction {
            id,
            name,
            description,
            specialization,
            bonuses: default_bonuses,
            colors,
            emblem: String::new(),
            cities: Vec::new(),
            is_player_faction: false,
        }
    }

    /// Получает модификатор производства для конкретного ресурса
    pub fn get_resource_production_modifier(&self, resource_type: &ResourceType) -> i32 {
        self.bonuses
            .iter()
            .filter_map(|bonus| match bonus {
                FactionBonus::ResourceProduction(rt, modifier) if rt == resource_type => Some(*modifier),
                _ => None,
            })
            .sum()
    }

    /// Получает модификатор для указанного типа бонуса
    pub fn get_modifier_for_type<T>(&self, filter_fn: T) -> i32
    where
        T: Fn(&FactionBonus) -> Option<i32>,
    {
        self.bonuses
            .iter()
            .filter_map(filter_fn)
            .sum()
    }

    /// Проверяет, имеет ли фракция доступ к указанному уникальному зданию
    pub fn has_unique_building(&self, building_id: &str) -> bool {
        self.bonuses
            .iter()
            .any(|bonus| match bonus {
                FactionBonus::UniqueBuildings(buildings) => buildings.contains(&building_id.to_string()),
                _ => false,
            })
    }

    /// Проверяет, имеет ли фракция доступ к указанной уникальной технологии
    pub fn has_unique_technology(&self, tech_id: &str) -> bool {
        self.bonuses
            .iter()
            .any(|bonus| match bonus {
                FactionBonus::UniqueTechnologies(techs) => techs.contains(&tech_id.to_string()),
                _ => false,
            })
    }
}

/// Управляет всеми фракциями в игре
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FactionManager {
    factions: HashMap<String, Faction>,
    player_faction_id: Option<String>,
}

impl FactionManager {
    pub fn new() -> Self {
        FactionManager {
            factions: HashMap::new(),
            player_faction_id: None,
        }
    }

    /// Создает и добавляет стандартные фракции для игры
    pub fn create_default_factions(&mut self) {
        let trade_faction = Faction::new(
            "trade_alliance".to_string(),
            "Торговый Альянс".to_string(),
            FactionSpecialization::Trade,
        );
        
        let industry_faction = Faction::new(
            "industrial_guild".to_string(),
            "Промышленная Гильдия".to_string(),
            FactionSpecialization::Industry,
        );
        
        let agriculture_faction = Faction::new(
            "naturalist_order".to_string(),
            "Орден Натуралистов".to_string(),
            FactionSpecialization::Agriculture,
        );
        
        let tech_faction = Faction::new(
            "technocrats".to_string(),
            "Технократы".to_string(),
            FactionSpecialization::Technology,
        );
        
        let military_faction = Faction::new(
            "military_coalition".to_string(),
            "Военная Коалиция".to_string(),
            FactionSpecialization::Military,
        );

        self.add_faction(trade_faction);
        self.add_faction(industry_faction);
        self.add_faction(agriculture_faction);
        self.add_faction(tech_faction);
        self.add_faction(military_faction);
    }

    /// Добавляет фракцию в менеджер
    pub fn add_faction(&mut self, faction: Faction) {
        let id = faction.id.clone();
        
        // Если это первая фракция игрока, устанавливаем её как фракцию игрока
        if faction.is_player_faction && self.player_faction_id.is_none() {
            self.player_faction_id = Some(id.clone());
        }
        
        self.factions.insert(id, faction);
    }

    /// Получает фракцию по её ID
    pub fn get_faction(&self, id: &str) -> Option<&Faction> {
        self.factions.get(id)
    }

    /// Получает мутабельную ссылку на фракцию
    pub fn get_faction_mut(&mut self, id: &str) -> Option<&mut Faction> {
        self.factions.get_mut(id)
    }

    /// Получает все фракции
    pub fn get_all_factions(&self) -> Vec<&Faction> {
        self.factions.values().collect()
    }

    /// Устанавливает фракцию игрока
    pub fn set_player_faction(&mut self, faction_id: &str) -> bool {
        if let Some(faction) = self.factions.get_mut(faction_id) {
            faction.is_player_faction = true;
            self.player_faction_id = Some(faction_id.to_string());
            
            // Сбрасываем флаг player_faction для других фракций
            for (id, f) in self.factions.iter_mut() {
                if id != faction_id {
                    f.is_player_faction = false;
                }
            }
            true
        } else {
            false
        }
    }

    /// Получает фракцию игрока
    pub fn get_player_faction(&self) -> Option<&Faction> {
        self.player_faction_id.as_ref().and_then(|id| self.get_faction(id))
    }
} 