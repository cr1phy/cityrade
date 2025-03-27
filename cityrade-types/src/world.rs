use crate::resources::ResourceType;
use rand::{Rng, SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct WorldGenerator {
    _seed: u64,
    rng: StdRng,
}

impl WorldGenerator {
    pub fn new(seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or_else(|| rand::random());
        let rng = StdRng::seed_from_u64(seed);

        WorldGenerator { _seed: seed, rng }
    }

    pub fn generate(&mut self, width: u64, height: u64) -> WorldMap {
        let mut world = WorldMap::new(width, height);

        // Генерация водоемов
        self.generate_water_bodies(&mut world);

        // Генерация ресурсов
        self.generate_resources(&mut world);

        world
    }

    fn generate_water_bodies(&mut self, world: &mut WorldMap) {
        let water_bodies_count = (world.get_width() * world.get_height()) as f64 * 0.05;

        for _ in 0..water_bodies_count as u64 {
            let x = self.rng.random_range(0..world.get_width()) as i32;
            let y = self.rng.random_range(0..world.get_height()) as i32;

            // Создаем водоем разного размера
            let size = self.rng.random_range(3..10);
            self.create_water_body(world, x, y, size);
        }
    }

    fn create_water_body(&mut self, world: &mut WorldMap, x: i32, y: i32, size: u32) {
        for dx in -(size as i32)..=(size as i32) {
            for dy in -(size as i32)..=(size as i32) {
                let distance = ((dx * dx + dy * dy) as f64).sqrt();
                if distance <= size as f64 {
                    let noise = self.rng.random_range(-1.5..1.5);
                    if distance + noise <= size as f64 {
                        let nx = x + dx;
                        let ny = y + dy;

                        if nx >= 0
                            && nx < world.get_width() as i32
                            && ny >= 0
                            && ny < world.get_height() as i32
                        {
                            world.set_tile(nx, ny, TerrainTile::Water);
                        }
                    }
                }
            }
        }
    }

    fn generate_resources(&mut self, world: &mut WorldMap) {
        // Генерация областей с ресурсами (можно привязать к структуре из resources.rs)
        let resource_spots = (world.get_width() * world.get_height()) as f64 * 0.03;

        for _ in 0..resource_spots as u64 {
            let x = self.rng.random_range(0..world.get_width()) as i32;
            let y = self.rng.random_range(0..world.get_height()) as i32;

            // Проверяем, что это не вода
            if let Some(TerrainTile::Land) = world.get_tile(x, y) {
                // Здесь можно расширить: добавить enum с типами ресурсов в TerrainTile
                // и генерировать разные ресурсы (например, дерево, камень, золото)
                world.set_tile(x, y, TerrainTile::ResourceSpot(ResourceType::default()));
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMap {
    width: u64,
    height: u64,
    terrain: HashMap<(i32, i32), TerrainTile>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TerrainTile {
    Land,
    Water,
    Mountain,
    Forest,
    Desert,
    Building(String),           // Имя здания
    ResourceSpot(ResourceType), // Месторождение ресурса
    City(String),               // Имя города
    Unknown,
}

impl WorldMap {
    pub fn new(width: u64, height: u64) -> WorldMap {
        let mut terrain = HashMap::new();

        // Заполнение карты начальными значениями
        for x in 0..width as i32 {
            for y in 0..height as i32 {
                terrain.insert((x, y), TerrainTile::Land);
            }
        }

        WorldMap {
            width,
            height,
            terrain,
        }
    }

    pub fn get_width(&self) -> u64 {
        self.width
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    pub fn expand_map(&mut self, new_width: u64, new_height: u64) {
        for x in self.width..new_width {
            for y in 0..self.height {
                self.terrain
                    .insert((x as i32, y as i32), TerrainTile::Unknown);
            }
        }

        for y in self.height..new_height {
            for x in 0..new_width {
                self.terrain
                    .insert((x as i32, y as i32), TerrainTile::Unknown);
            }
        }

        self.width = new_width;
        self.height = new_height;
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: TerrainTile) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.terrain.insert((x, y), tile);
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&TerrainTile> {
        self.terrain.get(&(x, y))
    }

    pub fn add_building(&mut self, x: i32, y: i32, building_name: String) {
        if let Some(_) = self.terrain.get(&(x, y)) {
            self.terrain
                .insert((x, y), TerrainTile::Building(building_name));
        }
    }

    pub fn add_city(&mut self, x: i32, y: i32, city_name: String) {
        if let Some(_) = self.terrain.get(&(x, y)) {
            self.terrain.insert((x, y), TerrainTile::City(city_name));
        }
    }
}
