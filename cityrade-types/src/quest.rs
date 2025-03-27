use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
<<<<<<< HEAD
use std::time::Duration;
use crate::resources::{ResourceType, BuildingType};
use crate::diplomacy::FactionAction;
use std::str::FromStr;
=======
>>>>>>> d7ffaf0 (initial)

/// Статус квеста
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
    Available,  // Квест доступен, но еще не начат
}

impl QuestStatus {
    /// Получить цвет для отображения статуса в UI
    pub fn get_color(&self) -> &str {
        match self {
            QuestStatus::NotStarted => "gray",
            QuestStatus::InProgress => "yellow",
            QuestStatus::Completed => "green",
            QuestStatus::Failed => "red",
            QuestStatus::Available => "blue",
        }
    }
    
    /// Проверяет, является ли статус активным (в процессе)
    pub fn is_active(&self) -> bool {
        *self == QuestStatus::InProgress
    }
    
    /// Проверяет, является ли квест завершенным
    pub fn is_completed(&self) -> bool {
        *self == QuestStatus::Completed
    }
    
    /// Проверяет, является ли квест проваленным
    pub fn is_failed(&self) -> bool {
        *self == QuestStatus::Failed
    }
    
    /// Проверяет, доступен ли квест для начала
    pub fn is_available(&self) -> bool {
        *self == QuestStatus::Available || *self == QuestStatus::NotStarted
    }
}

/// Цель квеста
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjective {
    pub id: String,
    pub description: String,
    pub current_value: u32,
    pub target_value: u32,
    pub completed: bool,
}

impl QuestObjective {
    /// Создает новую цель квеста
    pub fn new(id: String, description: String, target_value: u32) -> Self {
        Self {
            id,
            description,
            current_value: 0,
            target_value,
            completed: false,
        }
    }
    
    /// Обновляет прогресс цели
    pub fn update_progress(&mut self, value: u32) {
        self.current_value = value;
        self.completed = self.current_value >= self.target_value;
    }
    
    /// Увеличивает прогресс цели
    pub fn increment_progress(&mut self, amount: u32) {
        self.current_value += amount;
        self.completed = self.current_value >= self.target_value;
    }
    
    /// Возвращает процент выполнения цели
    pub fn get_progress_percentage(&self) -> u8 {
        if self.target_value == 0 {
            return 100;
        }
        
        let percentage = (self.current_value as f32 / self.target_value as f32 * 100.0) as u8;
        percentage.min(100)
    }
}

/// Тип награды
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardType {
    Resource(String, u32),
    Experience(u32),
    Item(String, u32),
    Reputation(String, i32),
    Custom(String),
}

/// Награда за квест
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestReward {
<<<<<<< HEAD
    pub resources: HashMap<ResourceType, u32>,
    pub reputation: HashMap<String, i32>,
    pub unlockables: Vec<String>,
=======
    pub reward_type: RewardType,
    pub description: String,
>>>>>>> d7ffaf0 (initial)
}

impl QuestReward {
    /// Создает новую награду за квест
<<<<<<< HEAD
    pub fn new(resources: HashMap<ResourceType, u32>, reputation: HashMap<String, i32>, unlockables: Vec<String>) -> Self {
        Self {
            resources,
            reputation,
            unlockables,
=======
    pub fn new(reward_type: RewardType, description: &str) -> Self {
        Self {
            reward_type,
            description: description.to_string(),
>>>>>>> d7ffaf0 (initial)
        }
    }
    
    /// Создает награду типа Resource
<<<<<<< HEAD
    pub fn resource(resource_type: &str, amount: u32, _description: &str) -> Self {
        let mut resources = HashMap::new();
        resources.insert(ResourceType::from_str(resource_type).unwrap(), amount);
        Self::new(resources, HashMap::new(), Vec::new())
    }
    
    /// Создает награду типа Experience
    pub fn experience(_amount: u32) -> Self {
        Self::new(HashMap::new(), HashMap::new(), Vec::new())
    }
    
    /// Создает награду типа Item
    pub fn item(item_id: &str, amount: u32, _description: &str) -> Self {
        let mut resources = HashMap::new();
        resources.insert(ResourceType::from_str(item_id).unwrap(), amount);
        Self::new(resources, HashMap::new(), Vec::new())
=======
    pub fn resource(resource_type: &str, amount: u32, description: &str) -> Self {
        Self::new(
            RewardType::Resource(resource_type.to_string(), amount),
            description,
        )
    }
    
    /// Создает награду типа Experience
    pub fn experience(amount: u32) -> Self {
        Self::new(
            RewardType::Experience(amount),
            &format!("{} единиц опыта", amount),
        )
    }
    
    /// Создает награду типа Item
    pub fn item(item_id: &str, amount: u32, description: &str) -> Self {
        Self::new(
            RewardType::Item(item_id.to_string(), amount),
            description,
        )
>>>>>>> d7ffaf0 (initial)
    }
    
    /// Создает награду типа Reputation
    pub fn reputation(faction: &str, amount: i32) -> Self {
<<<<<<< HEAD
        let _direction = if amount >= 0 { "+" } else { "" };
        let mut reputation = HashMap::new();
        reputation.insert(faction.to_string(), amount);
        Self::new(HashMap::new(), reputation, Vec::new())
=======
        let direction = if amount >= 0 { "+" } else { "" };
        Self::new(
            RewardType::Reputation(faction.to_string(), amount),
            &format!("{}{} репутации с фракцией {}", direction, amount, faction),
        )
>>>>>>> d7ffaf0 (initial)
    }
}

/// Категория квеста
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestCategory {
    Main,       // Основные сюжетные квесты
    Side,       // Побочные квесты
    Daily,      // Ежедневные задания
    Weekly,     // Еженедельные задания
    Repeatable, // Повторяемые квесты
    Special,    // Особые квесты (события, праздники)
    Guild,      // Гильдейские квесты
    Trade,      // Торговые квесты
    Faction,    // Квесты фракций
    Hidden,     // Скрытые квесты
}

<<<<<<< HEAD
/// Тип квеста
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestType {
    ResourceCollection(HashMap<ResourceType, u32>),
    BuildingConstruction(BuildingType, u32),
    PopulationGrowth(u32),
    DiplomaticAction(FactionAction),
}

=======
>>>>>>> d7ffaf0 (initial)
/// Основная структура квеста
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
<<<<<<< HEAD
    pub quest_type: QuestType,
    pub reward: QuestReward,
    pub time_limit: Option<Duration>,
    pub objectives: Vec<QuestObjective>,
=======
    pub objectives: Vec<QuestObjective>,
    pub rewards: Vec<QuestReward>,
>>>>>>> d7ffaf0 (initial)
    pub status: QuestStatus,
    pub category: QuestCategory,
    pub level: u32,
    pub min_level: Option<u32>,
    pub max_level: Option<u32>,
    pub expires_at: Option<DateTime<Utc>>,
    pub prerequisites: Vec<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub rewards_claimed: bool,
}

impl Quest {
    /// Создает новый квест
    pub fn new<S: Into<String>>(id: S, title: S, description: S) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
<<<<<<< HEAD
            quest_type: QuestType::ResourceCollection(HashMap::new()),
            reward: QuestReward::new(HashMap::new(), HashMap::new(), Vec::new()),
            time_limit: None,
            objectives: Vec::new(),
=======
            objectives: Vec::new(),
            rewards: Vec::new(),
>>>>>>> d7ffaf0 (initial)
            status: QuestStatus::NotStarted,
            category: QuestCategory::Side,
            level: 1,
            min_level: None,
            max_level: None,
            expires_at: None,
            prerequisites: Vec::new(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            rewards_claimed: false,
        }
    }
    
    /// Добавляет цель к квесту
    pub fn with_objective(mut self, objective: QuestObjective) -> Self {
        self.objectives.push(objective);
        self
    }
    
    /// Добавляет награду к квесту
    pub fn with_reward(mut self, reward: QuestReward) -> Self {
<<<<<<< HEAD
        self.reward = reward;
=======
        self.rewards.push(reward);
>>>>>>> d7ffaf0 (initial)
        self
    }
    
    /// Устанавливает категорию квеста
    pub fn with_category(mut self, category: QuestCategory) -> Self {
        self.category = category;
        self
    }
    
    /// Устанавливает статус квеста
    pub fn with_status(mut self, status: QuestStatus) -> Self {
        self.status = status;
        self
    }
    
    /// Устанавливает уровень квеста
    pub fn with_level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }
    
    /// Устанавливает диапазон уровней квеста
    pub fn with_level_range(mut self, min_level: u32, max_level: u32) -> Self {
        self.min_level = Some(min_level);
        self.max_level = Some(max_level);
        self
    }
    
    /// Устанавливает дату истечения квеста
    pub fn with_expiry(mut self, expires_at: DateTime<Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }
    
    /// Добавляет предпосылку для квеста
    pub fn with_prerequisite<S: Into<String>>(mut self, quest_id: S) -> Self {
        self.prerequisites.push(quest_id.into());
        self
    }
    
    /// Добавляет тег для фильтрации квестов
    pub fn with_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }
    
    /// Начинает квест
    pub fn start(&mut self) -> Result<(), String> {
        if self.status.is_available() {
            self.status = QuestStatus::InProgress;
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err(format!("Нельзя начать квест со статусом {:?}", self.status))
        }
    }
    
    /// Завершает квест
    pub fn complete(&mut self) -> Result<(), String> {
        if self.status == QuestStatus::InProgress {
            if self.all_objectives_completed() {
                self.status = QuestStatus::Completed;
                self.updated_at = Utc::now();
                Ok(())
            } else {
                Err("Не все цели квеста выполнены".to_string())
            }
        } else {
            Err(format!("Нельзя завершить квест со статусом {:?}", self.status))
        }
    }
    
    /// Проваливает квест
    pub fn fail(&mut self) -> Result<(), String> {
        if self.status == QuestStatus::InProgress {
            self.status = QuestStatus::Failed;
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err(format!("Нельзя провалить квест со статусом {:?}", self.status))
        }
    }
    
    /// Сбрасывает статус квеста в начальное состояние
    pub fn reset(&mut self) -> Result<(), String> {
        if self.status == QuestStatus::Completed || self.status == QuestStatus::Failed {
            // Сбрасываем статус и прогресс целей
            self.status = QuestStatus::NotStarted;
            for objective in &mut self.objectives {
                objective.current_value = 0;
                objective.completed = false;
            }
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err(format!("Нельзя сбросить квест со статусом {:?}", self.status))
        }
    }
    
    /// Обновляет прогресс конкретной цели квеста
    pub fn update_objective(&mut self, objective_id: &str, value: u32) -> Result<bool, String> {
        if self.status != QuestStatus::InProgress {
            return Err(format!("Нельзя обновить цель квеста со статусом {:?}", self.status));
        }
        
        let objective = self.objectives
            .iter_mut()
            .find(|o| o.id == objective_id)
            .ok_or_else(|| format!("Цель с id {} не найдена", objective_id))?;
            
        objective.update_progress(value);
        self.updated_at = Utc::now();
        
        let all_completed = self.all_objectives_completed();
        
        Ok(all_completed)
    }
    
    /// Увеличивает прогресс конкретной цели квеста
    pub fn increment_objective(&mut self, objective_id: &str, amount: u32) -> Result<bool, String> {
        if self.status != QuestStatus::InProgress {
            return Err(format!("Нельзя обновить цель квеста со статусом {:?}", self.status));
        }
        
        let objective = self.objectives
            .iter_mut()
            .find(|o| o.id == objective_id)
            .ok_or_else(|| format!("Цель с id {} не найдена", objective_id))?;
            
        objective.increment_progress(amount);
        self.updated_at = Utc::now();
        
        let all_completed = self.all_objectives_completed();
        
        Ok(all_completed)
    }
    
    /// Проверяет, выполнены ли все цели квеста
    pub fn all_objectives_completed(&self) -> bool {
        if self.objectives.is_empty() {
            return false;
        }
        self.objectives.iter().all(|o| o.completed)
    }
    
    /// Возвращает общий прогресс квеста в процентах
    pub fn get_progress_percentage(&self) -> u8 {
        if self.objectives.is_empty() {
            return 0;
        }
        
        let total: u32 = self.objectives.iter()
            .map(|o| o.get_progress_percentage() as u32)
            .sum();
            
        (total / self.objectives.len() as u32) as u8
    }
    
    /// Проверяет, доступен ли квест для игрока указанного уровня
    pub fn is_available_for_level(&self, player_level: u32) -> bool {
        if let Some(min_level) = self.min_level {
            if player_level < min_level {
                return false;
            }
        }
        
        if let Some(max_level) = self.max_level {
            if player_level > max_level {
                return false;
            }
        }
        
        true
    }
    
    /// Проверяет, не истек ли срок выполнения квеста
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }
}

/// Структура для управления квестами
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestManager {
    pub quests: HashMap<String, Quest>,
    pub completed_quests: Vec<String>,
    pub active_quests: Vec<String>,
    pub failed_quests: Vec<String>,
    pub available_quests: Vec<String>,
    pub player_level: u32,
}

impl QuestManager {
    /// Создает нового менеджера квестов
    pub fn new() -> Self {
        Self {
            quests: HashMap::new(),
            completed_quests: Vec::new(),
            active_quests: Vec::new(),
            failed_quests: Vec::new(),
            available_quests: Vec::new(),
            player_level: 1,
        }
    }
    
    /// Добавляет новый квест
    pub fn add_quest(&mut self, quest: Quest) {
        let quest_id = quest.id.clone();
        
        // Обновляем списки в зависимости от статуса
        match quest.status {
            QuestStatus::NotStarted | QuestStatus::Available => {
                self.available_quests.push(quest_id.clone());
            }
            QuestStatus::InProgress => {
                self.active_quests.push(quest_id.clone());
            }
            QuestStatus::Completed => {
                self.completed_quests.push(quest_id.clone());
            }
            QuestStatus::Failed => {
                self.failed_quests.push(quest_id.clone());
            }
        }
        
        self.quests.insert(quest_id, quest);
    }
    
    /// Получает квест по ID
    pub fn get_quest(&self, quest_id: &str) -> Option<&Quest> {
        self.quests.get(quest_id)
    }
    
    /// Получает изменяемую ссылку на квест по ID
    pub fn get_quest_mut(&mut self, quest_id: &str) -> Option<&mut Quest> {
        self.quests.get_mut(quest_id)
    }
    
    /// Начинает квест
    pub fn start_quest(&mut self, quest_id: &str) -> Result<(), String> {
        let quest = self.get_quest_mut(quest_id)
            .ok_or_else(|| format!("Квест с id {} не найден", quest_id))?;
            
        // Проверяем текущий статус
        if !quest.status.is_available() {
            return Err(format!("Квест уже имеет статус {:?}", quest.status));
        }
        
        // Начинаем квест
        quest.start()?;
        
        // Обновляем списки
        self.available_quests.retain(|id| id != quest_id);
        self.active_quests.push(quest_id.to_string());
        
        Ok(())
    }
    
    /// Завершает квест
    pub fn complete_quest(&mut self, id: &str) -> Result<Vec<QuestReward>, String> {
        // Клонируем id, чтобы не было проблем с заимствованием
        let quest_id = id.to_string();
        
        // Получаем квест, пытаемся завершить его и клонируем награды
        let quest_rewards;
        
        // Сначала пытаемся завершить квест и получить награды
        {
            let quest = self.get_quest_mut(&quest_id)
                .ok_or_else(|| format!("Квест с id {} не найден", quest_id))?;
            
            quest.complete()?;
<<<<<<< HEAD
            quest_rewards = quest.reward.clone();
=======
            quest_rewards = quest.rewards.clone();
>>>>>>> d7ffaf0 (initial)
        }
        
        // Обновляем списки
        self.active_quests.retain(|id| id != &quest_id);
        self.completed_quests.push(quest_id);
        
        // Обновляем доступность других квестов, которые могли зависеть от этого
        self.update_available_quests();
        
<<<<<<< HEAD
        Ok(vec![quest_rewards])
=======
        Ok(quest_rewards)
>>>>>>> d7ffaf0 (initial)
    }
    
    /// Проваливает квест
    pub fn fail_quest(&mut self, quest_id: &str) -> Result<(), String> {
        let quest = self.get_quest_mut(quest_id)
            .ok_or_else(|| format!("Квест с id {} не найден", quest_id))?;
            
        // Проверяем текущий статус
        if quest.status != QuestStatus::InProgress {
            return Err(format!("Квест имеет статус {:?} вместо InProgress", quest.status));
        }
        
        // Проваливаем квест
        quest.fail()?;
        
        // Обновляем списки
        self.active_quests.retain(|id| id != quest_id);
        self.failed_quests.push(quest_id.to_string());
        
        Ok(())
    }
    
    /// Обновляет статус всех квестов на основе уровня игрока
    pub fn update_available_quests(&mut self) {
        // Собираем id квестов, которые можно сделать доступными
        let mut quests_to_update = Vec::new();
        
        for (id, quest) in &self.quests {
<<<<<<< HEAD
            if quest.status == QuestStatus::NotStarted 
                && quest.is_available_for_level(self.player_level) 
                && !quest.is_expired() 
            {
                let prerequisites_completed = quest.prerequisites.iter()
                    .all(|prereq_id| {
                        self.quests.get(prereq_id)
                            .map(|q| q.status == QuestStatus::Completed)
                            .unwrap_or(false)
                    });
                    
                if prerequisites_completed {
                    quests_to_update.push(id.clone());
=======
            if quest.status == QuestStatus::NotStarted {
                if quest.is_available_for_level(self.player_level) && !quest.is_expired() {
                    let prerequisites_completed = quest.prerequisites.iter()
                        .all(|prereq_id| {
                            self.quests.get(prereq_id)
                                .map(|q| q.status == QuestStatus::Completed)
                                .unwrap_or(false)
                        });
                        
                    if prerequisites_completed {
                        quests_to_update.push(id.clone());
                    }
>>>>>>> d7ffaf0 (initial)
                }
            }
        }
        
        // Теперь обновляем статусы этих квестов
        for id in quests_to_update {
            if let Some(quest) = self.quests.get_mut(&id) {
                quest.status = QuestStatus::Available;
            }
        }
    }
    
    /// Обновляет прогресс цели квеста
    pub fn update_objective(&mut self, quest_id: &str, objective_id: &str, value: u32) -> Result<bool, String> {
        let quest = self.get_quest_mut(quest_id)
            .ok_or_else(|| format!("Квест с id {} не найден", quest_id))?;
            
        quest.update_objective(objective_id, value)
    }
    
    /// Увеличивает прогресс цели квеста
    pub fn increment_objective(&mut self, quest_id: &str, objective_id: &str, amount: u32) -> Result<bool, String> {
        let quest = self.get_quest_mut(quest_id)
            .ok_or_else(|| format!("Квест с id {} не найден", quest_id))?;
            
        quest.increment_objective(objective_id, amount)
    }
    
    /// Возвращает список всех активных квестов
    pub fn get_active_quests(&self) -> Vec<&Quest> {
        self.active_quests
            .iter()
            .filter_map(|id| self.quests.get(id))
            .collect()
    }
    
    /// Возвращает список всех доступных квестов
    pub fn get_available_quests(&self) -> Vec<&Quest> {
        self.available_quests
            .iter()
            .filter_map(|id| self.quests.get(id))
            .collect()
    }
    
    /// Возвращает список всех завершенных квестов
    pub fn get_completed_quests(&self) -> Vec<&Quest> {
        self.completed_quests
            .iter()
            .filter_map(|id| self.quests.get(id))
            .collect()
    }
    
    /// Возвращает список всех проваленных квестов
    pub fn get_failed_quests(&self) -> Vec<&Quest> {
        self.failed_quests
            .iter()
            .filter_map(|id| self.quests.get(id))
            .collect()
    }
    
    /// Возвращает список квестов по категории
    pub fn get_quests_by_category(&self, category: &QuestCategory) -> Vec<&Quest> {
        self.quests
            .values()
            .filter(|q| q.category == *category)
            .collect()
    }
    
    /// Возвращает список квестов по тегу
    pub fn get_quests_by_tag(&self, tag: &str) -> Vec<&Quest> {
        self.quests
            .values()
            .filter(|q| q.tags.contains(&tag.to_string()))
            .collect()
    }
}

/// Структура для хранения прогресса выполнения достижений
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub unlocked: bool,
    pub unlock_date: Option<DateTime<Utc>>,
    pub icon: String,
    pub points: u32,
    pub hidden: bool,
    pub progress: Option<(u32, u32)>, // (текущее, целевое)
}

impl Achievement {
    /// Создает новое достижение
    pub fn new<S: Into<String>>(id: S, title: S, description: S, points: u32) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            unlocked: false,
            unlock_date: None,
            icon: "default_achievement".to_string(),
            points,
            hidden: false,
            progress: None,
        }
    }
    
    /// Устанавливает иконку достижения
    pub fn with_icon<S: Into<String>>(mut self, icon: S) -> Self {
        self.icon = icon.into();
        self
    }
    
    /// Устанавливает флаг скрытого достижения
    pub fn set_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
    
    /// Устанавливает прогресс достижения
    pub fn with_progress(mut self, current: u32, target: u32) -> Self {
        self.progress = Some((current, target));
        self
    }
    
    /// Разблокирует достижение
    pub fn unlock(&mut self) -> bool {
        if !self.unlocked {
            self.unlocked = true;
            self.unlock_date = Some(Utc::now());
            true
        } else {
            false
        }
    }
    
    /// Обновляет прогресс достижения
    pub fn update_progress(&mut self, current: u32) -> bool {
        if self.unlocked {
            return false;
        }
        
        if let Some((_, target)) = self.progress {
            self.progress = Some((current, target));
            
            // Проверяем, достигли ли мы цели
            if current >= target {
                return self.unlock();
            }
        }
        
        false
    }
    
    /// Увеличивает прогресс достижения
    pub fn increment_progress(&mut self, amount: u32) -> bool {
        if self.unlocked {
            return false;
        }
        
        if let Some((current, target)) = self.progress {
            let new_current = current + amount;
            self.progress = Some((new_current, target));
            
            // Проверяем, достигли ли мы цели
            if new_current >= target {
                return self.unlock();
            }
        }
        
        false
    }
    
    /// Возвращает процент выполнения достижения
    pub fn get_progress_percentage(&self) -> u8 {
        if self.unlocked {
            return 100;
        }
        
        if let Some((current, target)) = self.progress {
            if target == 0 {
                return 100;
            }
            
            let percentage = (current as f32 / target as f32 * 100.0) as u8;
            percentage.min(100)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quest_status_methods() {
        // Тест для метода is_failed
        let status_failed = QuestStatus::Failed;
        let status_completed = QuestStatus::Completed;
        let status_in_progress = QuestStatus::InProgress;
<<<<<<< HEAD
        let status_available = QuestStatus::Available;
        let status_not_started = QuestStatus::NotStarted;
        
        // Проверка is_failed
        assert!(status_failed.is_failed());
        assert!(!status_completed.is_failed());
        assert!(!status_in_progress.is_failed());
        assert!(!status_available.is_failed());
        assert!(!status_not_started.is_failed());
        
        // Проверка is_completed
        assert!(!status_failed.is_completed());
        assert!(status_completed.is_completed());
        assert!(!status_in_progress.is_completed());
        assert!(!status_available.is_completed());
        assert!(!status_not_started.is_completed());
        
        // Проверка is_active
        assert!(!status_failed.is_active());
        assert!(!status_completed.is_active());
        assert!(status_in_progress.is_active());
        assert!(!status_available.is_active());
        assert!(!status_not_started.is_active());
        
        // Проверка is_available
        assert!(!status_failed.is_available());
        assert!(!status_completed.is_available());
        assert!(!status_in_progress.is_available());
        assert!(status_available.is_available());
        assert!(status_not_started.is_available());
        
        // Проверка get_color
        assert_eq!(status_failed.get_color(), "red");
        assert_eq!(status_completed.get_color(), "green");
        assert_eq!(status_in_progress.get_color(), "yellow");
        assert_eq!(status_available.get_color(), "blue");
        assert_eq!(status_not_started.get_color(), "gray");
=======
        
        assert!(status_failed.is_failed());
        assert!(!status_completed.is_failed());
        assert!(!status_in_progress.is_failed());
        
        // Тест для метода is_completed
        assert!(!status_failed.is_completed());
        assert!(status_completed.is_completed());
        assert!(!status_in_progress.is_completed());
        
        // Тест для метода is_active
        assert!(!status_failed.is_active());
        assert!(!status_completed.is_active());
        assert!(status_in_progress.is_active());
        
        // Тест для метода is_available
        assert!(!status_failed.is_available());
        assert!(!status_completed.is_available());
        assert!(!status_in_progress.is_available());
        assert!(QuestStatus::Available.is_available());
        assert!(QuestStatus::NotStarted.is_available());
>>>>>>> d7ffaf0 (initial)
    }
} 