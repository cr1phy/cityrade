pub mod account;
pub mod building;
pub mod chat;
pub mod city;
pub mod commands;
pub mod diplomacy;
pub mod events;
pub mod faction;
pub mod generator;
pub mod item;
pub mod market;
<<<<<<< HEAD
pub mod player;
=======
>>>>>>> d7ffaf0 (initial)
pub mod plugin;
pub mod population;
pub mod quest;
pub mod random_events;
pub mod resources;
pub mod technology;
pub mod world;
<<<<<<< HEAD
pub mod achievements;

// Модуль prelude для удобного импорта часто используемых типов
pub mod prelude {
    // Ресурсы
    pub use crate::resources::{ResourceType, BuildingType, BuildingEffect};
    
    // События
    pub use crate::events::{Event, EventSystem, EventPriority, EventResult};
    
    // Дипломатия
    pub use crate::diplomacy::{
        RelationType, DiplomaticAction, DiplomaticRelation, 
        DiplomacyManager, JointProject, FactionAction
    };
    
    // Квесты
    pub use crate::quest::{Quest, QuestType, QuestReward, QuestStatus};
    
    // Плагины
    pub use crate::plugin::Plugin;
    
    // Команды
    pub use crate::commands::{Command, CommandSender};
    
    // Достижения
    pub use crate::achievements::{AchievementSystem, AchievementTrigger};
    
    // Утилиты
    pub use std::collections::{HashMap, HashSet};
    pub use serde::{Serialize, Deserialize};
    pub use uuid::Uuid;
}

// Глобальные константы
pub mod constants {
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const MAX_PLAYERS_PER_WORLD: usize = 100;
    pub const DEFAULT_WORLD_SIZE: (u32, u32) = (1000, 1000);
    pub const TICK_RATE_MS: u64 = 50;
    pub const MAX_BUILDING_LEVEL: u32 = 5;
    pub const MAX_TECHNOLOGY_LEVEL: u32 = 10;
}

// Трейт для типов с ID
pub trait HasId {
    fn id(&self) -> &str;
}

// Расширение функциональности для Vec<T>
pub trait VecExt<T> {
    fn find_by_id(&self, id: &str) -> Option<&T> where T: HasId;
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut T> where T: HasId;
}

impl<T> VecExt<T> for Vec<T> {
    fn find_by_id(&self, id: &str) -> Option<&T> where T: HasId {
        self.iter().find(|item| item.id() == id)
    }
    
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut T> where T: HasId {
        self.iter_mut().find(|item| item.id() == id)
    }
}
=======
>>>>>>> d7ffaf0 (initial)

#[cfg(test)]
mod tests;
