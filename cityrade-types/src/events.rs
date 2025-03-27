use serde::{Serialize, Deserialize};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use super::resources::{ResourceType, BuildingEffect};

/// Трейт для событий
pub trait Event: Any + Debug + Send + Sync {
    /// Получить имя события
    fn name(&self) -> &str;
    
    /// Получить "отменяемость" события
    /// Если событие отменяемое, то обработчик может его отменить
    fn is_cancellable(&self) -> bool {
        false
    }
    
    /// Получить идентификатор типа события
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
    
    /// Преобразовать событие в Any для даункастинга
    fn as_any(&self) -> &dyn Any;
}

/// Результат обработки события
pub enum EventResult {
    /// Событие обработано успешно, продолжить выполнение
    Continue,
    /// Событие отменено
    Cancel,
}

/// Приоритет обработчика события
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    Lowest = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Highest = 4,
    Monitor = 5,
}

/// Тип обработчика событий
pub type EventHandlerFn = Box<dyn Fn(&dyn Event) -> EventResult + Send + Sync>;

/// Обработчик событий
pub struct EventHandler {
    /// Идентификатор обработчика
    pub id: String,
    /// Приоритет обработчика
    pub priority: EventPriority,
    /// Игнорировать ли отмененные события
    pub ignore_cancelled: bool,
    /// Функция обработки события
    pub handler: EventHandlerFn,
}

/// Система событий
pub struct EventSystem {
    /// Карта обработчиков по типам событий
    handlers: HashMap<TypeId, Vec<EventHandler>>,
}

impl EventSystem {
    /// Создать новую систему событий
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    
    /// Зарегистрировать обработчик для указанного типа события
    pub fn register_handler<E: Event + 'static>(
        &mut self,
        id: String,
        priority: EventPriority,
        ignore_cancelled: bool,
        handler: impl Fn(&E) -> EventResult + Send + Sync + 'static,
    ) {
        let type_id = TypeId::of::<E>();
        let handlers = self.handlers.entry(type_id).or_insert_with(Vec::new);
        
        // Создаем обработчик, который преобразует dyn Event в конкретный тип события
        let handler_fn: EventHandlerFn = Box::new(move |event: &dyn Event| {
            if let Some(e) = event.as_any().downcast_ref::<E>() {
                handler(e)
            } else {
                EventResult::Continue
            }
        });
        
        handlers.push(EventHandler {
            id,
            priority,
            ignore_cancelled,
            handler: handler_fn,
        });
        
        // Сортируем обработчики по приоритету
        handlers.sort_by(|a, b| a.priority.cmp(&b.priority));
    }
    
    /// Удалить обработчик по ID
    pub fn unregister_handler(&mut self, id: &str) {
        for handlers in self.handlers.values_mut() {
            handlers.retain(|handler| handler.id != id);
        }
    }
    
    /// Вызвать событие и выполнить все зарегистрированные обработчики
    pub fn call_event(&self, event: &dyn Event) -> bool {
        let type_id = TypeId::of::<dyn Event>();
        let mut cancelled = false;
        
        if let Some(handlers) = self.handlers.get(&type_id) {
            for handler in handlers {
                if cancelled && handler.ignore_cancelled {
                    continue;
                }
                
                match (handler.handler)(event) {
                    EventResult::Continue => {},
                    EventResult::Cancel => {
                        if event.is_cancellable() {
                            cancelled = true;
                        }
                    }
                }
            }
        }
        
        !cancelled
    }
}

/// Событие подключения игрока
#[derive(Debug)]
pub struct PlayerJoinEvent {
    /// ID игрока
    pub player_id: String,
    /// ID мира
    pub world_id: String,
    /// Отменено ли событие
    cancelled: bool,
}

impl PlayerJoinEvent {
    /// Создать новое событие подключения игрока
    pub fn new(player_id: String, world_id: String) -> Self {
        Self {
            player_id,
            world_id,
            cancelled: false,
        }
    }
    
    /// Отменить событие
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }
    
    /// Проверить, отменено ли событие
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }
}

impl Event for PlayerJoinEvent {
    fn name(&self) -> &str {
        "player_join"
    }
    
    fn is_cancellable(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Событие отключения игрока
#[derive(Debug)]
pub struct PlayerLeaveEvent {
    /// ID игрока
    pub player_id: String,
    /// ID мира
    pub world_id: String,
}

impl PlayerLeaveEvent {
    /// Создать новое событие отключения игрока
    pub fn new(player_id: String, world_id: String) -> Self {
        Self {
            player_id,
            world_id,
        }
    }
}

impl Event for PlayerLeaveEvent {
    fn name(&self) -> &str {
        "player_leave"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Событие строительства здания
#[derive(Debug)]
pub struct BuildingConstructedEvent {
    /// ID здания
    pub building_id: String,
    /// ID игрока, построившего здание
    pub player_id: String,
    /// ID мира
    pub world_id: String,
    /// Позиция здания
    pub position: (i32, i32),
    /// Отменено ли событие
    cancelled: bool,
}

impl BuildingConstructedEvent {
    /// Создать новое событие строительства здания
    pub fn new(building_id: String, player_id: String, world_id: String, position: (i32, i32)) -> Self {
        Self {
            building_id,
            player_id,
            world_id,
            position,
            cancelled: false,
        }
    }
    
    /// Отменить событие
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }
    
    /// Проверить, отменено ли событие
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }
}

impl Event for BuildingConstructedEvent {
    fn name(&self) -> &str {
        "building_constructed"
    }
    
    fn is_cancellable(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventChain {
    pub current_event: RandomEvent,
    pub possible_outcomes: Vec<(EventOutcome, f32)>,
    pub timer: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventOutcome {
    ResourceEffect(ResourceType, i32),
    NewEvent(RandomEvent),
    BuildingEffect(String, BuildingEffect),
    DiplomaticShift(String, i32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomEvent {
    pub id: String,
    pub description: String,
    pub effects: Vec<EventOutcome>,
} 
