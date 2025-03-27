use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
<<<<<<< HEAD
use super::resources::{ResourceType, BuildingType};
=======
>>>>>>> d7ffaf0 (initial)

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TechnologyType {
    // Экономические
    Agriculture,
    Mining,
    Forestry,
    Trade,
    Banking,
    AdvancedTrading,
    MarketAnalysis,

    // Строительные
    BasicConstruction,
    AdvancedConstruction,
    StoneWorks,
    Architecture,
    CityPlanning,

    // Военные
    BasicMilitary,
    AdvancedMilitary,
    Fortification,
    Siege,
    Tactics,

    // Социальные
    Education,
    Culture,
    Administration,
    Governance,
    Diplomacy,

    // Промышленные
    BasicIndustry,
    Machinery,
    Engineering,
    Automation,
}

// Новая структура - категория технологий
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TechnologyCategory {
    Economic,
    Construction,
    Military,
    Social,
    Industrial,
}

// Новая структура - эффект технологии
#[derive(Debug, Clone, Serialize, Deserialize)]
<<<<<<< HEAD
pub struct TechnologyEffectData {
=======
pub struct TechnologyEffect {
>>>>>>> d7ffaf0 (initial)
    pub description: String,          // Описание эффекта
    pub resource_bonus: Option<(String, f32)>, // (тип_ресурса, процент_бонуса)
    pub building_unlock: Option<String>, // Разблокируемое здание
    pub cost_reduction: Option<(String, f32)>, // (тип_затрат, процент_снижения)
    pub other_bonuses: HashMap<String, f32>, // Другие бонусы (имя_бонуса, значение)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technology {
    pub tech_type: TechnologyType,
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub research_time: u32,           // Новое поле: время исследования в ходах
    pub prerequisites: Vec<TechnologyType>,
    pub category: TechnologyCategory, // Новое поле: категория технологии
    pub era: u32,                     // Новое поле: эра технологии
<<<<<<< HEAD
    pub unlock_effects: Vec<TechnologyEffectData>,
=======
    pub unlock_effects: Vec<TechnologyEffect>, // Изменено: теперь используем структуру эффектов вместо строк
>>>>>>> d7ffaf0 (initial)
}

// Новая структура - статус исследования технологии
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResearchStatus {
    NotStarted,
    InProgress(u32),  // Текущий прогресс исследования
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnologyTree {
<<<<<<< HEAD
    pub technologies: HashMap<TechnologyType, Technology>,
    pub research_status: HashMap<TechnologyType, ResearchStatus>,
    pub research_focus: Option<TechnologyType>,
    pub completed_technologies: HashSet<TechnologyType>,
    pub research_points: u32,
    pub research_rate: u32,
    pub tech_bonuses: HashMap<String, f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnologyNode {
    pub requirements: Vec<String>,
    pub research_cost: HashMap<ResourceType, u32>,
    pub effects: Vec<TechnologyEffectData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TechnologyEffect {
    ProductionBoost(ResourceType, f32),
    UnlockBuilding(BuildingType),
    DiplomaticCapacity(u32),
    TradeRouteSlot(u32),
=======
    technologies: HashMap<TechnologyType, Technology>,
    research_status: HashMap<TechnologyType, ResearchStatus>, // Новое поле: статус исследования для каждой технологии
    research_focus: Option<TechnologyType>,                  // Новое поле: текущий фокус исследования
    completed_technologies: HashSet<TechnologyType>,         // Новое поле: множество завершенных технологий
    research_points: u32,                                    // Новое поле: накопленные очки исследования
    research_rate: u32,                                      // Новое поле: скорость получения очков исследования за ход
    tech_bonuses: HashMap<String, f32>,                      // Новое поле: бонусы от технологий
>>>>>>> d7ffaf0 (initial)
}

impl TechnologyTree {
    pub fn new() -> Self {
<<<<<<< HEAD
        TechnologyTree {
            technologies: Self::default_technologies(),
            research_status: HashMap::new(),
=======
        let technologies = Self::default_technologies();
        let mut research_status = HashMap::new();
        
        // Инициализируем статус исследования для всех технологий
        for tech_type in technologies.keys() {
            research_status.insert(tech_type.clone(), ResearchStatus::NotStarted);
        }
        
        TechnologyTree {
            technologies,
            research_status,
>>>>>>> d7ffaf0 (initial)
            research_focus: None,
            completed_technologies: HashSet::new(),
            research_points: 0,
            research_rate: 10,
            tech_bonuses: HashMap::new(),
        }
    }
    
    // Создаем набор стандартных технологий
    fn default_technologies() -> HashMap<TechnologyType, Technology> {
        let mut technologies = HashMap::new();

        // Базовые технологии (без предпосылок)
        technologies.insert(TechnologyType::Agriculture, Technology {
            tech_type: TechnologyType::Agriculture,
            name: "Сельское хозяйство".to_string(),
            description: "Улучшает производство пищи для населения".to_string(),
            cost: 100,
            research_time: 5,
            prerequisites: Vec::new(),
            category: TechnologyCategory::Economic,
            era: 1,
            unlock_effects: vec![
<<<<<<< HEAD
                TechnologyEffectData {
=======
                TechnologyEffect {
>>>>>>> d7ffaf0 (initial)
                    description: "Увеличивает производство пищи на 20%".to_string(),
                    resource_bonus: Some(("Food".to_string(), 0.2)),
                    building_unlock: Some("Farm".to_string()),
                    cost_reduction: None,
                    other_bonuses: HashMap::new(),
                }
            ],
        });

        technologies.insert(TechnologyType::Mining, Technology {
            tech_type: TechnologyType::Mining,
            name: "Горное дело".to_string(),
            description: "Улучшает добычу камня и других минералов".to_string(),
            cost: 100,
            research_time: 5,
            prerequisites: Vec::new(),
            category: TechnologyCategory::Economic,
            era: 1,
            unlock_effects: vec![
<<<<<<< HEAD
                TechnologyEffectData {
=======
                TechnologyEffect {
>>>>>>> d7ffaf0 (initial)
                    description: "Увеличивает добычу камня на 20%".to_string(),
                    resource_bonus: Some(("Stone".to_string(), 0.2)),
                    building_unlock: Some("Mine".to_string()),
                    cost_reduction: None,
                    other_bonuses: HashMap::new(),
                }
            ],
        });

        technologies.insert(TechnologyType::Forestry, Technology {
            tech_type: TechnologyType::Forestry,
            name: "Лесное хозяйство".to_string(),
            description: "Улучшает заготовку древесины".to_string(),
            cost: 100,
            research_time: 5,
            prerequisites: Vec::new(),
            category: TechnologyCategory::Economic,
            era: 1,
            unlock_effects: vec![
<<<<<<< HEAD
                TechnologyEffectData {
=======
                TechnologyEffect {
>>>>>>> d7ffaf0 (initial)
                    description: "Увеличивает производство дерева на 20%".to_string(),
                    resource_bonus: Some(("Wood".to_string(), 0.2)),
                    building_unlock: Some("Sawmill".to_string()),
                    cost_reduction: None,
                    other_bonuses: HashMap::new(),
                }
            ],
        });

        technologies.insert(TechnologyType::BasicConstruction, Technology {
            tech_type: TechnologyType::BasicConstruction,
            name: "Основы строительства".to_string(),
            description: "Базовые принципы строительства зданий".to_string(),
            cost: 100,
            research_time: 5,
            prerequisites: Vec::new(),
            category: TechnologyCategory::Construction,
            era: 1,
            unlock_effects: vec![
<<<<<<< HEAD
                TechnologyEffectData {
=======
                TechnologyEffect {
>>>>>>> d7ffaf0 (initial)
                    description: "Позволяет строить базовые здания".to_string(),
                    resource_bonus: None,
                    building_unlock: Some("House".to_string()),
                    cost_reduction: Some(("Construction".to_string(), 0.1)),
                    other_bonuses: HashMap::new(),
                }
            ],
        });

        // Технологии второго уровня
        technologies.insert(TechnologyType::Trade, Technology {
            tech_type: TechnologyType::Trade,
            name: "Торговля".to_string(),
            description: "Развивает торговые отношения с другими городами".to_string(),
            cost: 200,
            research_time: 8,
            prerequisites: vec![TechnologyType::Agriculture],
            category: TechnologyCategory::Economic,
            era: 1,
            unlock_effects: vec![
<<<<<<< HEAD
                TechnologyEffectData {
=======
                TechnologyEffect {
>>>>>>> d7ffaf0 (initial)
                    description: "Позволяет строить рынки".to_string(),
                    resource_bonus: None,
                    building_unlock: Some("Market".to_string()),
                    cost_reduction: None,
                    other_bonuses: {
                        let mut map = HashMap::new();
                        map.insert("TradeIncome".to_string(), 0.1);
                        map
                    },
                }
            ],
        });

        technologies.insert(TechnologyType::AdvancedConstruction, Technology {
            tech_type: TechnologyType::AdvancedConstruction,
            name: "Продвинутое строительство".to_string(),
            description: "Усовершенствованные методы строительства".to_string(),
            cost: 200,
            research_time: 10,
            prerequisites: vec![TechnologyType::BasicConstruction, TechnologyType::Mining],
            category: TechnologyCategory::Construction,
            era: 2,
            unlock_effects: vec![
<<<<<<< HEAD
                TechnologyEffectData {
=======
                TechnologyEffect {
>>>>>>> d7ffaf0 (initial)
                    description: "Позволяет строить продвинутые здания".to_string(),
                    resource_bonus: None,
                    building_unlock: Some("Apartment".to_string()),
                    cost_reduction: Some(("Construction".to_string(), 0.15)),
                    other_bonuses: HashMap::new(),
                }
            ],
        });

        // Добавляем новые технологии
        technologies.insert(TechnologyType::Banking, Technology {
            tech_type: TechnologyType::Banking,
            name: "Банковское дело".to_string(),
            description: "Развитие системы финансов и кредитования".to_string(),
            cost: 300,
            research_time: 12,
            prerequisites: vec![TechnologyType::Trade],
            category: TechnologyCategory::Economic,
            era: 2,
            unlock_effects: vec![
<<<<<<< HEAD
                TechnologyEffectData {
=======
                TechnologyEffect {
>>>>>>> d7ffaf0 (initial)
                    description: "Увеличивает доход от золота на 15%".to_string(),
                    resource_bonus: Some(("Gold".to_string(), 0.15)),
                    building_unlock: Some("Bank".to_string()),
                    cost_reduction: None,
                    other_bonuses: {
                        let mut map = HashMap::new();
                        map.insert("InterestRate".to_string(), 0.05);
                        map
                    },
                }
            ],
        });

        technologies.insert(TechnologyType::Education, Technology {
            tech_type: TechnologyType::Education,
            name: "Образование".to_string(),
            description: "Развитие системы обучения и передачи знаний".to_string(),
            cost: 250,
            research_time: 10,
            prerequisites: vec![],
            category: TechnologyCategory::Social,
            era: 2,
            unlock_effects: vec![
<<<<<<< HEAD
                TechnologyEffectData {
=======
                TechnologyEffect {
>>>>>>> d7ffaf0 (initial)
                    description: "Увеличивает скорость исследований на 20%".to_string(),
                    resource_bonus: None,
                    building_unlock: Some("School".to_string()),
                    cost_reduction: None,
                    other_bonuses: {
                        let mut map = HashMap::new();
                        map.insert("ResearchRate".to_string(), 0.2);
                        map
                    },
                }
            ],
        });

        technologies
    }

    pub fn get_all_technologies(&self) -> &HashMap<TechnologyType, Technology> {
        &self.technologies
    }

    pub fn get_technology(&self, tech_type: &TechnologyType) -> Option<&Technology> {
        self.technologies.get(tech_type)
    }
    
    // Получить статус исследования для технологии
    pub fn get_research_status(&self, tech_type: &TechnologyType) -> Option<&ResearchStatus> {
        self.research_status.get(tech_type)
    }
    
    // Проверить, доступна ли технология для исследования
    pub fn is_available_for_research(&self, tech_type: &TechnologyType) -> bool {
        if let Some(tech) = self.technologies.get(tech_type) {
            // Технология доступна, если все её предпосылки исследованы
            tech.prerequisites.iter().all(|prereq| {
                self.research_status.get(prereq) == Some(&ResearchStatus::Completed)
            })
        } else {
            false
        }
    }
    
    // Начать исследование технологии
    pub fn start_research(&mut self, tech_type: TechnologyType) -> Result<(), String> {
        if !self.is_available_for_research(&tech_type) {
            return Err("Не выполнены предпосылки для этой технологии".to_string());
        }
        
        if let Some(status) = self.research_status.get(&tech_type) {
            match status {
                ResearchStatus::Completed => {
                    return Err("Эта технология уже исследована".to_string());
                }
                ResearchStatus::InProgress(_) => {
                    return Err("Исследование этой технологии уже ведётся".to_string());
                }
                _ => {}
            }
        }
        
        // Устанавливаем фокус исследования
        self.research_focus = Some(tech_type.clone());
        
        // Обновляем статус
        self.research_status.insert(tech_type, ResearchStatus::InProgress(0));
        
        Ok(())
    }
    
    // Добавить очки исследования к текущему фокусу
    pub fn add_research_points(&mut self, points: u32) -> Option<TechnologyType> {
        if let Some(tech_type) = &self.research_focus {
            if let Some(tech) = self.technologies.get(tech_type) {
                if let Some(ResearchStatus::InProgress(current_points)) = self.research_status.get_mut(tech_type) {
                    *current_points += points;
                    
                    // Проверяем, завершено ли исследование
                    if *current_points >= tech.cost {
                        let tech_type = tech_type.clone();
                        self.research_status.insert(tech_type.clone(), ResearchStatus::Completed);
                        self.completed_technologies.insert(tech_type.clone());
                        self.research_focus = None;
                        
                        // Применяем эффекты технологии
                        for effect in &tech.unlock_effects {
                            if let Some((resource, bonus)) = &effect.resource_bonus {
                                self.tech_bonuses.insert(format!("Resource_{}", resource), *bonus);
                            }
                            
                            if let Some((cost_type, reduction)) = &effect.cost_reduction {
                                self.tech_bonuses.insert(format!("Cost_{}", cost_type), *reduction);
                            }
                            
                            for (bonus_type, value) in &effect.other_bonuses {
                                self.tech_bonuses.insert(bonus_type.clone(), *value);
                            }
                        }
                        
                        return Some(tech_type.clone());
                    }
                }
            }
        }
        
        None
    }
    
    // Обновить исследования (вызывать каждый ход)
    pub fn update_research(&mut self) -> Option<TechnologyType> {
        self.add_research_points(self.research_rate)
    }
    
    // Получить текущий прогресс исследования
    pub fn get_research_progress(&self) -> Option<(TechnologyType, u32, u32)> {
        if let Some(tech_type) = &self.research_focus {
            if let Some(tech) = self.technologies.get(tech_type) {
                if let Some(ResearchStatus::InProgress(current_points)) = self.research_status.get(tech_type) {
                    return Some((tech_type.clone(), *current_points, tech.cost));
                }
            }
        }
        
        None
    }
    
    // Установить скорость исследования
    pub fn set_research_rate(&mut self, rate: u32) {
        self.research_rate = rate;
    }
    
    // Получить текущий бонус от технологий по имени
    pub fn get_tech_bonus(&self, bonus_name: &str) -> f32 {
        *self.tech_bonuses.get(bonus_name).unwrap_or(&0.0)
    }
    
    // Получить список доступных для исследования технологий
    pub fn get_available_technologies(&self) -> Vec<&Technology> {
        self.technologies.values()
            .filter(|tech| {
                let tech_status = self.research_status.get(&tech.tech_type);
                
                // Технология не исследована и не в процессе
                tech_status != Some(&ResearchStatus::Completed) && 
                tech_status != Some(&ResearchStatus::InProgress(0)) &&
                
                // Все предпосылки выполнены
                tech.prerequisites.iter().all(|prereq| {
                    self.research_status.get(prereq) == Some(&ResearchStatus::Completed)
                })
            })
            .collect()
    }
    
    // Получить список завершенных технологий
    pub fn get_completed_technologies(&self) -> Vec<&Technology> {
        self.technologies.values()
            .filter(|tech| {
                self.completed_technologies.contains(&tech.tech_type)
            })
            .collect()
    }
    
    // Получить бонус к производству ресурса
    pub fn get_resource_production_bonus(&self, resource_name: &str) -> f32 {
        self.get_tech_bonus(&format!("Resource_{}", resource_name))
    }
    
    // Получить бонус к снижению стоимости
    pub fn get_cost_reduction_bonus(&self, cost_type: &str) -> f32 {
        self.get_tech_bonus(&format!("Cost_{}", cost_type))
    }
}
