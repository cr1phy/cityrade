# Документация библиотеки cityrade-types

## Обзор

Библиотека `cityrade-types` содержит основные типы данных и структуры, используемые в игре Cityrade. Эта документация предоставляет информацию о каждом модуле, его типах данных и функциональности.

## Содержание

### Основные игровые системы

- [Ресурсы (Resources)](resources.md) - Типы и управление игровыми ресурсами
- [Города (City)](city.md) - Система управления городами
- [Здания (Building)](building.md) - Строительство и управление зданиями
- [Фракции (Faction)](faction.md) - Система фракций и их особенностей
- [Дипломатия (Diplomacy)](diplomacy.md) - Дипломатические отношения между фракциями
- [Технологии (Technology)](technology.md) - Система исследований и технологического развития
- [Мир (World)](world.md) - Генерация и управление игровым миром
- [Рынок (Market)](market.md) - Торговля и рыночная экономика

### Интеграция систем

- [Руководство по интеграции систем](integration.md) - Инструкции по интеграции различных систем и их взаимодействию

## Общие принципы работы

Библиотека `cityrade-types` предоставляет различные структуры данных, которые могут использоваться для:

1. **Определения игровых объектов** - Такие как города, здания, ресурсы и фракции.
2. **Моделирования игровых процессов** - Включая производство ресурсов, рост населения, исследование технологий и дипломатические отношения.
3. **Хранения и сериализации игрового состояния** - Большинство структур реализуют трейты Serialize и Deserialize.
4. **Предоставления игровой логики** - Методы структур определяют, как игровые объекты взаимодействуют друг с другом.

## Примеры использования

### Основные игровые циклы

Ниже представлен упрощенный пример основного игрового цикла, включающего несколько систем:

```rust
use cityrade_types::{
    city::{City, Terrain},
    faction::{Faction, FactionManager, FactionSpecialization},
    diplomacy::{DiplomacyManager, RelationType},
    resources::ResourceType,
    building::BuildingType,
};

// Инициализация менеджеров
let mut faction_manager = FactionManager::new();
let mut diplomacy_manager = DiplomacyManager::new();

// Создание фракций
let player_faction_id = faction_manager.create_faction(
    "Торговый Альянс".to_string(), 
    "Ваша фракция, специализирующаяся на торговле".to_string(),
    FactionSpecialization::Trade
);
faction_manager.set_player_faction(&player_faction_id);

let ai_faction_id = faction_manager.create_faction(
    "Промышленная Гильдия".to_string(), 
    "AI фракция, специализирующаяся на производстве".to_string(),
    FactionSpecialization::Industry
);

// Установка дипломатических отношений
diplomacy_manager.set_relation(
    &player_faction_id, 
    &ai_faction_id, 
    RelationType::Neutral, 
    50
);

// Создание города для игрока
let mut player_city = City::new(
    "Столица".to_string(),
    player_faction_id.clone(),
    Terrain::Plain,
    (100, 100)
);

// Построение здания с учетом бонусов фракции
let player_faction = faction_manager.get_faction(&player_faction_id).unwrap();
let resource_modifiers = player_faction.get_resource_production_modifiers();

// Применение модификаторов фракции (например, Торговый Альянс получает бонус к золоту)
for (resource_type, modifier) in resource_modifiers {
    if resource_type == ResourceType::Gold {
        // Бонус к производству золота
        let current_rate = player_city.resources.get_production_rate(&resource_type);
        player_city.resources.set_production_rate(
            resource_type, 
            (current_rate as f32 * modifier) as i32
        );
    }
}

// Игровой цикл (один ход)
player_city.update(); // Обновление города, включая ресурсы и население

// Если у нас есть торговый договор, получаем дополнительные ресурсы
if diplomacy_manager.get_relation(&player_faction_id, &ai_faction_id).relation_type == RelationType::Friendly {
    player_city.add_resources(&ResourceType::Gold, 25); // Торговый бонус
}
```

## Дополнительная информация

Для подробной информации о конкретных системах и их взаимодействии, обратитесь к соответствующим разделам документации, перечисленным в содержании.

Для разработчиков, желающих расширить библиотеку, рекомендуется следовать существующим шаблонам проектирования и обеспечивать совместимость с существующими системами. 