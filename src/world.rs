use std::collections::HashMap;

use crate::{
    block::{get_block_texture_ids, Block},
    texture::TextureManager,
    Vertex,
};
use cgmath::{Vector2, Vector3};
use rand::Rng;
use noise::{NoiseFn, Perlin};

#[derive(Debug)]
pub struct Chunk {
    block_data: [[[Block; 16]; 16]; 256],
    position: Vector2<f32>,
    mesh: Option<Vec<Vertex>>,
}

impl Chunk {
    pub fn new<T: Into<Vector2<f32>>>(position: T) -> Self {
        let block_data = [[[Block::Air; 16]; 16]; 256];
        Self {
            block_data,
            position: position.into(),
            mesh: None,
        }
    }

    pub fn mesh(&self) -> Option<&[Vertex]> {
        self.mesh.as_ref().map(|x| &**x)
    }

    pub fn generate_mesh(&mut self, texture_manager: &TextureManager) -> &[Vertex] {
        // let start = std::time::Instant::now();

        let mut texture_id_cache = HashMap::new();
        let mut vertices = vec![];
        for y in 0..self.block_data.len() {
            for x in 0..self.block_data[y].len() {
                for z in 0..self.block_data[y][x].len() {
                    let current = self.block_data[y][x][z];
                    if current == Block::Air {
                        continue;
                    }
                    let mut top = Block::Air;
                    let mut bottom = Block::Air;
                    let mut north = Block::Air;
                    let mut south = Block::Air;
                    let mut east = Block::Air;
                    let mut west = Block::Air;
                    if y < 255 {
                        top = self.block_data[y + 1][x][z];
                    }
                    if y > 0 {
                        bottom = self.block_data[y - 1][x][z];
                    }
                    if x < 15 {
                        north = self.block_data[y][x + 1][z];
                    }
                    if x > 0 {
                        south = self.block_data[y][x - 1][z];
                    }
                    if z > 0 {
                        east = self.block_data[y][x][z - 1];
                    }
                    if z < 15 {
                        west = self.block_data[y][x][z + 1];
                    }

                    let position: Vector3<f32> = (
                        x as f32 + self.position.x * 16.0,
                        y as f32,
                        z as f32 + self.position.y * 16.0,
                    )
                        .into();

                    if let None = texture_id_cache.get(&current) {
                        let textures = get_block_texture_ids(current, texture_manager);

                        match top {
                            Block::Air => {
                                let texture_id = textures[crate::Cardinal::Up as usize];
                                vertices.append(&mut top_face(position, texture_id).to_vec());
                            }
                            _ => {}
                        }
                        match bottom {
                            Block::Air => {
                                let texture_id = textures[crate::Cardinal::Down as usize];
                                vertices.append(&mut bottom_face(position, texture_id).to_vec());
                            }
                            _ => {}
                        }
                        match north {
                            Block::Air => {
                                let texture_id = textures[crate::Cardinal::North as usize];
                                vertices.append(&mut north_face(position, texture_id).to_vec());
                            }
                            _ => {}
                        }
                        match south {
                            Block::Air => {
                                let texture_id = textures[crate::Cardinal::South as usize];
                                vertices.append(&mut south_face(position, texture_id).to_vec());
                            }
                            _ => {}
                        }
                        match east {
                            Block::Air => {
                                let texture_id = textures[crate::Cardinal::East as usize];
                                vertices.append(&mut east_face(position, texture_id).to_vec());
                            }
                            _ => {}
                        }
                        match west {
                            Block::Air => {
                                let texture_id = textures[crate::Cardinal::West as usize];
                                vertices.append(&mut west_face(position, texture_id).to_vec());
                            }
                            _ => {}
                        }
                        texture_id_cache.insert(current, textures);
                    } else {
                        let textures = texture_id_cache.get(&current).unwrap();
                        match top {
                            Block::Air => {
                                let texture_id = textures[crate::Cardinal::Up as usize];
                                vertices.append(&mut top_face(position, texture_id).to_vec());
                            }
                            _ => {}
                        }
                        match bottom {
                            Block::Air => {
                                let texture_id = textures[crate::Cardinal::Down as usize];
                                vertices.append(&mut bottom_face(position, texture_id).to_vec());
                            }
                            _ => {}
                        }
                        match north {
                            Block::Air => {
                                let texture_id = textures[crate::Cardinal::North as usize];
                                vertices.append(&mut north_face(position, texture_id).to_vec());
                            }
                            _ => {}
                        }
                        match south {
                            Block::Air => {
                                let texture_id = textures[crate::Cardinal::South as usize];
                                vertices.append(&mut south_face(position, texture_id).to_vec());
                            }
                            _ => {}
                        }
                        match east {
                            Block::Air => {
                                let texture_id = textures[crate::Cardinal::East as usize];
                                vertices.append(&mut east_face(position, texture_id).to_vec());
                            }
                            _ => {}
                        }
                        match west {
                            Block::Air => {
                                let texture_id = textures[crate::Cardinal::West as usize];
                                vertices.append(&mut west_face(position, texture_id).to_vec());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        // println!("{:?}", start.elapsed().as_secs_f32());
        self.mesh = Some(vertices);
        self.mesh.as_ref().unwrap()
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            block_data: [[[Block::default(); 16]; 16]; 256],
            position: (0.0, 0.0).into(),
            mesh: None,
        }
    }
}

pub struct World {
    chunks: Vec<Vec<Chunk>>,
    seed: [u8; 32],
    seed_string: String,
}

impl World {
    pub fn new(seed: String) -> Self {
        let seed_string = seed.clone();
        let mut rng = rand_seeder::SipHasher::from(seed).into_rng();
        let mut seed = [0; 32];
        rng.fill(&mut seed);

        Self {
            chunks: vec![vec![]],
            seed,
            seed_string,
        }
    }
    pub fn mesh(&mut self, texture_manager: &TextureManager) -> Vec<Vertex> {
        let mut mesh = vec![];
        for chunk_rows in &mut self.chunks {
            for chunk in chunk_rows {
                if let Some(chunk_mesh) = &chunk.mesh() {
                    for vertex in *chunk_mesh {
                        mesh.push(*vertex);
                    }
                } else {
                    let chunk_mesh = chunk.generate_mesh(texture_manager);
                    for vertex in chunk_mesh {
                        mesh.push(*vertex);
                    }
                }
            }
        }
        mesh
    }

    pub fn generate_chunk(&mut self, at_position: (f32, f32)) {
        let perlin = Perlin::new(1);
        let mut chunk = Chunk::new(at_position);
        for x in 0..16 {
            for z in 0..16 {
                let y =
                (
                    1.0 * perlin.get([0.5 * (x as f32 / 16.0 + chunk.position.x) as f64, 0.5 * (z as f32 / 16.0 + chunk.position.y) as f64]) +
                    3.33 * perlin.get([0.05 * (x as f32 / 16.0 + chunk.position.x) as f64, 0.05 * (z as f32 / 16.0 + chunk.position.y) as f64]) +
                    20.0 * perlin.get([0.0005 * (x as f32 / 16.0 + chunk.position.x) as f64, 0.0005 * (z as f32 / 16.0 + chunk.position.y) as f64]) + 
                    300.0 * perlin.get([0.0005 * (x as f32 / 16.0 + chunk.position.x) as f64, 0.0005 * (z as f32 / 16.0 + chunk.position.y) as f64])
                ) * 3.0 + 60.0;
                // println!("{:?}", y);
                let y = y as usize;
                chunk.block_data[y][x][z] = Block::Grass;
                for height in (0..y).rev() {
                    chunk.block_data[height][x][z] = Block::Dirt;
                    if y < 4 {
                        chunk.block_data[height][x][z] = Block::Stone;
                    }
                    else if height < y - 4 {
                        chunk.block_data[height][x][z] = Block::Stone;
                    }
                }
            }
        }
        chunk.block_data[0] = [[Block::Bedrock; 16]; 16];
        self.chunks[0].push(chunk);
    }
}

fn north_face(position: Vector3<f32>, texture_id: f32) -> [Vertex; 6] {
    // [x, y, z]

    let y = position.y - 60.0;
    let positions = [
        // North bottom left (0)
        [position.z, y, position.x],
        // North top right (1)
        [position.z + 1.0, y + 1.0, position.x],
        // North bottom right(2)
        [position.z + 1.0, y, position.x],
        // North top left (3)
        [position.z, y + 1.0, position.x],
        // South top left (4)
        [position.z + 1.0, y + 1.0, position.x + 1.0],
        // South top right (5)
        [position.z, y + 1.0, position.x + 1.0],
        // South bottom left (6)
        [position.z + 1.0, y, position.x + 1.0],
        // South bottom right (7)
        [position.z, y, position.x + 1.0],
    ];
    [
        //first triangle
        Vertex {
            position: positions[6],
            tex_coord: [0.0, 1.0, texture_id],
        },
        Vertex {
            position: positions[5],
            tex_coord: [1.0, 0.0, texture_id],
        },
        Vertex {
            position: positions[7],
            tex_coord: [1.0, 1.0, texture_id],
        },
        // second triangle
        Vertex {
            position: positions[6],
            tex_coord: [0.0, 1.0, texture_id],
        },
        Vertex {
            position: positions[4],
            tex_coord: [0.0, 0.0, texture_id],
        },
        Vertex {
            position: positions[5],
            tex_coord: [1.0, 0.0, texture_id],
        },
    ]
}
fn south_face(position: Vector3<f32>, texture_id: f32) -> [Vertex; 6] {
    // [x, y, z]
    let y = position.y - 60.0;
    let positions = [
        // North bottom left (0)
        [position.z, y, position.x],
        // North top right (1)
        [position.z + 1.0, y + 1.0, position.x],
        // North bottom right(2)
        [position.z + 1.0, y, position.x],
        // North top left (3)
        [position.z, y + 1.0, position.x],
        // South top left (4)
        [position.z + 1.0, y + 1.0, position.x + 1.0],
        // South top right (5)
        [position.z, y + 1.0, position.x + 1.0],
        // South bottom left (6)
        [position.z + 1.0, y, position.x + 1.0],
        // South bottom right (7)
        [position.z, y, position.x + 1.0],
    ];
    [
        //first triangle
        Vertex {
            position: positions[0],
            tex_coord: [0.0, 1.0, texture_id],
        },
        Vertex {
            position: positions[3],
            tex_coord: [0.0, 0.0, texture_id],
        },
        Vertex {
            position: positions[1],
            tex_coord: [1.0, 0.0, texture_id],
        },
        // second triangle
        Vertex {
            position: positions[0],
            tex_coord: [0.0, 1.0, texture_id],
        },
        Vertex {
            position: positions[1],
            tex_coord: [1.0, 0.0, texture_id],
        },
        Vertex {
            position: positions[2],
            tex_coord: [1.0, 1.0, texture_id],
        },
    ]
}
fn east_face(position: Vector3<f32>, texture_id: f32) -> [Vertex; 6] {
    // [x, y, z]
    let y = position.y - 60.0;
    let positions = [
        // North bottom left (0)
        [position.z, y, position.x],
        // North top right (1)
        [position.z + 1.0, y + 1.0, position.x],
        // North bottom right(2)
        [position.z + 1.0, y, position.x],
        // North top left (3)
        [position.z, y + 1.0, position.x],
        // South top left (4)
        [position.z + 1.0, y + 1.0, position.x + 1.0],
        // South top right (5)
        [position.z, y + 1.0, position.x + 1.0],
        // South bottom left (6)
        [position.z + 1.0, y, position.x + 1.0],
        // South bottom right (7)
        [position.z, y, position.x + 1.0],
    ];
    [
        //first triangle
        Vertex {
            position: positions[7],
            tex_coord: [0.0, 1.0, texture_id],
        },
        Vertex {
            position: positions[3],
            tex_coord: [1.0, 0.0, texture_id],
        },
        Vertex {
            position: positions[0],
            tex_coord: [1.0, 1.0, texture_id],
        },
        //second triangle
        Vertex {
            position: positions[7],
            tex_coord: [0.0, 1.0, texture_id],
        },
        Vertex {
            position: positions[5],
            tex_coord: [0.0, 0.0, texture_id],
        },
        Vertex {
            position: positions[3],
            tex_coord: [1.0, 0.0, texture_id],
        },
    ]
}
fn west_face(position: Vector3<f32>, texture_id: f32) -> [Vertex; 6] {
    // [x, y, z]
    let y = position.y - 60.0;
    let positions = [
        // North bottom left (0)
        [position.z, y, position.x],
        // North top right (1)
        [position.z + 1.0, y + 1.0, position.x],
        // North bottom right(2)
        [position.z + 1.0, y, position.x],
        // North top left (3)
        [position.z, y + 1.0, position.x],
        // South top left (4)
        [position.z + 1.0, y + 1.0, position.x + 1.0],
        // South top right (5)
        [position.z, y + 1.0, position.x + 1.0],
        // South bottom left (6)
        [position.z + 1.0, y, position.x + 1.0],
        // South bottom right (7)
        [position.z, y, position.x + 1.0],
    ];
    [
        //first triangle
        Vertex {
            position: positions[2],
            tex_coord: [0.0, 1.0, texture_id],
        },
        Vertex {
            position: positions[4],
            tex_coord: [1.0, 0.0, texture_id],
        },
        Vertex {
            position: positions[6],
            tex_coord: [1.0, 1.0, texture_id],
        },
        //second triangle
        Vertex {
            position: positions[2],
            tex_coord: [0.0, 1.0, texture_id],
        },
        Vertex {
            position: positions[1],
            tex_coord: [0.0, 0.0, texture_id],
        },
        Vertex {
            position: positions[4],
            tex_coord: [1.0, 0.0, texture_id],
        },
    ]
}
fn top_face(position: Vector3<f32>, texture_id: f32) -> [Vertex; 6] {
    // [x, y, z]
    let y = position.y - 60.0;
    let positions = [
        // North bottom left (0)
        [position.z, y, position.x],
        // North top right (1)
        [position.z + 1.0, y + 1.0, position.x],
        // North bottom right(2)
        [position.z + 1.0, y, position.x],
        // North top left (3)
        [position.z, y + 1.0, position.x],
        // South top left (4)
        [position.z + 1.0, y + 1.0, position.x + 1.0],
        // South top right (5)
        [position.z, y + 1.0, position.x + 1.0],
        // South bottom left (6)
        [position.z + 1.0, y, position.x + 1.0],
        // South bottom right (7)
        [position.z, y, position.x + 1.0],
    ];
    [
        //first triangle
        Vertex {
            position: positions[3],
            tex_coord: [0.0, 1.0, texture_id],
        },
        Vertex {
            position: positions[5],
            tex_coord: [0.0, 0.0, texture_id],
        },
        Vertex {
            position: positions[4],
            tex_coord: [1.0, 0.0, texture_id],
        },
        //second triangle
        Vertex {
            position: positions[3],
            tex_coord: [0.0, 1.0, texture_id],
        },
        Vertex {
            position: positions[4],
            tex_coord: [1.0, 0.0, texture_id],
        },
        Vertex {
            position: positions[1],
            tex_coord: [1.0, 1.0, texture_id],
        },
    ]
}
fn bottom_face(position: Vector3<f32>, texture_id: f32) -> [Vertex; 6] {
    // [x, y, z]
    let y = position.y - 60.0;
    let positions = [
        // North bottom left (0)
        [position.z, y, position.x],
        // North top right (1)
        [position.z + 1.0, y + 1.0, position.x],
        // North bottom right(2)
        [position.z + 1.0, y, position.x],
        // North top left (3)
        [position.z, y + 1.0, position.x],
        // South top left (4)
        [position.z + 1.0, y + 1.0, position.x + 1.0],
        // South top right (5)
        [position.z, y + 1.0, position.x + 1.0],
        // South bottom left (6)
        [position.z + 1.0, y, position.x + 1.0],
        // South bottom right (7)
        [position.z, y, position.x + 1.0],
    ];
    [
        //first triangle
        Vertex {
            position: positions[2],
            tex_coord: [0.0, 1.0, texture_id],
        },
        Vertex {
            position: positions[7],
            tex_coord: [1.0, 0.0, texture_id],
        },
        Vertex {
            position: positions[0],
            tex_coord: [1.0, 1.0, texture_id],
        },
        //second triangle
        Vertex {
            position: positions[2],
            tex_coord: [0.0, 1.0, texture_id],
        },
        Vertex {
            position: positions[6],
            tex_coord: [0.0, 0.0, texture_id],
        },
        Vertex {
            position: positions[7],
            tex_coord: [1.0, 0.0, texture_id],
        },
    ]
}
