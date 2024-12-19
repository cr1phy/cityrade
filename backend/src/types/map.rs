use noise::{NoiseFn, Perlin};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub id: Uuid,
    pub seed: u32,
    pub width: usize,
    pub height: usize,
    pub chunks: Vec<Chunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub data: Vec<Vec<Terrain>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Terrain {
    Water,
    Grass,
    Mountain,
    Forest,
}

impl Map {
    pub fn new(seed: u32, width: usize, height: usize) -> Self {
        let mut map = Map {
            id: Uuid::now_v7(),
            seed,
            width,
            height,
            chunks: vec![],
        };
        map.generate_chunks(0, 0, width as i32, height as i32);
        map
    }

    pub fn generate_chunks(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
        let perlin = Perlin::new(self.seed);

        for y in start_y..end_y {
            for x in start_x..end_x {
                let mut chunk_data = vec![];

                for row in 0..16 {
                    let mut chunk_row = vec![];
                    for col in 0..16 {
                        let nx = (x * 16 + col) as f64 / (self.width as f64 * 16.0);
                        let ny = (y * 16 + row) as f64 / (self.height as f64 * 16.0);

                        let noise_value = perlin.get([nx, ny]);
                        let terrain = match noise_value {
                            v if v < -0.5 => Terrain::Water,
                            v if v < 0.0 => Terrain::Grass,
                            v if v < 0.5 => Terrain::Forest,
                            _ => Terrain::Mountain,
                        };
                        chunk_row.push(terrain);
                    }
                    chunk_data.push(chunk_row);
                }

                self.chunks.push(Chunk { x, y, data: chunk_data });
            }
        }
    }

    pub fn get_chunk(&self, x: i32, y: i32) -> Option<&Chunk> {
        self.chunks.iter().find(|chunk| chunk.x == x && chunk.y == y)
    }
}
