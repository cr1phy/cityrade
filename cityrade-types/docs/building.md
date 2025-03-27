# Документация модуля Building

Модуль `building` предоставляет типы и структуры для управления зданиями в игре Cityrade.

## Типы данных

### BuildingType

Перечисление, определяющее типы зданий, доступные в игре.

```rust
pub enum BuildingType {
    Residential, // Увеличивает лимит населения
    Farm,        // Производит еду
    LumberMill,  // Производит дерево
    Mine,        // Производит камень и железо
    Market,      // Увеличивает доход золота
    Barracks,    // Тренирует войска
    PowerPlant,  // Производит энергию
    Laboratory,  // Исследования
    Temple,      // Повышает счастье населения
    WaterMill,   // Увеличивает производство ресурсов
    Wall,        // Защита города
    Workshop,    // Улучшает производство предметов
    CrystalMine, // Производит кристаллы
}
```

#### Методы BuildingType

- `display_name(&self) -> &str` - Возвращает локализованное название здания на русском языке.
- `description(&self) -> &str` - Возвращает описание здания на русском языке.
- `base_cost(&self) -> Vec<(ResourceType, u32)>` - Возвращает базовую стоимость здания в виде списка пар (тип ресурса, количество).
- `production_effect(&self, level: u32) -> Vec<(ResourceType, i32)>` - Возвращает эффект, который здание данного уровня оказывает на производство ресурсов. Положительные значения обозначают производство, отрицательные - потребление.

### Building

Структура, представляющая конкретное здание в игре.

```rust
pub struct Building {
    pub id: String,              // Уникальный идентификатор здания
    pub name: String,            // Пользовательское название здания
    pub building_type: BuildingType, // Тип здания
    pub level: u32,              // Уровень здания
    pub position: (i32, i32),    // Координаты на карте города
}
```

#### Методы Building

- `new(id: String, name: String, building_type: BuildingType, position: (i32, i32)) -> Building` - Создает новое здание первого уровня.
- `upgrade(&mut self)` - Повышает уровень здания на единицу.
- `get_info(&self) -> String` - Возвращает строку с подробной информацией о здании, включая название, идентификатор, уровень, тип и описание.
- `upgrade_cost(&self) -> Vec<(ResourceType, u32)>` - Рассчитывает стоимость улучшения здания на следующий уровень. Стоимость увеличивается с каждым уровнем.
- `production_effect(&self) -> Vec<(ResourceType, i32)>` - Возвращает текущий эффект здания на производство ресурсов.
- `apply_production_to_resources(&self, resources: &mut Resource)` - Применяет эффект производства здания к указанным ресурсам (добавляет или вычитает соответствующие ресурсы).

## Примеры использования

### Создание и улучшение здания

```rust
use cityrade_types::building::{Building, BuildingType};

// Создание нового здания
let mut farm = Building::new(
    "farm_1".to_string(),
    "Ферма в южном районе".to_string(),
    BuildingType::Farm,
    (10, 15), // Позиция на карте города
);

// Вывод информации о здании
println!("{}", farm.get_info());

// Улучшение здания
farm.upgrade();
println!("Ферма улучшена до уровня {}", farm.level);

// Получение стоимости следующего улучшения
let upgrade_cost = farm.upgrade_cost();
for (resource, amount) in upgrade_cost {
    println!("Требуется {} единиц {}", amount, resource);
}
```

### Применение эффектов здания к ресурсам

```rust
use cityrade_types::building::{Building, BuildingType};
use cityrade_types::resources::Resource;

// Создание ресурсов и здания
let mut resources = Resource::new();
let power_plant = Building::new(
    "power_1".to_string(),
    "Электростанция".to_string(),
    BuildingType::PowerPlant,
    (5, 5),
);

// Вывод текущего эффекта производства
println!("Эффект производства электростанции:");
for (resource, amount) in power_plant.production_effect() {
    if amount > 0 {
        println!("Производит {} единиц {}", amount, resource);
    } else if amount < 0 {
        println!("Потребляет {} единиц {}", -amount, resource);
    }
}

// Применение эффектов к ресурсам
power_plant.apply_production_to_resources(&mut resources);
println!("После работы электростанции:");
for (resource, amount) in resources.get_all_resources() {
    println!("{}: {}", resource, amount);
}
```

## Рекомендации по интеграции

1. При создании новых зданий в городе используйте метод `new()` и указывайте уникальный идентификатор для каждого здания.

2. Для расчета общего производства в городе, примените метод `apply_production_to_resources()` для каждого здания в цикле.

3. При проверке возможности строительства или улучшения здания используйте методы `base_cost()` или `upgrade_cost()` соответственно, и проверьте наличие ресурсов с помощью `Resource::can_afford()`.

4. Используйте метод `get_info()` для отображения информации о здании в пользовательском интерфейсе.

5. Для создания новых типов зданий, расширяйте перечисление `BuildingType` и реализуйте для них необходимые методы, соблюдая баланс между стоимостью и получаемыми преимуществами. 