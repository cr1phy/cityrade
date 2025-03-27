# Документация модуля Faction

Модуль `faction` предоставляет типы и структуры для создания, управления и взаимодействия с игровыми фракциями в Cityrade.

## Типы данных

### FactionSpecialization

Перечисление, определяющее специализацию фракции, которая влияет на её базовые бонусы.

```rust
pub enum FactionSpecialization {
    Trade,        // Фокус на торговле и коммерции
    Industry,     // Фокус на промышленности и добыче ресурсов
    Agriculture,  // Фокус на сельском хозяйстве и экологии
    Technology,   // Фокус на науке и исследованиях
    Military,     // Фокус на военной мощи
    Balanced,     // Без явной специализации (по умолчанию)
}
```

### FactionBonus

Перечисление, представляющее различные бонусы и штрафы, которые может иметь фракция.

```rust
pub enum FactionBonus {
    ResourceProduction(ResourceType, i32),   // Модификатор производства конкретного ресурса
    BuildingCost(i32),                       // Модификатор стоимости строительства
    ResearchSpeed(i32),                      // Модификатор скорости исследований
    TradeDeals(i32),                         // Модификатор торговых сделок
    DiplomaticInfluence(i32),                // Модификатор дипломатических отношений
    BuildingSpeed(i32),                      // Модификатор скорости строительства
    MilitaryStrength(i32),                   // Модификатор для военной мощи
    PopulationGrowth(i32),                   // Модификатор для прироста населения
    UniqueBuildings(Vec<String>),            // Доступ к уникальным зданиям
    UniqueTechnologies(Vec<String>),         // Доступ к уникальным технологиям
}
```

### Faction

Структура, представляющая фракцию в игре.

```rust
pub struct Faction {
    pub id: String,               // Уникальный идентификатор фракции
    pub name: String,             // Название фракции
    pub description: String,      // Описание фракции
    pub specialization: FactionSpecialization,  // Специализация фракции
    pub bonuses: Vec<FactionBonus>,  // Список бонусов фракции
    pub colors: (u8, u8, u8),     // RGB представление цветов фракции
    pub emblem: String,           // Путь к файлу с эмблемой
    pub cities: Vec<String>,      // ID городов, принадлежащих фракции
    pub is_player_faction: bool,  // Является ли фракция игрока
}
```

#### Методы Faction

- `new(id: String, name: String, specialization: FactionSpecialization) -> Self` - Создаёт новую фракцию с указанными параметрами.
- `get_resource_production_modifier(&self, resource_type: &ResourceType) -> i32` - Возвращает суммарный модификатор производства конкретного ресурса.
- `get_modifier_for_type<T>(&self, filter_fn: T) -> i32` - Возвращает суммарный модификатор для указанного типа бонуса.
- `has_unique_building(&self, building_id: &str) -> bool` - Проверяет, имеет ли фракция доступ к указанному уникальному зданию.
- `has_unique_technology(&self, tech_id: &str) -> bool` - Проверяет, имеет ли фракция доступ к указанной уникальной технологии.

### FactionManager

Структура для управления всеми фракциями в игре.

```rust
pub struct FactionManager {
    factions: HashMap<String, Faction>,
    player_faction_id: Option<String>,
}
```

#### Методы FactionManager

- `new() -> Self` - Создает новый менеджер фракций.
- `create_default_factions(&mut self)` - Создает и добавляет стандартные фракции (Торговый Альянс, Промышленная Гильдия, Орден Натуралистов, Технократы, Военная Коалиция).
- `add_faction(&mut self, faction: Faction)` - Добавляет фракцию в менеджер.
- `get_faction(&self, id: &str) -> Option<&Faction>` - Получает фракцию по её ID.
- `get_faction_mut(&mut self, id: &str) -> Option<&mut Faction>` - Получает мутабельную ссылку на фракцию.
- `get_all_factions(&self) -> Vec<&Faction>` - Получает все фракции.
- `set_player_faction(&mut self, faction_id: &str) -> bool` - Устанавливает фракцию игрока.
- `get_player_faction(&self) -> Option<&Faction>` - Получает фракцию игрока.

## Примеры использования

### Создание фракции

```rust
use cityrade_types::faction::{Faction, FactionSpecialization};

// Создаем новую фракцию с указанными параметрами
let mut custom_faction = Faction::new(
    "custom_faction_id".to_string(),
    "Моя Фракция".to_string(),
    FactionSpecialization::Technology,
);

// Проверим, какие бонусы получила фракция
let research_bonus = custom_faction.get_modifier_for_type(|bonus| match bonus {
    FactionBonus::ResearchSpeed(value) => Some(*value),
    _ => None,
});
println!("Бонус к исследованиям: {}%", research_bonus); // Выведет: Бонус к исследованиям: 20%
```

### Работа с менеджером фракций

```rust
use cityrade_types::faction::FactionManager;

// Создаем менеджер фракций
let mut manager = FactionManager::new();

// Создаем стандартные фракции
manager.create_default_factions();

// Получаем список всех фракций
let all_factions = manager.get_all_factions();
println!("Количество фракций: {}", all_factions.len()); // Выведет: Количество фракций: 5

// Устанавливаем фракцию игрока
let success = manager.set_player_faction("technocrats");
if success {
    println!("Фракция игрока успешно установлена!");
}

// Получаем фракцию игрока
if let Some(player_faction) = manager.get_player_faction() {
    println!("Фракция игрока: {}", player_faction.name); // Выведет: Фракция игрока: Технократы
}
```

## Рекомендации по интеграции

1. При инициализации игрового мира рекомендуется создать `FactionManager` и добавить в него стандартные фракции с помощью метода `create_default_factions()`.

2. Для применения бонусов фракций используйте методы `get_resource_production_modifier` и `get_modifier_for_type` при расчете соответствующих значений в игре.

3. При создании пользовательского интерфейса используйте цвета фракции (`colors`) для обеспечения визуальной согласованности.

4. Свойство `is_player_faction` можно использовать для определения фракции, за которую играет пользователь, а метод `set_player_faction` - для изменения фракции игрока.

5. Для проверки доступа к уникальным зданиям и технологиям используйте методы `has_unique_building` и `has_unique_technology`. 