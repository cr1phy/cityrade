use serde::{Serialize, Deserialize};
use std::collections::{HashSet, HashMap};
use super::resources::{ResourceType, BuildingType};
use super::diplomacy::RelationType;

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementSystem {
    pub unlocked: HashSet<String>,
    pub progress: HashMap<String, f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AchievementTrigger {
    ResourceThreshold(ResourceType, u32),
    BuildingCount(BuildingType, u32),
    PopulationCount(u32),
    DiplomaticRelation(String, RelationType),
} 