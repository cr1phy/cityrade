pub trait Effect {
    /// Выполняет эффект, изменяя состояние целевой сущности.
    /// Например, карта, сектор или ресурсы.
    fn apply(&self, target: &mut EffectTarget);
}

/// Возможные цели для эффекта (например, здания, ресурсы, население, карта).
#[derive(Debug)]
pub enum EffectTarget {
    Population { current: u64, max: u64 },
    Resources { gold: i64, food: i64, wood: i64 },
    Happiness { value: i64 },
    CityInfrastructure { level: u32 },
}

// /// Эффект, увеличивающий количество населения.
// pub struct IncreasePopulation {
//     pub amount: u64,
// }

// impl Effect for IncreasePopulation {
//     fn apply(&self, target: &mut EffectTarget) {
//         if let EffectTarget::Population { current, max } = target {
//             *current = (*current + self.amount).min(*max);
//         }
//     }
// }

// /// Эффект, снижающий уровень счастья.
// pub struct DecreaseHappiness {
//     pub amount: i64,
// }

// impl Effect for DecreaseHappiness {
//     fn apply(&self, target: &mut EffectTarget) {
//         if let EffectTarget::Happiness { value } = target {
//             *value -= self.amount;
//         }
//     }
// }

// /// Эффект для добавления ресурсов.
// pub struct AddResources {
//     pub gold: i64,
//     pub food: i64,
//     pub wood: i64,
// }

// impl Effect for AddResources {
//     fn apply(&self, target: &mut EffectTarget) {
//         if let EffectTarget::Resources { gold, food, wood } = target {
//             *gold += self.gold;
//             *food += self.food;
//             *wood += self.wood;
//         }
//     }
// }
