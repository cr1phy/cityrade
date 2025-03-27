use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::{Rng, rngs::ThreadRng};
use rand_distr::weighted::WeightedIndex;
use crate::resources::{Resource, ResourceType};
use crate::city::City;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventCategory {
    Economic,   // Экономические события
    Climate,    // Климатические события
    Social,     // Социальные события
    Political,  // Политические события
    Random,     // Случайные события
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventSeverity {
    Positive,   // Положительное событие
    Neutral,    // Нейтральное событие
    Minor,      // Незначительное негативное событие
    Major,      // Значительное негативное событие
    Disaster,   // Катастрофическое событие
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEffect {
    pub resource_type: Option<ResourceType>,
    pub amount: i32,                          // Может быть положительным или отрицательным
    pub percent: Option<f32>,                 // Процентное изменение
    pub duration: Option<u32>,               // Длительность эффекта в ходах (None = мгновенное)
    pub description: String,                 // Описание эффекта
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomEvent {
    pub event_id: String,
    pub name: String,
    pub description: String,
    pub category: EventCategory,
    pub severity: EventSeverity,
    pub effects: Vec<EventEffect>,
    pub weight: u32,                          // Вес (вероятность) события
    pub min_population: Option<u32>,          // Минимальное население для события
    pub required_buildings: Vec<String>,      // Требуемые здания
    pub incompatible_events: Vec<String>,     // Несовместимые события
    pub follow_up_events: Vec<(String, u32)>, // (event_id, chance_percent) для событий-продолжений
}

#[derive(Debug)]
pub struct EventManager {
    events: HashMap<String, RandomEvent>,
    active_events: Vec<(String, u32)>, // (event_id, remaining_turns)
    rng: ThreadRng,
}

impl EventManager {
    pub fn new() -> Self {
        let events = Self::default_events();
        
        EventManager {
            events,
            active_events: Vec::new(),
            rng: rand::rng(),
        }
    }
    
    // Создаем набор стандартных событий
    fn default_events() -> HashMap<String, RandomEvent> {
        let mut events = HashMap::new();
        
        // Экономические события
        events.insert("market_boom".to_string(), RandomEvent {
            event_id: "market_boom".to_string(),
            name: "Экономический рост".to_string(),
            description: "Рынок переживает внезапный бум, цены на товары растут.".to_string(),
            category: EventCategory::Economic,
            severity: EventSeverity::Positive,
            effects: vec![
                EventEffect {
                    resource_type: Some(ResourceType::Gold),
                    amount: 50,
                    percent: Some(0.1),
                    duration: Some(3),
                    description: "Увеличение золота на 10% в течение 3 ходов".to_string(),
                }
            ],
            weight: 20,
            min_population: Some(50),
            required_buildings: vec!["Market".to_string()],
            incompatible_events: vec!["market_crash".to_string()],
            follow_up_events: Vec::new(),
        });
        
        events.insert("market_crash".to_string(), RandomEvent {
            event_id: "market_crash".to_string(),
            name: "Экономический кризис".to_string(),
            description: "Рынок переживает внезапный обвал, цены на товары падают.".to_string(),
            category: EventCategory::Economic,
            severity: EventSeverity::Major,
            effects: vec![
                EventEffect {
                    resource_type: Some(ResourceType::Gold),
                    amount: -30,
                    percent: Some(-0.15),
                    duration: Some(4),
                    description: "Уменьшение золота на 15% в течение 4 ходов".to_string(),
                }
            ],
            weight: 10,
            min_population: Some(100),
            required_buildings: vec!["Market".to_string()],
            incompatible_events: vec!["market_boom".to_string()],
            follow_up_events: Vec::new(),
        });
        
        // Климатические события
        events.insert("good_harvest".to_string(), RandomEvent {
            event_id: "good_harvest".to_string(),
            name: "Отличный урожай".to_string(),
            description: "Благоприятные погодные условия привели к отличному урожаю.".to_string(),
            category: EventCategory::Climate,
            severity: EventSeverity::Positive,
            effects: vec![
                EventEffect {
                    resource_type: Some(ResourceType::Food),
                    amount: 100,
                    percent: Some(0.2),
                    duration: Some(2),
                    description: "Увеличение производства еды на 20% в течение 2 ходов".to_string(),
                }
            ],
            weight: 20,
            min_population: None,
            required_buildings: vec!["Farm".to_string()],
            incompatible_events: vec!["drought".to_string()],
            follow_up_events: Vec::new(),
        });
        
        events.insert("drought".to_string(), RandomEvent {
            event_id: "drought".to_string(),
            name: "Засуха".to_string(),
            description: "Продолжительное отсутствие осадков привело к засухе.".to_string(),
            category: EventCategory::Climate,
            severity: EventSeverity::Major,
            effects: vec![
                EventEffect {
                    resource_type: Some(ResourceType::Food),
                    amount: -50,
                    percent: Some(-0.3),
                    duration: Some(4),
                    description: "Уменьшение производства еды на 30% в течение 4 ходов".to_string(),
                },
                EventEffect {
                    resource_type: Some(ResourceType::Energy),
                    amount: -30,
                    percent: Some(-0.2),
                    duration: Some(5),
                    description: "Уменьшение запасов энергии на 20% в течение 5 ходов".to_string(),
                }
            ],
            weight: 15,
            min_population: None,
            required_buildings: Vec::new(),
            incompatible_events: vec!["good_harvest".to_string()],
            follow_up_events: vec![("famine".to_string(), 30)],
        });
        
        // Социальные события
        events.insert("festival".to_string(), RandomEvent {
            event_id: "festival".to_string(),
            name: "Городской фестиваль".to_string(),
            description: "Жители организовали фестиваль, поднимающий настроение всему городу.".to_string(),
            category: EventCategory::Social,
            severity: EventSeverity::Positive,
            effects: vec![
                EventEffect {
                    resource_type: Some(ResourceType::Population),
                    amount: 5,
                    percent: Some(0.05),
                    duration: Some(1),
                    description: "Увеличение прироста населения на 5% в течение 1 хода".to_string(),
                },
                EventEffect {
                    resource_type: Some(ResourceType::Gold),
                    amount: -20,
                    percent: None,
                    duration: None,
                    description: "Единовременные расходы на организацию фестиваля".to_string(),
                }
            ],
            weight: 25,
            min_population: Some(30),
            required_buildings: Vec::new(),
            incompatible_events: Vec::new(),
            follow_up_events: Vec::new(),
        });
        
        events.insert("plague".to_string(), RandomEvent {
            event_id: "plague".to_string(),
            name: "Эпидемия".to_string(),
            description: "В городе началась эпидемия, люди болеют и умирают.".to_string(),
            category: EventCategory::Social,
            severity: EventSeverity::Disaster,
            effects: vec![
                EventEffect {
                    resource_type: Some(ResourceType::Population),
                    amount: -15,
                    percent: Some(-0.1),
                    duration: Some(6),
                    description: "Уменьшение населения на 10% в течение 6 ходов".to_string(),
                }
            ],
            weight: 5,
            min_population: Some(100),
            required_buildings: Vec::new(),
            incompatible_events: Vec::new(),
            follow_up_events: Vec::new(),
        });
        
        events
    }
    
    // Добавить новое событие
    pub fn add_event(&mut self, event: RandomEvent) {
        self.events.insert(event.event_id.clone(), event);
    }
    
    // Получить все события
    pub fn get_all_events(&self) -> &HashMap<String, RandomEvent> {
        &self.events
    }
    
    // Получить конкретное событие
    pub fn get_event(&self, event_id: &str) -> Option<&RandomEvent> {
        self.events.get(event_id)
    }
    
    // Сгенерировать случайное событие с учетом текущего состояния города
    pub fn generate_event(&mut self, city: &City) -> Option<&RandomEvent> {
        let population = city.population;
        
        // Фильтруем события, которые могут произойти
        let eligible_events: Vec<&RandomEvent> = self.events.values()
            .filter(|event| {
                // Проверка минимального населения
                if let Some(min_pop) = event.min_population {
                    if population < min_pop {
                        return false;
                    }
                }
                
                // Проверка требуемых зданий
                for building_name in &event.required_buildings {
                    if !city.buildings.values().any(|b| &b.name == building_name) {
                        return false;
                    }
                }
                
                // Проверка несовместимых событий
                for active_event_id in &self.active_events {
                    if event.incompatible_events.contains(&active_event_id.0) {
                        return false;
                    }
                }
                
                true
            })
            .collect();
        
        if eligible_events.is_empty() {
            return None;
        }
        
        // Создаем взвешенное распределение
        let weights: Vec<u32> = eligible_events.iter()
            .map(|event| event.weight)
            .collect();
        
        let dist = WeightedIndex::new(&weights).ok()?;
        
        // Выбираем случайное событие
        let chosen_index = self.rng.sample(dist);
        let chosen_event = eligible_events[chosen_index];
        
        // Добавляем событие в активные, если у него есть длительные эффекты
        let has_duration = chosen_event.effects.iter()
            .any(|effect| effect.duration.is_some() && effect.duration.unwrap() > 0);
            
        if has_duration {
            let max_duration = chosen_event.effects.iter()
                .filter_map(|effect| effect.duration)
                .max()
                .unwrap_or(0);
                
            self.active_events.push((chosen_event.event_id.clone(), max_duration));
        }
        
        // Проверка на события-продолжения
        for (follow_up_id, chance) in &chosen_event.follow_up_events {
            if self.rng.random_range(1..=100) <= *chance {
                if let Some(follow_up_event) = self.events.get(follow_up_id) {
                    println!("Запланировано событие-продолжение: {}", follow_up_event.name);
                    // Логика планирования событий-продолжений
                }
            }
        }
        
        Some(chosen_event)
    }
    
    // Применить эффекты события к ресурсам
    pub fn apply_event_effects(&self, event: &RandomEvent, resources: &mut Resource) {
        for effect in &event.effects {
            if let Some(resource_type) = &effect.resource_type {
                let current_amount = resources.get(resource_type);
                
                // Применение абсолютного изменения
                if effect.amount != 0 {
                    let new_amount = (current_amount as i32 + effect.amount).max(0) as u32;
                    resources.set(resource_type.clone(), new_amount);
                }
                
                // Применение процентного изменения
                if let Some(percent) = effect.percent {
                    let change = (current_amount as f32 * percent).round() as i32;
                    let new_amount = (current_amount as i32 + change).max(0) as u32;
                    resources.set(resource_type.clone(), new_amount);
                }
            }
        }
    }
    
    // Обновить состояние активных событий (вызывать в конце каждого хода)
    pub fn update_active_events(&mut self, resources: &mut Resource) {
        // Обновляем счетчики активных событий
        let mut i = 0;
        while i < self.active_events.len() {
            let (event_id, turns_left) = &mut self.active_events[i];
            
            if *turns_left <= 1 {
                // Событие закончилось
                self.active_events.remove(i);
            } else {
                // Событие продолжается
                *turns_left -= 1;
                
                // Применяем продолжающиеся эффекты
                if let Some(event) = self.events.get(event_id) {
                    for effect in &event.effects {
                        if let Some(duration) = effect.duration {
                            if duration > 0 {
                                // Применяем только продолжающиеся эффекты
                                if let Some(resource_type) = &effect.resource_type {
                                    let current_amount = resources.get(resource_type);
                                    
                                    // Применение процентного изменения
                                    if let Some(percent) = effect.percent {
                                        let change = (current_amount as f32 * percent).round() as i32;
                                        let new_amount = (current_amount as i32 + change).max(0) as u32;
                                        resources.set(resource_type.clone(), new_amount);
                                    }
                                }
                            }
                        }
                    }
                }
                
                i += 1;
            }
        }
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
} 