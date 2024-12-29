use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Различные цели, для которых используются здания.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BuildingPurpose {
    Residential,   // Жилое
    Public,        // Общественное (школа, больница)
    Industrial,    // Промышленное (заводы)
    Agricultural,  // Сельскохозяйственное
    Other(String), // Прочее
}

/// Статусы, показывающие состояние здания.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BuildingStatus {
    Planning,          // Планирование
    UnderConstruction, // В процессе строительства
    InUse,             // В эксплуатации
    NeedsRepair,       // Нуждается в ремонте
    Abandoned,         // Заброшено
    Demolished,        // Снесено
}

/// Структура, представляющая здание.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub id: u64,
    pub name: String,
    pub purpose: BuildingPurpose,
    pub status: BuildingStatus,
    pub size: (u32, u32),
    pub coordinates: Option<(i32, i32)>,
    pub required_resources: Vec<(ResourceType, u32)>,
    pub build_start_time: Option<DateTime<Utc>>,
    pub build_duration: Option<Duration>,
}

impl Building {
    /// Создает новое здание с указанными параметрами.
    pub fn new(
        id: u64,
        name: &str,
        purpose: BuildingPurpose,
        size: (u32, u32),
        required_resources: Vec<(ResourceType, u32)>,
        build_duration: Option<Duration>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            purpose,
            status: BuildingStatus::Planning,
            size,
            coordinates: None,
            required_resources,
            build_start_time: None,
            build_duration,
        }
    }

    /// Устанавливает координаты для здания.
    pub fn set_coordinates(&mut self, x: i32, y: i32) {
        self.coordinates = Some((x, y));
    }

    /// Начинает процесс строительства.
    pub fn start_construction(&mut self) {
        self.status = BuildingStatus::UnderConstruction;
        self.build_start_time = Some(Utc::now());
    }

    /// Проверяет, завершено ли строительство.
    pub fn check_construction_complete(&mut self) -> bool {
        if let (Some(start_time), Some(duration)) = (self.build_start_time, self.build_duration) {
            if Utc::now() >= start_time + duration {
                self.status = BuildingStatus::InUse;
                self.build_start_time = None; // Очищаем после завершения.
                return true;
            }
        }
        false
    }

    /// Добавляет эффект здания (TODO: позже можно определить через трейт `Effect`).
    pub fn apply_effects(&self) {
        match &self.purpose {
            BuildingPurpose::Residential => {
                // Увеличение населения или скорости роста.
            }
            BuildingPurpose::Industrial => {
                // Повышение производства ресурсов.
            }
            _ => {}
        }
    }
}

/// Пример использования типов ресурсов.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Wood,
    Stone,
    Iron,
    Gold,
    Food,
}
