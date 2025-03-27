# Система технологий (Technology System)

Система технологий в Cityrade позволяет игрокам исследовать новые технологии, которые открывают доступ к новым зданиям, улучшают производство ресурсов и дают различные бонусы.

## Основные структуры данных

### TechnologyType (Тип технологии)

Перечисление `TechnologyType` определяет все доступные в игре технологии, сгруппированные по категориям:

```rust
pub enum TechnologyType {
    // Экономические
    Agriculture,    // Сельское хозяйство
    Mining,         // Горное дело
    Forestry,       // Лесное хозяйство
    Trade,          // Торговля
    Banking,        // Банковское дело
    AdvancedTrading, // Продвинутая торговля
    MarketAnalysis, // Анализ рынка

    // Строительные
    BasicConstruction,    // Основы строительства
    AdvancedConstruction, // Продвинутое строительство
    StoneWorks,          // Каменные работы
    Architecture,        // Архитектура
    CityPlanning,        // Городское планирование

    // Военные
    BasicMilitary,       // Базовая военная подготовка
    AdvancedMilitary,    // Продвинутая военная подготовка
    Fortification,       // Фортификация
    Siege,               // Осадное дело
    Tactics,             // Тактика

    // Социальные
    Education,           // Образование
    Culture,             // Культура
    Administration,      // Администрация
    Governance,          // Управление
    Diplomacy,           // Дипломатия

    // Промышленные
    BasicIndustry,       // Базовая промышленность
    Machinery,           // Машиностроение
    Engineering,         // Инженерное дело
    Automation,          // Автоматизация
}
```

### TechnologyCategory (Категория технологии)

Перечисление `TechnologyCategory` определяет категории технологий:

```rust
pub enum TechnologyCategory {
    Economic,     // Экономические технологии
    Construction, // Строительные технологии
    Military,     // Военные технологии
    Social,       // Социальные технологии
    Industrial,   // Промышленные технологии
}
```

### TechnologyEffect (Эффект технологии)

Структура `TechnologyEffect` описывает конкретное влияние технологии на игровые механики:

```rust
pub struct TechnologyEffect {
    pub description: String,                    // Описание эффекта
    pub resource_bonus: Option<(String, f32)>,  // (тип_ресурса, процент_бонуса)
    pub building_unlock: Option<String>,        // Разблокируемое здание
    pub cost_reduction: Option<(String, f32)>,  // (тип_затрат, процент_снижения)
    pub other_bonuses: HashMap<String, f32>,    // Другие бонусы (имя_бонуса, значение)
}
```

### Technology (Технология)

Структура `Technology` представляет конкретную технологию в игре:

```rust
pub struct Technology {
    pub tech_type: TechnologyType,           // Тип технологии
    pub name: String,                        // Название технологии
    pub description: String,                 // Описание технологии
    pub cost: u32,                           // Стоимость исследования (в очках исследования)
    pub research_time: u32,                  // Время исследования в ходах
    pub prerequisites: Vec<TechnologyType>,  // Необходимые предпосылки (другие технологии)
    pub category: TechnologyCategory,        // Категория технологии
    pub era: u32,                            // Эра технологии
    pub unlock_effects: Vec<TechnologyEffect>, // Эффекты разблокировки
}
```

### ResearchStatus (Статус исследования)

Перечисление `ResearchStatus` отражает текущий статус исследования технологии:

```rust
pub enum ResearchStatus {
    NotStarted,        // Исследование не начато
    InProgress(u32),   // Исследование в процессе (с текущим прогрессом)
    Completed,         // Исследование завершено
}
```

### TechnologyTree (Дерево технологий)

Структура `TechnologyTree` управляет всеми технологиями в игре:

```rust
pub struct TechnologyTree {
    technologies: HashMap<TechnologyType, Technology>,          // Все доступные технологии
    research_status: HashMap<TechnologyType, ResearchStatus>,   // Статус исследования каждой технологии
    research_focus: Option<TechnologyType>,                     // Текущий фокус исследования
    completed_technologies: HashSet<TechnologyType>,            // Завершенные технологии
    research_points: u32,                                       // Накопленные очки исследования
    research_rate: u32,                                         // Скорость получения очков исследования за ход
    tech_bonuses: HashMap<String, f32>,                         // Бонусы от технологий
}
```

## Основные методы

### Создание дерева технологий

```rust
let mut tech_tree = TechnologyTree::new();
```

Конструктор `new()` создает новое дерево технологий со всеми доступными технологиями и инициализирует их статус как `NotStarted`.

### Начало исследования технологии

```rust
match tech_tree.start_research(TechnologyType::Agriculture) {
    Ok(_) => println!("Начато исследование сельского хозяйства"),
    Err(e) => println!("Ошибка при начале исследования: {}", e),
}
```

Метод `start_research()` начинает исследование указанной технологии, если выполнены все предпосылки.

### Обновление исследования

```rust
// В игровом цикле, каждый ход
if let Some(completed_tech) = tech_tree.update_research() {
    println!("Завершено исследование: {:?}", completed_tech);
    // Обработка завершения исследования
}
```

Метод `update_research()` добавляет очки исследования к текущему фокусу и возвращает технологию, если исследование завершено.

### Получение прогресса исследования

```rust
if let Some((tech_type, current, total)) = tech_tree.get_research_progress() {
    let percent = (current as f32 / total as f32) * 100.0;
    println!("Прогресс исследования {:?}: {:.1}% ({}/{})", tech_type, percent, current, total);
}
```

Метод `get_research_progress()` возвращает информацию о текущем прогрессе исследования.

### Получение доступных технологий

```rust
let available_techs = tech_tree.get_available_technologies();
println!("Доступные для исследования технологии:");
for tech in available_techs {
    println!("- {} ({})", tech.name, tech.description);
}
```

Метод `get_available_technologies()` возвращает список технологий, доступных для исследования.

## Примеры использования

### Пример 1: Базовое исследование технологий

```rust
// Создаем дерево технологий
let mut tech_tree = TechnologyTree::new();

// Устанавливаем скорость исследования
tech_tree.set_research_rate(15); // 15 очков исследования за ход

// Начинаем исследование базовой технологии (без предпосылок)
tech_tree.start_research(TechnologyType::Agriculture).unwrap();

// Симулируем несколько ходов
for i in 1..=10 {
    println!("Ход {}", i);
    
    if let Some((tech, progress, total)) = tech_tree.get_research_progress() {
        println!("Исследуется: {:?} - Прогресс: {}/{}", tech, progress, total);
    }
    
    if let Some(completed_tech) = tech_tree.update_research() {
        println!("Завершено исследование: {:?}", completed_tech);
        
        // После завершения первой технологии начинаем следующую
        if completed_tech == TechnologyType::Agriculture {
            tech_tree.start_research(TechnologyType::Trade).unwrap();
        }
    }
}

// Получаем все завершенные технологии
let completed = tech_tree.get_completed_technologies();
println!("Всего исследовано технологий: {}", completed.len());
```

### Пример 2: Интеграция с городом

```rust
struct City {
    name: String,
    population: u32,
    resources: Resources,
    buildings: Vec<Building>,
    tech_tree: TechnologyTree,
    // Другие поля города
}

impl City {
    // Обновление города (вызывается каждый ход)
    fn update(&mut self) {
        // Обновляем ресурсы, население и т.д.
        self.update_resources();
        self.update_population();
        
        // Рассчитываем скорость исследования на основе населения и зданий
        let base_research = 5; // Базовая скорость исследования
        let population_bonus = self.population / 100; // Бонус от населения
        
        // Бонус от зданий (например, школы, библиотеки)
        let building_bonus = self.buildings.iter()
            .filter(|b| b.type_name == "School" || b.type_name == "Library")
            .map(|b| b.level * 2) // Каждый уровень здания даёт +2 к исследованию
            .sum::<u32>();
        
        // Общая скорость исследования
        let research_rate = base_research + population_bonus + building_bonus;
        
        // Применяем модификаторы от технологий
        let education_bonus = 1.0 + self.tech_tree.get_tech_bonus("ResearchRate");
        let final_rate = (research_rate as f32 * education_bonus).round() as u32;
        
        // Устанавливаем скорость исследования
        self.tech_tree.set_research_rate(final_rate);
        
        // Обновляем исследования
        if let Some(completed_tech) = self.tech_tree.update_research() {
            self.handle_completed_technology(completed_tech);
        }
    }
    
    // Обработка завершенной технологии
    fn handle_completed_technology(&mut self, tech_type: TechnologyType) {
        println!("Город {} завершил исследование: {:?}", self.name, tech_type);
        
        // Обновляем бонусы к производству ресурсов
        self.update_resource_production_bonuses();
        
        // Разблокируем новые здания
        if let Some(tech) = self.tech_tree.get_technology(&tech_type) {
            for effect in &tech.unlock_effects {
                if let Some(building_name) = &effect.building_unlock {
                    println!("Разблокировано новое здание: {}", building_name);
                    self.unlocked_buildings.insert(building_name.clone());
                }
            }
        }
        
        // Показываем уведомление игроку
        self.show_notification(format!("Исследована новая технология: {:?}", tech_type));
        
        // Автоматически начинаем следующее доступное исследование, если ничего не исследуется
        if self.tech_tree.get_research_progress().is_none() {
            let available = self.tech_tree.get_available_technologies();
            if !available.is_empty() {
                let next_tech = available[0].tech_type.clone();
                self.tech_tree.start_research(next_tech).unwrap();
            }
        }
    }
    
    // Обновление бонусов к производству ресурсов на основе исследованных технологий
    fn update_resource_production_bonuses(&mut self) {
        let resources = ["Wood", "Stone", "Food", "Gold", "Iron", "Crystal"];
        
        for resource in resources.iter() {
            let bonus = self.tech_tree.get_resource_production_bonus(resource);
            
            if bonus > 0.0 {
                println!("Бонус к производству {}: +{:.1}%", resource, bonus * 100.0);
                // Применяем бонус к производству
                self.resources.set_production_modifier(resource, 1.0 + bonus);
            }
        }
    }
}
```

### Пример 3: Интерфейс экрана исследований

```rust
// Функция отрисовки экрана исследований
fn render_research_screen(tech_tree: &TechnologyTree) {
    // Получаем текущее исследование
    let current_research = tech_tree.get_research_progress();
    
    // Заголовок экрана
    println!("=== ЭКРАН ИССЛЕДОВАНИЙ ===");
    
    // Текущее исследование
    if let Some((tech_type, progress, total)) = current_research {
        let percent = (progress as f32 / total as f32) * 100.0;
        let tech = tech_tree.get_technology(&tech_type).unwrap();
        
        println!("Текущее исследование: {}", tech.name);
        println!("Описание: {}", tech.description);
        println!("Прогресс: {}/{} ({:.1}%)", progress, total, percent);
        println!("Осталось ходов: ~{}", (total - progress) / tech_tree.research_rate);
    } else {
        println!("В данный момент ничего не исследуется");
    }
    
    // Доступные для исследования технологии
    let available = tech_tree.get_available_technologies();
    println!("\nДоступные технологии ({}):", available.len());
    
    for (i, tech) in available.iter().enumerate() {
        println!("{}. {} - {} очков, {} ходов",
            i + 1,
            tech.name,
            tech.cost,
            tech.research_time
        );
        println!("   {}", tech.description);
    }
    
    // Завершенные исследования
    let completed = tech_tree.get_completed_technologies();
    println!("\nЗавершенные исследования ({}):", completed.len());
    
    for tech in completed {
        println!("- {}", tech.name);
    }
    
    // Текущая скорость исследования
    println!("\nСкорость исследования: {} очков за ход", tech_tree.research_rate);
}

// Функция выбора новой технологии для исследования
fn select_research(tech_tree: &mut TechnologyTree) {
    let available = tech_tree.get_available_technologies();
    
    if available.is_empty() {
        println!("Нет доступных технологий для исследования");
        return;
    }
    
    // Выводим список доступных технологий
    println!("Выберите технологию для исследования:");
    for (i, tech) in available.iter().enumerate() {
        println!("{}. {} ({} очков)", i + 1, tech.name, tech.cost);
    }
    
    // Получаем выбор пользователя
    println!("Введите номер технологии:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= available.len() {
            let selected_tech = available[index - 1].tech_type.clone();
            
            match tech_tree.start_research(selected_tech) {
                Ok(_) => println!("Начато исследование: {}", available[index - 1].name),
                Err(e) => println!("Ошибка: {}", e),
            }
        } else {
            println!("Некорректный выбор");
        }
    } else {
        println!("Неверный ввод");
    }
}
```

## Интеграция с другими системами

### Интеграция с системой зданий

```rust
// После завершения исследования технологии
if let Some(completed_tech) = tech_tree.update_research() {
    let tech = tech_tree.get_technology(&completed_tech).unwrap();
    
    // Разблокировка новых зданий
    for effect in &tech.unlock_effects {
        if let Some(building_name) = &effect.building_unlock {
            building_manager.unlock_building(building_name);
        }
    }
    
    // Применение бонусов к стоимости строительства
    for effect in &tech.unlock_effects {
        if let Some((cost_type, reduction)) = &effect.cost_reduction {
            if cost_type == "Construction" {
                building_manager.set_cost_modifier(*reduction);
            }
        }
    }
}
```

### Интеграция с системой ресурсов

```rust
// Применение бонусов технологий к производству ресурсов
fn calculate_resource_production(resource_type: &str, base_production: u32, tech_tree: &TechnologyTree) -> u32 {
    let tech_bonus = tech_tree.get_resource_production_bonus(resource_type);
    let modified_production = base_production as f32 * (1.0 + tech_bonus);
    modified_production.round() as u32
}

// Пример использования
let wood_production = calculate_resource_production("Wood", 10, &tech_tree);
println!("Производство дерева: {}/ход", wood_production);
```

## Советы по использованию

1. **Балансировка дерева технологий**: Старайтесь создавать сбалансированное дерево технологий, где каждая ветвь имеет свои преимущества и специализацию.

2. **Гибкость в исследовании**: Позвольте игрокам выбирать свой путь исследований, не ограничивая их строгой линейной прогрессией.

3. **Визуализация**: Создайте наглядную визуализацию дерева технологий, которая поможет игрокам планировать стратегию исследований.

4. **Эры и эпохи**: Группируйте технологии по эрам или эпохам развития, чтобы создать ощущение прогресса и исторической эволюции.

5. **Связь с игровыми механиками**: Убедитесь, что каждая технология имеет заметный эффект на игровой процесс, будь то разблокировка нового контента или улучшение существующих механик. 