# Система случайных событий (Random Events System)

Система случайных событий в Cityrade предоставляет механизм для генерации событий, влияющих на геймплей и придающих игре больше разнообразия. События могут быть как положительными, так и отрицательными, и охватывают различные аспекты игры.

## Основные структуры данных

### EventCategory (Категория события)

Перечисление `EventCategory` определяет различные категории событий в игре:

```rust
pub enum EventCategory {
    Economic,   // Экономические события
    Climate,    // Климатические события
    Social,     // Социальные события
    Political,  // Политические события
    Random,     // Случайные события
}
```

### EventSeverity (Серьезность события)

Перечисление `EventSeverity` определяет серьезность или характер события:

```rust
pub enum EventSeverity {
    Positive,   // Положительное событие
    Neutral,    // Нейтральное событие
    Minor,      // Незначительное негативное событие
    Major,      // Значительное негативное событие
    Disaster,   // Катастрофическое событие
}
```

### EventEffect (Эффект события)

Структура `EventEffect` описывает конкретное влияние события на игровые ресурсы или параметры:

```rust
pub struct EventEffect {
    pub resource_type: Option<ResourceType>, // Тип ресурса, на который влияет эффект
    pub amount: i32,                         // Абсолютное изменение (может быть положительным или отрицательным)
    pub percent: Option<f32>,                // Процентное изменение
    pub duration: Option<u32>,               // Длительность эффекта в ходах (None = мгновенное)
    pub description: String,                 // Описание эффекта
}
```

### RandomEvent (Случайное событие)

Структура `RandomEvent` представляет конкретное событие в игре:

```rust
pub struct RandomEvent {
    pub event_id: String,                     // Уникальный идентификатор события
    pub name: String,                         // Название события
    pub description: String,                  // Описание события
    pub category: EventCategory,              // Категория события
    pub severity: EventSeverity,              // Серьезность события
    pub effects: Vec<EventEffect>,            // Эффекты события
    pub weight: u32,                          // Вес (вероятность) события
    pub min_population: Option<u32>,          // Минимальное население для события
    pub required_buildings: Vec<String>,      // Требуемые здания
    pub incompatible_events: Vec<String>,     // Несовместимые события
    pub follow_up_events: Vec<(String, u32)>, // События-продолжения и их вероятность
}
```

### EventManager (Менеджер событий)

Структура `EventManager` управляет всеми событиями в игре:

```rust
pub struct EventManager {
    events: HashMap<String, RandomEvent>,     // Все доступные события
    active_events: Vec<(String, u32)>,        // Активные события и их оставшаяся длительность
    rng: ThreadRng,                           // Генератор случайных чисел
}
```

## Основные методы

### Создание менеджера событий

```rust
let mut event_manager = EventManager::new();
```

Конструктор `new()` создает новый экземпляр `EventManager` с предопределенным набором стандартных событий.

### Генерация случайного события

```rust
if let Some(event) = event_manager.generate_event(&my_city) {
    println!("Произошло событие: {}", event.name);
    println!("Описание: {}", event.description);
    
    // Применяем эффекты события к ресурсам города
    event_manager.apply_event_effects(event, &mut my_city.resources);
}
```

Метод `generate_event()` анализирует текущее состояние города и генерирует подходящее случайное событие с учетом всех ограничений.

### Обновление активных событий

```rust
// В конце каждого хода
event_manager.update_active_events(&mut my_city.resources);
```

Метод `update_active_events()` обновляет статус всех активных событий, уменьшая их оставшуюся длительность и применяя их продолжающиеся эффекты.

### Добавление собственного события

```rust
let custom_event = RandomEvent {
    event_id: "custom_festival".to_string(),
    name: "Фестиваль урожая".to_string(),
    description: "Жители организовали фестиваль в честь хорошего урожая.".to_string(),
    category: EventCategory::Social,
    severity: EventSeverity::Positive,
    effects: vec![
        EventEffect {
            resource_type: Some(ResourceType::Food),
            amount: 50,
            percent: None,
            duration: None,
            description: "Дополнительная еда от фестиваля".to_string(),
        },
        EventEffect {
            resource_type: Some(ResourceType::Gold),
            amount: -20,
            percent: None,
            duration: None,
            description: "Расходы на проведение фестиваля".to_string(),
        }
    ],
    weight: 15,
    min_population: Some(50),
    required_buildings: vec!["Farm".to_string()],
    incompatible_events: Vec::new(),
    follow_up_events: Vec::new(),
};

event_manager.add_event(custom_event);
```

## Примеры использования

### Пример 1: Базовое использование

```rust
// Создаем менеджер событий
let mut event_manager = EventManager::new();

// В основном игровом цикле, каждый ход
if rng.gen_range(0..100) < 10 {  // 10% шанс события каждый ход
    if let Some(event) = event_manager.generate_event(&city) {
        // Показываем игроку информацию о событии
        show_event_notification(event);
        
        // Применяем эффекты события к ресурсам
        event_manager.apply_event_effects(event, &mut city.resources);
    }
}

// В конце хода обновляем все активные события
event_manager.update_active_events(&mut city.resources);
```

### Пример 2: Использование с системой событий города

```rust
// В системе обновления города
fn update_city(city: &mut City, event_manager: &mut EventManager) {
    // Обновление ресурсов и других параметров города
    city.update_resources();
    
    // Проверка на генерацию события с вероятностью, зависящей от счастья населения
    // Чем ниже счастье, тем выше шанс негативных событий
    let happiness = city.get_happiness();
    let event_chance = if happiness < 0.3 {
        20  // 20% шанс при низком счастье
    } else if happiness < 0.6 {
        10  // 10% шанс при среднем счастье
    } else {
        5   // 5% шанс при высоком счастье
    };
    
    if rand::thread_rng().gen_range(0..100) < event_chance {
        if let Some(event) = event_manager.generate_event(city) {
            // Обработка события
            handle_event(city, event, event_manager);
        }
    }
    
    // Обновление активных событий
    event_manager.update_active_events(&mut city.resources);
}

fn handle_event(city: &mut City, event: &RandomEvent, event_manager: &EventManager) {
    // Показываем уведомление игроку
    display_event_notification(event);
    
    // Применяем эффекты события
    event_manager.apply_event_effects(event, &mut city.resources);
    
    // Дополнительные эффекты в зависимости от серьезности события
    match event.severity {
        EventSeverity::Disaster => {
            // Снижаем счастье населения значительно
            city.modify_happiness(-0.2);
            
            // Возможно запускаем эвакуацию или другие защитные меры
            if city.has_building("EmergencyCenter") {
                city.start_emergency_protocol();
            }
        },
        EventSeverity::Major => {
            // Снижаем счастье населения умеренно
            city.modify_happiness(-0.1);
        },
        EventSeverity::Minor => {
            // Незначительное снижение счастья
            city.modify_happiness(-0.05);
        },
        EventSeverity::Positive => {
            // Повышаем счастье населения
            city.modify_happiness(0.1);
        },
        _ => {}
    }
}
```

## Интеграция с другими системами

### Интеграция с рыночной системой

Экономические события могут напрямую влиять на рыночные цены и торговлю:

```rust
// После применения экономического события
if event.category == EventCategory::Economic {
    match event.severity {
        EventSeverity::Major | EventSeverity::Disaster => {
            // Вызываем рыночный шок
            let severity = match event.severity {
                EventSeverity::Major => 0.2,     // 20% шок
                EventSeverity::Disaster => 0.4,   // 40% шок
                _ => 0.0,
            };
            
            // Находим затронутые ресурсы
            let affected_resources = event.effects.iter()
                .filter_map(|effect| effect.resource_type.clone())
                .collect::<Vec<_>>();
            
            if !affected_resources.is_empty() {
                // Применяем шок к конкретным ресурсам
                city.market.apply_market_shock(severity, Some(affected_resources));
            } else {
                // Общий рыночный шок
                city.market.apply_market_shock(severity, None);
            }
        },
        _ => {}
    }
}
```

### Интеграция с системой дипломатии

События могут влиять на дипломатические отношения:

```rust
// Для политических событий проверяем влияние на дипломатию
if event.category == EventCategory::Political {
    // Находим соседние фракции
    let neighboring_factions = diplomacy_manager.get_neighboring_factions(&city.faction_id);
    
    for faction_id in neighboring_factions {
        match event.severity {
            EventSeverity::Positive => {
                // Положительное событие улучшает отношения
                diplomacy_manager.change_reputation(&city.faction_id, &faction_id, 5);
            },
            EventSeverity::Disaster => {
                // Катастрофическое событие может ухудшить отношения
                // из-за беженцев или других проблем
                diplomacy_manager.change_reputation(&city.faction_id, &faction_id, -10);
            },
            _ => {}
        }
    }
}
```

## Советы по использованию

1. **Баланс событий**: Убедитесь, что у вас есть хороший баланс между положительными и отрицательными событиями, чтобы игра не казалась слишком легкой или слишком сложной.

2. **Контекстуальные события**: Создавайте события, которые имеют смысл в текущем контексте игры. Например, события засухи имеют больше смысла в городах с сельскохозяйственным уклоном.

3. **Масштабирование эффектов**: Масштабируйте эффекты событий в зависимости от размера города или этапа игры, чтобы они оставались актуальными на протяжении всей игры.

4. **События-продолжения**: Используйте механику событий-продолжений для создания мини-историй или квестов, которые разворачиваются с течением времени.

5. **Пользовательские события**: Позвольте модификаторам или игрокам добавлять собственные события для расширения игрового контента. 