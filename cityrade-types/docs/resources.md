# Система Ресурсов (Resource System)

Система ресурсов в Cityrade предоставляет базовый механизм для управления различными типами ресурсов, их количеством и скоростью производства.

## Основные структуры данных

### ResourceType (Тип ресурса)

Перечисление `ResourceType` определяет все доступные типы ресурсов в игре:

```rust
pub enum ResourceType {
    Gold,        // Золото - основная валюта
    Wood,        // Дерево - строительный материал
    Stone,       // Камень - строительный материал
    Food,        // Еда - необходима для роста населения
    Iron,        // Железо - редкий ресурс для продвинутых строений 
    Crystal,     // Кристаллы - редкий ресурс для продвинутых технологий
    Population,  // Население - рабочая сила
    Energy,      // Энергия - требуется для работы некоторых зданий
}
```

### Resource (Ресурс)

Структура `Resource` управляет коллекцией ресурсов и их скоростью производства:

```rust
pub struct Resource {
    resources: HashMap<ResourceType, u32>,         // Количество каждого ресурса
    production_rate: HashMap<ResourceType, i32>,   // Скорость производства ресурсов
}
```

## Основные методы

### Создание ресурсов

```rust
// Создание набора ресурсов с предустановленными значениями
let resources = Resource::new();

// Создание с пользовательскими значениями
let mut custom_values = HashMap::new();
custom_values.insert(ResourceType::Gold, 500);
custom_values.insert(ResourceType::Wood, 200);
let resources = Resource::with_values(custom_values);
```

Конструктор `new()` создает стандартный набор ресурсов с предустановленными значениями:
- Золото: 100 (производство: 5/ход)
- Дерево: 100 (производство: 8/ход)
- Камень: 50 (производство: 3/ход)
- Еда: 200 (производство: 10/ход)
- Железо: 0 (производство: 0/ход)
- Кристаллы: 0 (производство: 0/ход)
- Население: 10 (производство: 1/ход)
- Энергия: 50 (производство: 2/ход)

### Получение и установка ресурсов

```rust
// Получить текущее количество ресурса
let gold = resources.get(&ResourceType::Gold);

// Установить количество ресурса
resources.set(ResourceType::Wood, 300);

// Добавить ресурсы
resources.add(&ResourceType::Food, 50);

// Вычесть ресурсы (возвращает true, если операция успешна)
if resources.subtract(&ResourceType::Stone, 20) {
    println!("Успешно использовано 20 камня");
} else {
    println!("Недостаточно камня!");
}
```

### Управление производством

```rust
// Получить текущую скорость производства ресурса
let wood_production = resources.get_production_rate(&ResourceType::Wood);

// Установить скорость производства ресурса
resources.set_production_rate(ResourceType::Iron, 5);

// Обновить все ресурсы согласно их скорости производства
resources.update_production();
```

### Проверка наличия ресурсов и оплата

```rust
// Определяем стоимость строительства
let building_cost = vec![
    (ResourceType::Wood, 100),
    (ResourceType::Stone, 50),
    (ResourceType::Gold, 200)
];

// Проверяем, можем ли мы позволить себе строительство
if resources.can_afford(&building_cost) {
    // Оплачиваем строительство
    resources.pay(&building_cost);
    println!("Строительство начато!");
} else {
    println!("Недостаточно ресурсов для строительства!");
}
```

## Примеры использования

### Пример 1: Управление ресурсами города

```rust
struct City {
    name: String,
    resources: Resource,
    buildings: Vec<Building>,
}

impl City {
    fn new(name: &str) -> Self {
        City {
            name: name.to_string(),
            resources: Resource::new(),
            buildings: Vec::new(),
        }
    }
    
    fn update(&mut self) {
        // Обновляем производство ресурсов на основе зданий
        for building in &self.buildings {
            match building.building_type {
                BuildingType::Farm => {
                    self.resources.set_production_rate(
                        ResourceType::Food,
                        10 + building.level as i32 * 5
                    );
                },
                BuildingType::LumberMill => {
                    self.resources.set_production_rate(
                        ResourceType::Wood,
                        8 + building.level as i32 * 3
                    );
                },
                // ... другие типы зданий
            }
        }
        
        // Потребление еды населением
        let population = self.resources.get(&ResourceType::Population);
        let food_consumption = (population as i32) / 2; // 1/2 единицы еды на человека
        
        let current_food_production = self.resources.get_production_rate(&ResourceType::Food);
        self.resources.set_production_rate(
            ResourceType::Food,
            current_food_production - food_consumption
        );
        
        // Обновляем количество ресурсов
        self.resources.update_production();
    }
    
    fn can_build(&self, building_type: &BuildingType) -> bool {
        let cost = building_type.base_cost();
        self.resources.can_afford(&cost)
    }
    
    fn build(&mut self, building_type: BuildingType) -> Result<(), String> {
        let cost = building_type.base_cost();
        
        if self.resources.can_afford(&cost) {
            self.resources.pay(&cost);
            self.buildings.push(Building::new(building_type));
            Ok(())
        } else {
            Err("Недостаточно ресурсов для строительства".to_string())
        }
    }
}
```

### Пример 2: Создание интерфейса ресурсов

```rust
fn display_resources(resources: &Resource) {
    println!("=== РЕСУРСЫ ===");
    println!("Золото: {} ({:+})", 
        resources.get(&ResourceType::Gold),
        resources.get_production_rate(&ResourceType::Gold));
        
    println!("Дерево: {} ({:+})", 
        resources.get(&ResourceType::Wood),
        resources.get_production_rate(&ResourceType::Wood));
        
    println!("Камень: {} ({:+})", 
        resources.get(&ResourceType::Stone),
        resources.get_production_rate(&ResourceType::Stone));
        
    println!("Еда: {} ({:+})", 
        resources.get(&ResourceType::Food),
        resources.get_production_rate(&ResourceType::Food));
        
    println!("Железо: {} ({:+})", 
        resources.get(&ResourceType::Iron),
        resources.get_production_rate(&ResourceType::Iron));
        
    println!("Кристаллы: {} ({:+})", 
        resources.get(&ResourceType::Crystal),
        resources.get_production_rate(&ResourceType::Crystal));
        
    println!("Население: {} ({:+})", 
        resources.get(&ResourceType::Population),
        resources.get_production_rate(&ResourceType::Population));
        
    println!("Энергия: {} ({:+})", 
        resources.get(&ResourceType::Energy),
        resources.get_production_rate(&ResourceType::Energy));
}
```

## Интеграция с другими системами

### Интеграция с системой зданий

```rust
// В файле building.rs
impl Building {
    pub fn get_resource_production(&self) -> Vec<(ResourceType, i32)> {
        match self.building_type {
            BuildingType::Farm => vec![(ResourceType::Food, 10 + 5 * self.level as i32)],
            BuildingType::LumberMill => vec![(ResourceType::Wood, 8 + 3 * self.level as i32)],
            BuildingType::Mine => vec![
                (ResourceType::Stone, 5 + 2 * self.level as i32),
                (ResourceType::Iron, 2 + self.level as i32),
            ],
            // ... другие типы зданий
        }
    }
    
    pub fn get_resource_consumption(&self) -> Vec<(ResourceType, i32)> {
        match self.building_type {
            BuildingType::PowerPlant => vec![(ResourceType::Gold, 5 + 2 * self.level as i32)],
            BuildingType::Barracks => vec![
                (ResourceType::Food, 5 + 2 * self.level as i32),
                (ResourceType::Gold, 10 + 5 * self.level as i32),
            ],
            // ... другие типы зданий
        }
    }
}
```

### Интеграция с системой ландшафта

```rust
// В файле terrain.rs
impl Terrain {
    pub fn get_resource_modifier(&self, resource_type: &ResourceType) -> f32 {
        match (self, resource_type) {
            (Terrain::Forest, ResourceType::Wood) => 1.5,  // +50% к производству дерева в лесу
            (Terrain::Mountain, ResourceType::Stone) => 1.3, // +30% к производству камня в горах
            (Terrain::Mountain, ResourceType::Iron) => 1.2, // +20% к производству железа в горах
            (Terrain::Desert, ResourceType::Crystal) => 1.2, // +20% к производству кристаллов в пустыне
            (Terrain::Water, ResourceType::Food) => 1.3,   // +30% к производству еды на водных территориях
            (Terrain::Plain, ResourceType::Food) => 1.2,   // +20% к производству еды на равнинах
            // По умолчанию модификатор не применяется
            _ => 1.0,
        }
    }
}
```

### Интеграция с системой фракций

```rust
// В игровой логике
fn update_city_resources(city: &mut City, faction: &Faction) {
    // Получаем базовое производство ресурсов от зданий
    let mut total_production = HashMap::new();
    
    for building in &city.buildings {
        for (resource, amount) in building.get_resource_production() {
            let current = total_production.get(&resource).cloned().unwrap_or(0);
            total_production.insert(resource, current + amount);
        }
    }
    
    // Применяем бонусы фракции к производству ресурсов
    for (resource, amount) in total_production.iter_mut() {
        for bonus in faction.get_bonuses() {
            match bonus {
                FactionBonus::ResourceProduction(res_type, modifier) if *res_type == *resource => {
                    *amount = (*amount as f32 * modifier) as i32;
                }
                _ => {}
            }
        }
        
        // Устанавливаем итоговую скорость производства
        city.resources.set_production_rate(resource.clone(), *amount);
    }
    
    // Обновляем ресурсы
    city.resources.update_production();
}
```

## Советы по использованию

1. **Балансировка ресурсов**: Тщательно настраивайте начальные значения ресурсов и скорости производства для обеспечения сбалансированной игровой экономики.

2. **Отрицательное производство**: Используйте отрицательные скорости производства для моделирования расходов (например, содержание армии потребляет еду и золото).

3. **Ресурсные циклы**: Создавайте циклы производства ресурсов, где производство одного ресурса требует наличия другого.

4. **Визуализация**: Всегда отображайте не только текущее количество ресурса, но и скорость его производства, чтобы игроки могли планировать свои действия.

5. **Хранение ресурсов**: Рассмотрите возможность введения лимитов хранения ресурсов, чтобы стимулировать игроков тратить или продавать избыточные ресурсы.

6. **Специализация**: Поощряйте игроков специализироваться в производстве определенных ресурсов и торговать излишками с другими игроками или городами. 