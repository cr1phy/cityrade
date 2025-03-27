# Документация модуля World

Модуль `world` предоставляет типы и структуры для создания и управления игровым миром в Cityrade.

## Типы данных

### TerrainTile

Перечисление, определяющее типы тайлов (клеток) на карте мира.

```rust
pub enum TerrainTile {
    Land,                       // Земля (базовый тип)
    Water,                      // Вода
    Mountain,                   // Горы
    Forest,                     // Лес
    Desert,                     // Пустыня
    Building(String),           // Здание (с именем)
    ResourceSpot(ResourceType), // Месторождение ресурса
    City(String),               // Город (с именем)
    Unknown,                    // Неизведанная территория
}
```

### WorldMap

Структура, представляющая карту мира с тайлами местности.

```rust
pub struct WorldMap {
    width: u64,                              // Ширина карты
    height: u64,                             // Высота карты
    terrain: HashMap<(i32, i32), TerrainTile>, // Карта тайлов местности
}
```

#### Методы WorldMap

- `new(width: u64, height: u64) -> WorldMap` - Создает новую карту мира с указанными размерами, заполненную тайлами типа Land.
- `get_width(&self) -> u64` - Возвращает ширину карты.
- `get_height(&self) -> u64` - Возвращает высоту карты.
- `expand_map(&mut self, new_width: u64, new_height: u64)` - Расширяет карту до новых размеров, заполняя новые тайлы типом Unknown.
- `set_tile(&mut self, x: i32, y: i32, tile: TerrainTile)` - Устанавливает тип тайла в указанной позиции.
- `get_tile(&self, x: i32, y: i32) -> Option<&TerrainTile>` - Возвращает тип тайла в указанной позиции.
- `add_building(&mut self, x: i32, y: i32, building_name: String)` - Добавляет здание в указанную позицию.
- `add_city(&mut self, x: i32, y: i32, city_name: String)` - Добавляет город в указанную позицию.

### WorldGenerator

Структура для процедурной генерации игрового мира.

```rust
pub struct WorldGenerator {
    _seed: u64,       // Сид для генерации (влияет на рандомизацию)
    rng: StdRng,      // Генератор случайных чисел
}
```

#### Методы WorldGenerator

- `new(seed: Option<u64>) -> Self` - Создает новый генератор мира с указанным сидом или случайным, если сид не указан.
- `generate(&mut self, width: u64, height: u64) -> WorldMap` - Генерирует карту мира с указанными размерами.
- `generate_water_bodies(&mut self, world: &mut WorldMap)` - Генерирует водоемы на карте.
- `create_water_body(&mut self, world: &mut WorldMap, x: i32, y: i32, size: u32)` - Создает одиночный водоем указанного размера.
- `generate_resources(&mut self, world: &mut WorldMap)` - Генерирует ресурсы на карте.

## Примеры использования

### Создание и использование карты мира

```rust
use cityrade_types::world::{WorldMap, TerrainTile};

// Создание карты мира 100x100
let mut world_map = WorldMap::new(100, 100);

// Установка различных типов тайлов
world_map.set_tile(10, 10, TerrainTile::Forest);
world_map.set_tile(20, 15, TerrainTile::Mountain);
world_map.set_tile(30, 30, TerrainTile::Water);

// Добавление города и здания
world_map.add_city(50, 50, "Столица".to_string());
world_map.add_building(52, 48, "Ратуша".to_string());

// Получение информации о тайле
if let Some(tile) = world_map.get_tile(50, 50) {
    match tile {
        TerrainTile::City(name) => println!("На позиции (50, 50) находится город {}", name),
        _ => println!("На позиции (50, 50) нет города"),
    }
}
```

### Генерация мира с использованием генератора

```rust
use cityrade_types::world::{WorldGenerator, TerrainTile};

// Создание генератора с случайным сидом
let mut generator = WorldGenerator::new(None);

// Генерация карты мира 200x200
let mut world = generator.generate(200, 200);

// Использование сгенерированной карты
let mut forest_count = 0;
let mut water_count = 0;

for x in 0..world.get_width() as i32 {
    for y in 0..world.get_height() as i32 {
        if let Some(tile) = world.get_tile(x, y) {
            match tile {
                TerrainTile::Forest => forest_count += 1,
                TerrainTile::Water => water_count += 1,
                _ => {},
            }
        }
    }
}

println!("Сгенерирован мир: {} лесных участков, {} водных участков", 
    forest_count, water_count);
```

### Расширение карты мира

```rust
use cityrade_types::world::WorldMap;

// Создание начальной карты 50x50
let mut world_map = WorldMap::new(50, 50);

// Игрок исследует мир, и мы расширяем карту
world_map.expand_map(100, 75);

println!("Карта расширена до размеров {}x{}", 
    world_map.get_width(), world_map.get_height());
```

## Рекомендации по интеграции

1. Используйте `WorldMap` для хранения и визуализации игрового мира. Это базовая структура для отображения местности и объектов.

2. При создании нового мира, используйте `WorldGenerator` для процедурной генерации разнообразного ландшафта вместо создания пустой карты.

3. Для больших карт, рассмотрите возможность "видимой" карты, которая отображает только изведанные территории, расширяя её по мере исследования игроком.

4. При добавлении городов и зданий, используйте соответствующие методы `add_city()` и `add_building()`, чтобы правильно обновить тайлы.

5. Для изменения местности (например, в результате строительства или природных событий), используйте метод `set_tile()` для обновления состояния мира. 