// cityrade-types/src/diplomacy.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
<<<<<<< HEAD
use super::resources::ResourceType;

/// Типы дипломатических отношений между фракциями
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    Neutral,
    Friendly,
    Tense,
    Conflict,
    Alliance,
    TradePartner,
=======

/// Типы дипломатических отношений между фракциями
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    /// Полное сотрудничество, общая оборона, торговля без пошлин
    Alliance,
    /// Дружеские отношения, сниженные торговые пошлины
    Friendly,
    /// Стандартные отношения без бонусов и штрафов
    Neutral,
    /// Повышенные пошлины, ограничения на передвижение
    Tense,
    /// Торговое эмбарго, возможность военных действий
    Conflict,
>>>>>>> d7ffaf0 (initial)
}

impl Default for RelationType {
    fn default() -> Self {
        RelationType::Neutral
    }
}

/// Представляет дипломатические действия, которые могут быть выполнены между фракциями
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiplomaticAction {
    /// Улучшает экономические отношения и снижает пошлины
    TradeAgreement,
    /// Повышает репутацию и влияние во фракции
    CulturalExchange,
    /// Быстрое улучшение отношений
    ResourceGift,
    /// Ускоряет развитие технологий
    JointResearch,
    /// Накладывает экономические ограничения
    Sanctions,
    /// Получение информации о технологиях и ресурсах другой фракции
    Espionage,
    /// Требование с угрозой конфликта
    Ultimatum,
}

/// Дипломатические отношения между двумя фракциями
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaticRelation {
    pub relation_type: RelationType,
    pub reputation: i32,       // от -100 до 100
    pub trade_modifier: f32,   // множитель торговых сделок
    pub treaties: Vec<String>, // активные соглашения
    pub last_actions: Vec<(DiplomaticAction, u64)>, // действие и время действия
}

impl DiplomaticRelation {
    pub fn new() -> Self {
        DiplomaticRelation {
            relation_type: RelationType::Neutral,
            reputation: 0,
            trade_modifier: 1.0,
            treaties: Vec::new(),
            last_actions: Vec::new(),
        }
    }

    /// Обновляет отношения на основе репутации
    pub fn update_relation_type(&mut self) {
        self.relation_type = match self.reputation {
            i if i >= 75 => RelationType::Alliance,
<<<<<<< HEAD
            i if i >= 25 => RelationType::TradePartner,
            i if i > -25 => RelationType::Neutral,
            i if i > -75 => RelationType::Conflict,
=======
            i if i >= 25 => RelationType::Friendly,
            i if i > -25 => RelationType::Neutral,
            i if i > -75 => RelationType::Tense,
>>>>>>> d7ffaf0 (initial)
            _ => RelationType::Conflict,
        };

        // Обновляем торговый модификатор в зависимости от отношений
        self.trade_modifier = match self.relation_type {
            RelationType::Alliance => 1.5,
<<<<<<< HEAD
            RelationType::TradePartner => 1.2,
            RelationType::Friendly => 1.1,
=======
            RelationType::Friendly => 1.2,
>>>>>>> d7ffaf0 (initial)
            RelationType::Neutral => 1.0,
            RelationType::Tense => 0.8,
            RelationType::Conflict => 0.0, // Эмбарго - торговля запрещена
        };
    }

    /// Изменяет репутацию и обновляет отношения
    pub fn change_reputation(&mut self, amount: i32) {
        self.reputation = (self.reputation + amount).clamp(-100, 100);
        self.update_relation_type();
    }

    /// Добавляет дипломатическое действие в историю
    pub fn add_action(&mut self, action: DiplomaticAction, time: u64) {
        self.last_actions.push((action, time));
        
        // Ограничиваем историю последними 10 действиями
        if self.last_actions.len() > 10 {
            self.last_actions.remove(0);
        }
    }
}

/// Управляет всеми дипломатическими отношениями между фракциями
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiplomacyManager {
    // Карта отношений: (faction_id_1, faction_id_2) -> DiplomaticRelation
    relations: HashMap<(String, String), DiplomaticRelation>,
}

impl DiplomacyManager {
    pub fn new() -> Self {
        DiplomacyManager {
            relations: HashMap::new(),
        }
    }

    /// Получает отношения между двумя фракциями
    pub fn get_relation(&self, faction1: &str, faction2: &str) -> Option<&DiplomaticRelation> {
        // Сортируем ключи для обеспечения консистентного доступа
        let key = if faction1 < faction2 {
            (faction1.to_string(), faction2.to_string())
        } else {
            (faction2.to_string(), faction1.to_string())
        };
        
        self.relations.get(&key)
    }

    /// Устанавливает отношения между двумя фракциями
    pub fn set_relation(&mut self, faction1: &str, faction2: &str, relation: DiplomaticRelation) {
        // Сортируем ключи для обеспечения консистентного доступа
        let key = if faction1 < faction2 {
            (faction1.to_string(), faction2.to_string())
        } else {
            (faction2.to_string(), faction1.to_string())
        };
        
        self.relations.insert(key, relation);
    }

    /// Изменяет репутацию между двумя фракциями
    pub fn change_reputation(&mut self, faction1: &str, faction2: &str, amount: i32) {
        let key = if faction1 < faction2 {
            (faction1.to_string(), faction2.to_string())
        } else {
            (faction2.to_string(), faction1.to_string())
        };
        
        self.relations
            .entry(key)
            .or_insert_with(DiplomaticRelation::new)
            .change_reputation(amount);
    }

    /// Регистрирует дипломатическое действие
    pub fn register_action(&mut self, faction1: &str, faction2: &str, action: DiplomaticAction, time: u64) {
        let key = if faction1 < faction2 {
            (faction1.to_string(), faction2.to_string())
        } else {
            (faction2.to_string(), faction1.to_string())
        };
        
        self.relations
            .entry(key)
            .or_insert_with(DiplomaticRelation::new)
            .add_action(action, time);
    }
<<<<<<< HEAD
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JointProject {
    TradeRoute {
        factions: Vec<String>,
        resource: ResourceType,
        income_split: Vec<f32>,
    },
    ResearchPartnership {
        technology: String,
        cost_sharing: HashMap<String, f32>,
    },
    MilitaryAlliance {
        mutual_defense: bool,
        shared_intel: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionAction {
    TradeAgreement,
    PeaceTreaty,
    AllianceProposal,
    WarDeclaration,
=======
>>>>>>> d7ffaf0 (initial)
} 