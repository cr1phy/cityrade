# Документация модуля City

Модуль `city` предоставляет типы и структуры для управления городами в игре Cityrade.

## Типы данных

### Terrain

Перечисление, определяющее типы местности, на которых могут располагаться города.

```rust
pub enum Terrain {
    Plain,   // Равнина
    Forest,  // Лес
    Mountain, // Горы
    Desert,  // Пустыня
    Swamp,   // Болото
    Water,   // Вода
    Snow,    // Снег
}
```

#### Методы Terrain

- `display_name(&self) -> &str` - Возвращает локализованное название типа местности на русском языке.
- `resource_modifier(&self) -> HashMap<ResourceType, f32>` - Возвращает модификаторы производства ресурсов для данного типа местности. Значения больше 1.0 увеличивают производство, меньше 1.0 - уменьшают.

### CityStats

Структура, представляющая статистику города, которая влияет на различные аспекты его функционирования.

```rust
pub struct CityStats {
    pub happiness: u32,      // Счастье населения (влияет на рост)
    pub defense: u32,        // Защита от нападений
    pub culture: u32,        // Культурный уровень (влияет на технологии)
    pub max_population: u32, // Максимальное население
    pub max_buildings: u32,  // Максимальное количество зданий
}
```

### City

Структура, представляющая город в игре.

```rust
pub struct City {
    pub id: String,                           // Уникальный идентификатор города
    pub name: String,                         // Название города
    pub owner_id: String,                     // Идентификатор владельца города
    pub population: u32,                      // Текущее население
    pub buildings: HashMap<String, Building>, // Здания в городе
    pub resources: Resource,                  // Ресурсы города
    pub stats: CityStats,                     // Статистика города
    pub terrain: Terrain,                     // Тип местности
    pub position: (i32, i32),                 // Позиция на карте мира
    pub created_at: DateTime<Utc>,            // Дата создания
    pub last_updated: DateTime<Utc>,          // Дата последнего обновления
}
```

#### Методы City

- `new(name: String, owner_id: String, terrain: Terrain, position: (i32, i32)) -> City` - Создает новый город с указанными параметрами.
- `update(&mut self)` - Обновляет состояние города, включая ресурсы, статистику и население.
- `update_resource_production(&mut self)` - Пересчитывает производство ресурсов с учетом зданий и типа местности.
- `update_stats(&mut self)` - Обновляет статистику города на основе зданий и других факторов.
- `update_population(&mut self)` - Обновляет население города, учитывая доступность еды и счастье.
- `add_building(&mut self, building_type: BuildingType, name: String, position: (i32, i32)) -> Result<String, String>` - Добавляет новое здание в город, возвращает идентификатор здания или ошибку.
- `upgrade_building(&mut self, building_id: &str) -> Result<(), String>` - Улучшает указанное здание, возвращает успех или ошибку.
- `remove_building(&mut self, building_id: &str) -> Result<(), String>` - Удаляет указанное здание, возвращает успех или ошибку.
- `increase_population(&mut self, amount: u32)` - Увеличивает население города на указанное количество.
- `decrease_population(&mut self, amount: u32)` - Уменьшает население города на указанное количество.
- `get_resource_report(&self) -> String` - Возвращает текстовый отчет о ресурсах города.
- `get_buildings_report(&self) -> String` - Возвращает текстовый отчет о зданиях города.
- `get_stats_report(&self) -> String` - Возвращает текстовый отчет о статистике города.
- `subtract_resources(&mut self, resource_type: &ResourceType, amount: u32) -> bool` - Вычитает указанный ресурс из запасов города, возвращает успех операции.
- `add_resources(&mut self, resource_type: &ResourceType, amount: u32)` - Добавляет указанный ресурс к запасам города.

## Примеры использования

### Создание нового города

```rust
use cityrade_types::city::{City, Terrain};

// Создание нового города
let mut city = City::new(
    "Новоград".to_string(),
    "player1".to_string(),
    Terrain::Plain,
    (100, 150) // Координаты на карте мира
);

println!("Создан город {} типа местности {}", 
    city.name, 
    city.terrain.display_name()
);
```

### Управление зданиями города

```rust
use cityrade_types::city::City;
use cityrade_types::building::BuildingType;

// Предположим, у нас уже есть город
let mut city = // ... город из предыдущего примера

// Добавление нового здания
match city.add_building(
    BuildingType::Farm,
    "Южная ферма".to_string(),
    (5, 10) // Координаты внутри города
) {
    Ok(building_id) => {
        println!("Построена ферма с ID: {}", building_id);
        
        // Улучшение здания
        match city.upgrade_building(&building_id) {
            Ok(_) => println!("Ферма улучшена!"),
            Err(e) => println!("Ошибка при улучшении фермы: {}", e),
        }
    },
    Err(e) => println!("Ошибка при строительстве фермы: {}", e),
};
```

### Управление ресурсами и обновление города

```rust
use cityrade_types::city::City;
use cityrade_types::resources::ResourceType;

// Предположим, у нас уже есть город
let mut city = // ... город из предыдущего примера

// Добавление ресурсов
city.add_resources(&ResourceType::Gold, 100);
city.add_resources(&ResourceType::Wood, 50);

// Трата ресурсов
if city.subtract_resources(&ResourceType::Gold, 25) {
    println!("Потрачено 25 золота");
} else {
    println!("Недостаточно золота");
}

// Обновление состояния города (здания производят ресурсы, население растет/уменьшается)
city.update();

// Вывод отчетов
println!("{}", city.get_resource_report());
println!("{}", city.get_stats_report());
println!("{}", city.get_buildings_report());
```

## Рекомендации по интеграции

1. Города являются основными игровыми объектами, в которых сосредоточено управление ресурсами, зданиями и населением. Используйте метод `update()` регулярно для обновления состояния города.

2. При создании новых городов, учитывайте тип местности, так как он влияет на производство ресурсов. Располагайте города на местности, соответствующей вашей стратегии развития.

3. Для эффективного управления ресурсами, используйте методы `add_resources()` и `subtract_resources()`, а также следите за отчетами о ресурсах.

4. При строительстве зданий, учитывайте модификаторы производства от типа местности и эффекты от уже существующих зданий.

5. Следите за счастьем населения, так как оно влияет на рост города. Обеспечивайте достаточное количество еды и соответствующие здания для повышения счастья. 