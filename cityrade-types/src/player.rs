use crate::city::City;
use crate::resources::{Resource, ResourceType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Структура игрока
/// 
/// Представляет данные игрока в игровом мире.
/// Отличается от Account тем, что содержит только игровые данные,
/// а не данные для авторизации и профиля.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// Уникальный идентификатор игрока
    pub id: String,
    
    /// Имя игрока (может отличаться от логина аккаунта)
    pub name: String,
    
    /// ID аккаунта, привязанного к игроку
    pub account_id: String,
    
    /// ID текущего мира
    pub world_id: Option<String>,
    
    /// Текущая позиция игрока в мире (x, y)
    pub position: (i32, i32),
    
    /// Список городов, принадлежащих игроку
    pub cities: Vec<City>,
    
    /// Игровые ресурсы игрока
    pub resources: Resource,
    
    /// Уровень игрока
    pub level: u32,
    
    /// Опыт игрока
    pub experience: u64,
    
    /// Достижения игрока (ID достижения -> дата получения)
    pub achievements: HashMap<String, chrono::DateTime<chrono::Utc>>,
    
    /// Метаданные игрока (произвольные данные)
    pub metadata: HashMap<String, String>,
    
    /// Последняя активность игрока
    pub last_activity: chrono::DateTime<chrono::Utc>,
    
    /// Является ли игрок онлайн
    pub online: bool,
}

impl Player {
    /// Создает нового игрока
    pub fn new(name: String, account_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            account_id,
            world_id: None,
            position: (0, 0),
            cities: Vec::new(),
            resources: Resource::new(),
            level: 1,
            experience: 0,
            achievements: HashMap::new(),
            metadata: HashMap::new(),
            last_activity: chrono::Utc::now(),
            online: false,
        }
    }
    
    /// Добавляет опыт игроку и повышает уровень при необходимости
    pub fn add_experience(&mut self, amount: u64) -> bool {
        let old_level = self.level;
        self.experience += amount;
        
        // Формула расчета уровня: level = sqrt(experience / 100)
        let new_level = (self.experience as f64 / 100.0).sqrt() as u32 + 1;
        if new_level > self.level {
            self.level = new_level;
            return true; // Уровень повышен
        }
        
        false // Уровень не изменился
    }
    
    /// Добавляет город игроку
    pub fn add_city(&mut self, city: City) {
        self.cities.push(city);
    }
    
    /// Удаляет город игрока по ID
    pub fn remove_city(&mut self, city_id: &str) -> bool {
        let initial_len = self.cities.len();
        self.cities.retain(|city| city.id != city_id);
        self.cities.len() < initial_len
    }
    
    /// Получает город игрока по ID
    pub fn get_city(&self, city_id: &str) -> Option<&City> {
        self.cities.iter().find(|city| city.id == city_id)
    }
    
    /// Получает изменяемую ссылку на город игрока по ID
    pub fn get_city_mut(&mut self, city_id: &str) -> Option<&mut City> {
        self.cities.iter_mut().find(|city| city.id == city_id)
    }
    
    /// Обновляет позицию игрока
    pub fn update_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
        self.last_activity = chrono::Utc::now();
    }
    
    /// Входит в мир
    pub fn join_world(&mut self, world_id: String, position: (i32, i32)) {
        self.world_id = Some(world_id);
        self.position = position;
        self.online = true;
        self.last_activity = chrono::Utc::now();
    }
    
    /// Выходит из мира
    pub fn leave_world(&mut self) {
        self.online = false;
        self.last_activity = chrono::Utc::now();
    }
    
    /// Добавляет достижение
    pub fn add_achievement(&mut self, achievement_id: String) -> bool {
        if self.achievements.contains_key(&achievement_id) {
            return false; // Достижение уже получено
        }
        
        self.achievements.insert(achievement_id, chrono::Utc::now());
        true // Достижение добавлено
    }
    
    /// Проверяет, есть ли у игрока достижение
    pub fn has_achievement(&self, achievement_id: &str) -> bool {
        self.achievements.contains_key(achievement_id)
    }
    
    /// Устанавливает метаданные
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Получает метаданные
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
    
    /// Удаляет метаданные
    pub fn remove_metadata(&mut self, key: &str) -> bool {
        self.metadata.remove(key).is_some()
    }
    
    /// Возвращает общую ценность всех городов игрока
    pub fn get_total_cities_value(&self) -> u64 {
        self.cities.iter().map(|city| city.resources.get(&ResourceType::Gold) as u64).sum()
    }
    
    /// Проверяет, активен ли игрок (был онлайн в последние N минут)
    pub fn is_active(&self, minutes: i64) -> bool {
        if self.online {
            return true;
        }
        
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(self.last_activity);
        duration.num_minutes() < minutes
    }
}

/// Менеджер игроков
#[derive(Debug)]
pub struct PlayerManager {
    /// Карта игроков (ID игрока -> игрок)
    players: HashMap<String, Player>,
    
    /// Карта аккаунтов (ID аккаунта -> ID игрока)
    account_players: HashMap<String, String>,
}

impl PlayerManager {
    /// Создает новый менеджер игроков
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            account_players: HashMap::new(),
        }
    }
    
    /// Добавляет игрока
    pub fn add_player(&mut self, player: Player) {
        let player_id = player.id.clone();
        let account_id = player.account_id.clone();
        
        self.account_players.insert(account_id, player_id.clone());
        self.players.insert(player_id, player);
    }
    
    /// Удаляет игрока по ID
    pub fn remove_player(&mut self, player_id: &str) -> Option<Player> {
        if let Some(player) = self.players.remove(player_id) {
            self.account_players.remove(&player.account_id);
            Some(player)
        } else {
            None
        }
    }
    
    /// Получает игрока по ID
    pub fn get_player(&self, player_id: &str) -> Option<&Player> {
        self.players.get(player_id)
    }
    
    /// Получает изменяемую ссылку на игрока по ID
    pub fn get_player_mut(&mut self, player_id: &str) -> Option<&mut Player> {
        self.players.get_mut(player_id)
    }
    
    /// Получает игрока по ID аккаунта
    pub fn get_player_by_account(&self, account_id: &str) -> Option<&Player> {
        if let Some(player_id) = self.account_players.get(account_id) {
            self.players.get(player_id)
        } else {
            None
        }
    }
    
    /// Получает изменяемую ссылку на игрока по ID аккаунта
    pub fn get_player_by_account_mut(&mut self, account_id: &str) -> Option<&mut Player> {
        if let Some(player_id) = self.account_players.get(account_id).cloned() {
            self.players.get_mut(&player_id)
        } else {
            None
        }
    }
    
    /// Получает список всех игроков
    pub fn get_all_players(&self) -> Vec<&Player> {
        self.players.values().collect()
    }
    
    /// Получает список всех онлайн игроков
    pub fn get_online_players(&self) -> Vec<&Player> {
        self.players.values().filter(|p| p.online).collect()
    }
    
    /// Получает список всех игроков в указанном мире
    pub fn get_players_in_world(&self, world_id: &str) -> Vec<&Player> {
        self.players.values()
            .filter(|p| p.world_id.as_ref().map_or(false, |id| id == world_id))
            .collect()
    }
    
    /// Обновляет последнюю активность игрока
    pub fn update_activity(&mut self, player_id: &str) -> bool {
        if let Some(player) = self.players.get_mut(player_id) {
            player.last_activity = chrono::Utc::now();
            true
        } else {
            false
        }
    }
    
    /// Возвращает количество онлайн игроков
    pub fn online_count(&self) -> usize {
        self.players.values().filter(|p| p.online).count()
    }
    
    /// Получает топ N игроков по уровню
    pub fn get_top_players_by_level(&self, limit: usize) -> Vec<&Player> {
        let mut players: Vec<&Player> = self.players.values().collect();
        players.sort_by(|a, b| b.level.cmp(&a.level));
        players.truncate(limit);
        players
    }
    
    /// Получает топ N игроков по количеству городов
    pub fn get_top_players_by_cities(&self, limit: usize) -> Vec<&Player> {
        let mut players: Vec<&Player> = self.players.values().collect();
        players.sort_by(|a, b| b.cities.len().cmp(&a.cities.len()));
        players.truncate(limit);
        players
    }
    
    /// Получает топ N игроков по общей ценности городов
    pub fn get_top_players_by_city_value(&self, limit: usize) -> Vec<&Player> {
        let mut players: Vec<&Player> = self.players.values().collect();
        players.sort_by(|a, b| {
            let a_value = a.get_total_cities_value();
            let b_value = b.get_total_cities_value();
            b_value.cmp(&a_value)
        });
        players.truncate(limit);
        players
    }
}

/// Трейт для плагинов, которые хотят работать с игроками
pub trait PlayerPlugin: Send + Sync {
    /// Метод, вызываемый при входе игрока в мир
    fn on_player_join(&mut self, player: &Player, world_id: &str);
    
    /// Метод, вызываемый при выходе игрока из мира
    fn on_player_leave(&mut self, player: &Player);
    
    /// Метод, вызываемый при создании нового города игроком
    fn on_player_create_city(&mut self, player: &Player, city: &City);
    
    /// Метод, вызываемый при повышении уровня игрока
    fn on_player_level_up(&mut self, player: &Player, new_level: u32);
}
