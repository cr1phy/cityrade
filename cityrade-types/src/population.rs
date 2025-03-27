use crate::resources::ResourceType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum PopulationClass {
    Peasant,  // Крестьяне - производят пищу
    Worker,   // Рабочие - работают в производстве
    Merchant, // Торговцы - улучшают экономику
    Soldier,  // Солдаты - защищают город
    Scholar,  // Ученые - способствуют исследованиям
    Noble,    // Знать - увеличивает престиж и налоги
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    pub classes: HashMap<PopulationClass, u32>,
    pub happiness: f32,        // от 0.0 до 1.0
    pub growth_rate: f32,      // базовый прирост населения
    pub food_consumption: f32, // сколько пищи потребляет каждая единица населения в день
    pub next_growth_tick: u32, // через сколько тиков произойдет рост
}

impl Population {
    pub fn new() -> Self {
        let mut classes = HashMap::new();
        classes.insert(PopulationClass::Peasant, 20);
        classes.insert(PopulationClass::Worker, 10);
        classes.insert(PopulationClass::Merchant, 3);
        classes.insert(PopulationClass::Soldier, 5);
        classes.insert(PopulationClass::Scholar, 1);
        classes.insert(PopulationClass::Noble, 1);

        Population {
            classes,
            happiness: 0.7,
            growth_rate: 0.02,
            food_consumption: 0.5,
            next_growth_tick: 10,
        }
    }

    pub fn total(&self) -> u32 {
        self.classes.values().sum()
    }

    pub fn daily_food_consumption(&self) -> u32 {
        (self.total() as f32 * self.food_consumption) as u32
    }

    pub fn update(&mut self, food_available: u32, housing_capacity: u32) {
        // Расчет потребности в пище
        let food_needed = self.daily_food_consumption();

        // Обновление счастья на основе доступности пищи
        if food_available < food_needed {
            let food_ratio = food_available as f32 / food_needed as f32;
            self.happiness -= (1.0 - food_ratio) * 0.1;
        } else {
            self.happiness += 0.01;
        }

        // Ограничиваем счастье в диапазоне 0.0-1.0
        self.happiness = self.happiness.max(0.0).min(1.0);

        // Обновление роста населения
        if self.next_growth_tick > 0 {
            self.next_growth_tick -= 1;
        } else {
            // Рост населения зависит от счастья и наличия места
            let growth_modifier = if self.total() >= housing_capacity {
                0.0 // Нет роста, если нет жилья
            } else {
                self.happiness * 2.0 - 0.5 // Модификатор от -0.5 до 1.5
            };

            let effective_growth_rate = self.growth_rate * growth_modifier;

            if effective_growth_rate > 0.0 {
                // Положительный прирост - население растет
                let growth = (self.total() as f32 * effective_growth_rate) as u32;
                if growth > 0 {
                    // Распределяем новых жителей по классам
                    self.distribute_growth(growth);
                }
            } else if effective_growth_rate < 0.0 {
                // Отрицательный прирост - население уменьшается
                let decline = (self.total() as f32 * effective_growth_rate.abs()) as u32;
                if decline > 0 {
                    self.distribute_decline(decline);
                }
            }

            // Устанавливаем следующий тик роста
            self.next_growth_tick = 10;
        }
    }

    fn distribute_growth(&mut self, growth: u32) {
        // 60% прироста идет в крестьяне, 30% в рабочие, 10% распределяется по остальным
        let peasant_growth = (growth as f32 * 0.6) as u32;
        let worker_growth = (growth as f32 * 0.3) as u32;
        let other_growth = growth - peasant_growth - worker_growth;

        *self.classes.entry(PopulationClass::Peasant).or_insert(0) += peasant_growth;
        *self.classes.entry(PopulationClass::Worker).or_insert(0) += worker_growth;

        // Распределяем остальную часть поровну
        if other_growth > 0 {
            let classes = vec![
                PopulationClass::Merchant,
                PopulationClass::Soldier,
                PopulationClass::Scholar,
                PopulationClass::Noble,
            ];

            let growth_per_class = other_growth / classes.len() as u32;
            let remainder = other_growth % classes.len() as u32;

            for (i, class) in classes.iter().enumerate() {
                let extra = if i < remainder as usize { 1 } else { 0 };
                *self.classes.entry(class.clone()).or_insert(0) += growth_per_class + extra;
            }
        }
    }

    fn distribute_decline(&mut self, decline: u32) {
        // Сначала уменьшаем число крестьян и рабочих
        let mut remaining_decline = decline;

        // Пытаемся уменьшить население пропорционально их количеству
        let total = self.total();
        let mut remaining_population = total;

        for (_class, count) in self.classes.iter_mut() {
            let class_decline = (remaining_decline as f32 * (*count as f32 / total as f32)) as u32;
            let actual_decline = class_decline.min(*count);

            *count -= actual_decline;
            remaining_decline -= actual_decline;
            remaining_population -= actual_decline;

            if remaining_decline == 0 || remaining_population == 0 {
                break;
            }
        }
    }

    pub fn get_production_bonus(&self, resource: &ResourceType) -> f32 {
        match resource {
            ResourceType::Food => 0.5,
            ResourceType::Wood => 0.3,
            ResourceType::Stone => 0.2,
            ResourceType::Iron => 0.4,
            ResourceType::Gold => 0.1,
            ResourceType::Coal => 0.25,
            ResourceType::Tools => 0.35,
            ResourceType::LuxuryGoods => 0.15,
            ResourceType::Crystal => {
                let scholars = *self.classes.get(&PopulationClass::Scholar).unwrap_or(&0) as f32;
                scholars * 3.0 / self.total() as f32
            }
            ResourceType::Population => {
                let nobles = *self.classes.get(&PopulationClass::Noble).unwrap_or(&0) as f32;
                let scholars = *self.classes.get(&PopulationClass::Scholar).unwrap_or(&0) as f32;
                (nobles * 0.5 + scholars * 0.5) / self.total() as f32
            }
            ResourceType::Energy => {
                let workers = *self.classes.get(&PopulationClass::Worker).unwrap_or(&0) as f32;
                let scholars = *self.classes.get(&PopulationClass::Scholar).unwrap_or(&0) as f32;
                (workers * 1.0 + scholars * 1.5) / self.total() as f32
            }
        }
    }
}
