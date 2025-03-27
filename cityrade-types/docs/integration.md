# Руководство по интеграции систем фракций и дипломатии

Это руководство описывает, как интегрировать системы фракций и дипломатии в существующий проект Cityrade.

## Содержание
1. [Интеграция системы фракций](#интеграция-системы-фракций)
2. [Интеграция системы дипломатии](#интеграция-системы-дипломатии)
3. [Взаимодействие между системами](#взаимодействие-между-системами)
4. [Рекомендуемые сценарии использования](#рекомендуемые-сценарии-использования)

## Интеграция системы фракций

### Шаг 1: Инициализация FactionManager

Создайте экземпляр `FactionManager` при запуске игры и добавьте стандартные фракции:

```rust
use cityrade_types::faction::FactionManager;

// В структуре игрового состояния
pub struct GameState {
    // ... существующие поля
    faction_manager: FactionManager,
    // ... другие поля
}

impl GameState {
    pub fn new() -> Self {
        let mut faction_manager = FactionManager::new();
        faction_manager.create_default_factions();
        
        GameState {
            // ... инициализация других полей
            faction_manager,
            // ... другие поля
        }
    }
}
```

### Шаг 2: Выбор фракции при создании игрока

Позвольте игроку выбрать фракцию при создании персонажа:

```rust
impl GameState {
    pub fn select_player_faction(&mut self, faction_id: &str) -> bool {
        self.faction_manager.set_player_faction(faction_id)
    }
    
    pub fn get_available_factions(&self) -> Vec<&Faction> {
        self.faction_manager.get_all_factions()
    }
}
```

### Шаг 3: Применение бонусов фракций

Интегрируйте бонусы фракций в игровую механику:

```rust
impl GameState {
    pub fn calculate_resource_production(&self, resource_type: &ResourceType, base_production: i32) -> i32 {
        let faction_bonus = if let Some(faction) = self.faction_manager.get_player_faction() {
            faction.get_resource_production_modifier(resource_type)
        } else {
            0
        };
        
        // Применяем бонус фракции (процентное изменение)
        base_production + (base_production * faction_bonus / 100)
    }
    
    pub fn calculate_research_speed(&self, base_speed: i32) -> i32 {
        let faction_bonus = if let Some(faction) = self.faction_manager.get_player_faction() {
            faction.get_modifier_for_type(|bonus| match bonus {
                FactionBonus::ResearchSpeed(value) => Some(*value),
                _ => None,
            })
        } else {
            0
        };
        
        // Применяем бонус фракции (процентное изменение)
        base_speed + (base_speed * faction_bonus / 100)
    }
}
```

### Шаг 4: Обновление интерфейса

Адаптируйте пользовательский интерфейс для отображения информации о фракции:

```rust
fn update_ui(&self, ui: &mut Ui) {
    if let Some(faction) = self.faction_manager.get_player_faction() {
        let (r, g, b) = faction.colors;
        let faction_color = Color::Rgb(r, g, b);
        
        ui.add_text_with_color(&format!("Фракция: {}", faction.name), faction_color);
        ui.add_text(&format!("Описание: {}", faction.description));
        
        // Отображение бонусов фракции
        ui.add_text("Бонусы:");
        for bonus in &faction.bonuses {
            match bonus {
                FactionBonus::ResourceProduction(resource_type, value) => {
                    ui.add_text(&format!("Производство {}: {}%", resource_type, value));
                },
                // ... другие типы бонусов
            }
        }
    } else {
        ui.add_text("Фракция не выбрана");
    }
}
```

## Интеграция системы дипломатии

### Шаг 1: Инициализация DiplomacyManager

Добавьте `DiplomacyManager` в структуру игрового состояния:

```rust
use cityrade_types::diplomacy::DiplomacyManager;

pub struct GameState {
    // ... существующие поля
    faction_manager: FactionManager,
    diplomacy_manager: DiplomacyManager,
    // ... другие поля
}

impl GameState {
    pub fn new() -> Self {
        let mut faction_manager = FactionManager::new();
        faction_manager.create_default_factions();
        
        let diplomacy_manager = DiplomacyManager::new();
        
        GameState {
            // ... инициализация других полей
            faction_manager,
            diplomacy_manager,
            // ... другие поля
        }
    }
}
```

### Шаг 2: Инициализация начальных отношений между фракциями

Установите начальные отношения между фракциями при создании игры:

```rust
impl GameState {
    fn initialize_diplomatic_relations(&mut self) {
        // Торговый Альянс имеет хорошие отношения с Орденом Натуралистов
        self.diplomacy_manager.change_reputation("trade_alliance", "naturalist_order", 30);
        
        // Торговый Альянс в напряженных отношениях с Военной Коалицией
        self.diplomacy_manager.change_reputation("trade_alliance", "military_coalition", -40);
        
        // Промышленная Гильдия в плохих отношениях с Орденом Натуралистов
        self.diplomacy_manager.change_reputation("industrial_guild", "naturalist_order", -60);
        
        // Технократы в хороших отношениях с Промышленной Гильдией
        self.diplomacy_manager.change_reputation("technocrats", "industrial_guild", 40);
    }
}
```

### Шаг 3: Интеграция дипломатических действий

Создайте интерфейс для дипломатических действий:

```rust
impl GameState {
    pub fn perform_diplomatic_action(
        &mut self, 
        target_faction: &str, 
        action: DiplomaticAction
    ) -> Result<(), String> {
        let player_faction = self.faction_manager.get_player_faction()
            .ok_or("Фракция игрока не выбрана")?;
        
        let player_faction_id = &player_faction.id;
        let current_time = self.get_game_time();
        
        // Проверка возможности выполнения действия на основе текущих отношений
        if let Some(relation) = self.diplomacy_manager.get_relation(player_faction_id, target_faction) {
            match (action, relation.relation_type) {
                // Ультиматум нельзя выдвинуть союзнику
                (DiplomaticAction::Ultimatum, RelationType::Alliance) => {
                    return Err("Нельзя выдвинуть ультиматум союзнику".to_string());
                },
                // Совместные исследования недоступны при напряженных отношениях или конфликте
                (DiplomaticAction::JointResearch, RelationType::Tense) | 
                (DiplomaticAction::JointResearch, RelationType::Conflict) => {
                    return Err("Совместные исследования недоступны при плохих отношениях".to_string());
                },
                // Торговое соглашение недоступно при конфликте
                (DiplomaticAction::TradeAgreement, RelationType::Conflict) => {
                    return Err("Торговое соглашение недоступно при конфликте".to_string());
                },
                _ => { /* действие разрешено */ }
            }
        }
        
        // Регистрируем действие
        self.diplomacy_manager.register_action(
            player_faction_id,
            target_faction,
            action.clone(),
            current_time
        );
        
        // Изменяем репутацию в зависимости от действия
        let reputation_change = match action {
            DiplomaticAction::TradeAgreement => 15,
            DiplomaticAction::CulturalExchange => 10,
            DiplomaticAction::ResourceGift => 25,
            DiplomaticAction::JointResearch => 20,
            DiplomaticAction::Sanctions => -30,
            DiplomaticAction::Espionage => -25,
            DiplomaticAction::Ultimatum => -50,
        };
        
        self.diplomacy_manager.change_reputation(
            player_faction_id,
            target_faction,
            reputation_change
        );
        
        Ok(())
    }
}
```

### Шаг 4: Применение дипломатических модификаторов в игре

Используйте дипломатические отношения для модификации игровых параметров:

```rust
impl GameState {
    // Расчет стоимости торговли с учетом дипломатических отношений
    pub fn calculate_trade_cost(&self, faction_id: &str, base_cost: i32) -> i32 {
        let player_faction = match self.faction_manager.get_player_faction() {
            Some(f) => f,
            None => return base_cost,
        };
        
        let relation = match self.diplomacy_manager.get_relation(&player_faction.id, faction_id) {
            Some(r) => r,
            None => return base_cost,
        };
        
        // Применяем торговый модификатор
        (base_cost as f32 / relation.trade_modifier) as i32
    }
}
```

## Взаимодействие между системами

### Применение бонусов фракций к дипломатии

```rust
impl GameState {
    pub fn calculate_diplomatic_influence(&self, target_faction: &str, base_influence: i32) -> i32 {
        let player_faction = match self.faction_manager.get_player_faction() {
            Some(f) => f,
            None => return base_influence,
        };
        
        // Получаем дипломатический бонус фракции
        let diplomatic_bonus = player_faction.get_modifier_for_type(|bonus| match bonus {
            FactionBonus::DiplomaticInfluence(value) => Some(*value),
            _ => None,
        });
        
        // Применяем бонус
        base_influence + (base_influence * diplomatic_bonus / 100)
    }
}
```

## Рекомендуемые сценарии использования

### Сценарий 1: Торговля между фракциями

Используя механизмы фракций и дипломатии, можно реализовать систему торговли, где:

1. Стоимость товаров зависит от торгового модификатора фракции (`trade_modifier`).
2. Доступность определенных товаров зависит от типа отношений (`relation_type`).
3. Покупка/продажа товаров влияет на репутацию между фракциями.

### Сценарий 2: Квесты, зависящие от фракций

Создайте квесты, которые:

1. Доступны только игрокам определенной фракции.
2. Требуют определенного уровня отношений с другой фракцией.
3. По завершении влияют на репутацию между фракциями.

### Сценарий 3: Военные действия

Реализуйте возможность военных действий, которые:

1. Доступны только при отношениях типа `Conflict`.
2. Имеют различные бонусы в зависимости от фракции игрока (особенно для фракции с `Military` специализацией).
3. Значительно влияют на репутацию при успехе/неудаче.

### Сценарий 4: Исследования и технологии

Разработайте систему исследований, где:

1. Скорость исследования зависит от бонусов фракции (`ResearchSpeed`).
2. Некоторые технологии доступны только при определенных дипломатических отношениях.
3. Совместные исследования (`JointResearch`) ускоряют процесс исследования и улучшают отношения. 