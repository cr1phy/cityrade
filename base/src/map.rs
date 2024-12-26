use noise::{NoiseFn, Perlin};
use rand::RngCore;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const CHUNK_SIZE: usize = 32;

/// Represents the map consisting of chunks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub id: Uuid,
    pub seed: u32,
    pub chunks: Vec<Vec<Chunk>>,
    pub visible_x: i32,
    pub visible_y: i32,
}

impl Map {
    /// Generates a new map using Perlin noise with chunk support.
    ///
    /// # Arguments
    ///
    /// * `seed` - Seed for the noise generator.
    /// * `visible_x` - X-axis offset for the visible area.
    /// * `visible_y` - Y-axis offset for the visible area.
    ///
    /// # Returns
    ///
    /// A new instance of `Map`.
    pub fn generate(seed: u32, visible_x: i32, visible_y: i32) -> Self {
        let perlin = Perlin::new(rand::thread_rng().next_u32());
        let mut chunks = Vec::with_capacity(3);

        for y in (visible_y - 1)..=(visible_y + 1) {
            let chunk_row: Vec<Chunk> = (visible_x - 1..=visible_x + 1)
                .into_par_iter()
                .map(|x| Chunk::generate(perlin.clone(), seed, x, y))
                .collect();
            chunks.push(chunk_row);
        }

        Self {
            id: Uuid::now_v7(),
            seed,
            chunks,
            visible_x,
            visible_y,
        }
    }

    /// Expands the map in the specified direction.
    ///
    /// # Arguments
    ///
    /// * `direction` - Direction to expand the map.
    pub fn expand(&mut self, direction: Direction) {
        let perlin = Perlin::new(rand::thread_rng().next_u32());
        match direction {
            Direction::Up => {
                self.visible_y -= 1;
                let new_row: Vec<Chunk> = (self.visible_x - 1..=self.visible_x + 1)
                    .into_par_iter()
                    .map(|x| Chunk::generate(perlin, self.seed, x, self.visible_y - 1))
                    .collect();
                self.chunks.insert(0, new_row);
            },
            Direction::Down => {
                self.visible_y += 1;
                let new_row: Vec<Chunk> = (self.visible_x - 1..=self.visible_x + 1)
                    .into_par_iter()
                    .map(|x| Chunk::generate(perlin, self.seed, x, self.visible_y + 1))
                    .collect();
                self.chunks.push(new_row);
            },
            Direction::Left => {
                self.visible_x -= 1;
                for row in &mut self.chunks {
                    row.insert(0, Chunk::generate(perlin, self.seed, self.visible_x - 1, row[0].y));
                }
            },
            Direction::Right => {
                self.visible_x += 1;
                for row in &mut self.chunks {
                    row.push(Chunk::generate(perlin, self.seed, self.visible_x + 1, row[0].y));
                }
            }
        }
    }
}

/// Directions for expanding the map.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Represents a chunk of the map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub terrain: Vec<Vec<TerrainType>>,
    pub resources: Vec<Vec<Option<ResourceType>>>,
    pub x: i32,
    pub y: i32,
}

impl Chunk {
    /// Generates a chunk using Perlin noise.
    ///
    /// # Arguments
    ///
    /// * `perlin` - Perlin noise generator.
    /// * `seed` - Seed for the noise generator.
    /// * `offset_x` - X-axis offset for the chunk.
    /// * `offset_y` - Y-axis offset for the chunk.
    ///
    /// # Returns
    ///
    /// A new instance of `Chunk`.
    pub fn generate(perlin: Perlin, seed: u32, offset_x: i32, offset_y: i32) -> Self {
        let mut terrain = Vec::with_capacity(CHUNK_SIZE);
        let mut resources = Vec::with_capacity(CHUNK_SIZE);

        for y in 0..CHUNK_SIZE {
            let mut row_terrain = Vec::with_capacity(CHUNK_SIZE);
            let mut row_resources = Vec::with_capacity(CHUNK_SIZE);
            for x in 0..CHUNK_SIZE {
                let noise_value = perlin.get([((offset_x + x as i32) as f64) / 10.0, ((offset_y + y as i32) as f64) / 10.0, seed as f64]);
                row_terrain.push(TerrainType::from_noise(noise_value));
                
                let resource_value = perlin.get([((offset_x + x as i32) as f64) / 5.0, ((offset_y + y as i32) as f64) / 5.0, seed as f64]);
                row_resources.push(Some(ResourceType::from_noise(resource_value)));
            }
            terrain.push(row_terrain);
            resources.push(row_resources);
        }

        Self {
            terrain,
            resources,
            x: offset_x,
            y: offset_y,
        }
    }
}

/// Types of terrain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerrainType {
    Water,
    Grass,
    Mountain,
}

impl TerrainType {
    /// Converts noise value to terrain type.
    ///
    /// # Arguments
    ///
    /// * `value` - Noise value.
    ///
    /// # Returns
    ///
    /// Corresponding `TerrainType`.
    pub fn from_noise(value: f64) -> Self {
        if value < -0.3 {
            TerrainType::Water
        } else if value < 0.5 {
            TerrainType::Grass
        } else {
            TerrainType::Mountain
        }
    }
}

/// Types of resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Iron,
    Coal,
    Gold,
    Silver,
    None,
}

impl ResourceType {
    /// Converts noise value to resource type.
    ///
    /// # Arguments
    ///
    /// * `value` - Noise value.
    ///
    /// # Returns
    ///
    /// Corresponding `ResourceType`.
    pub fn from_noise(value: f64) -> Self {
        if value < -0.5 {
            ResourceType::None
        } else if value < 0.0 {
            ResourceType::Iron
        } else if value < 0.3 {
            ResourceType::Coal
        } else if value < 0.6 {
            ResourceType::Gold
        } else {
            ResourceType::Silver
        }
    }
}
