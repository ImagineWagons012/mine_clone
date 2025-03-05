use std::collections::HashMap;

use crate::{block::{get_block_texture_ids, Block}, texture::TextureManager, Vertex};
use rand::Rng;

#[derive(Debug)]
pub struct Chunk {
    pub block_data: [[[Block; 16]; 16]; 256],
}

impl Chunk {
    pub fn new() -> Self {
        let mut block_data = [[[Block::Air; 16]; 16]; 256];
        for plane in &mut block_data {
            let mut rng = rand::rng();
            plane[rng.random_range(0..16)][rng.random_range(0..16)] = Block::Grass;
        }
        Self { block_data }
    }
    pub fn generate_mesh(&self, texture_manager: &TextureManager) -> Vec<Vertex> {
        let start = std::time::Instant::now();

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

                    if let None = texture_id_cache.get(&current) {
                        let textures = get_block_texture_ids(current, texture_manager);
    
                        match top {
                            Block::Air => {
                                let texture_id = textures.get(&crate::Cardinal::Up).unwrap();
                                vertices.append(&mut top_face(x as f32, y as f32, z as f32, *texture_id).to_vec());
                            },
                            _ => {}
                        }
                        match bottom {
                            Block::Air => {
                                let texture_id = textures.get(&crate::Cardinal::Down).unwrap();
                                vertices.append(&mut bottom_face(x as f32, y as f32, z as f32, *texture_id).to_vec());
                            },
                            _ => {}
                        }
                        match north {
                            Block::Air => {
                                let texture_id = textures.get(&crate::Cardinal::North).unwrap();
                                vertices.append(&mut north_face(x as f32, y as f32, z as f32, *texture_id).to_vec());
                            },
                            _ => {}
                        }
                        match south {
                            Block::Air => {
                                let texture_id = textures.get(&crate::Cardinal::South).unwrap();
                                vertices.append(&mut south_face(x as f32, y as f32, z as f32, *texture_id).to_vec());
                            },
                            _ => {}
                        }
                        match east {
                            Block::Air => {
                                let texture_id = textures.get(&crate::Cardinal::East).unwrap();
                                vertices.append(&mut east_face(x as f32, y as f32, z as f32, *texture_id).to_vec());
                            },
                            _ => {}
                        }
                        match west {
                            Block::Air => {
                                let texture_id = textures.get(&crate::Cardinal::West).unwrap();
                                vertices.append(&mut west_face(x as f32, y as f32, z as f32, *texture_id).to_vec());
                            },
                            _ => {}
                        }
                        texture_id_cache.insert(current, textures);
                    }
                    else {
                        let textures = texture_id_cache.get(&current).unwrap();
                        match top {
                            Block::Air => {
                                let texture_id = textures.get(&crate::Cardinal::Up).unwrap();
                                vertices.append(&mut top_face(x as f32, y as f32, z as f32, *texture_id).to_vec());
                            },
                            _ => {}
                        }
                        match bottom {
                            Block::Air => {
                                let texture_id = textures.get(&crate::Cardinal::Down).unwrap();
                                vertices.append(&mut bottom_face(x as f32, y as f32, z as f32, *texture_id).to_vec());
                            },
                            _ => {}
                        }
                        match north {
                            Block::Air => {
                                let texture_id = textures.get(&crate::Cardinal::North).unwrap();
                                vertices.append(&mut north_face(x as f32, y as f32, z as f32, *texture_id).to_vec());
                            },
                            _ => {}
                        }
                        match south {
                            Block::Air => {
                                let texture_id = textures.get(&crate::Cardinal::South).unwrap();
                                vertices.append(&mut south_face(x as f32, y as f32, z as f32, *texture_id).to_vec());
                            },
                            _ => {}
                        }
                        match east {
                            Block::Air => {
                                let texture_id = textures.get(&crate::Cardinal::East).unwrap();
                                vertices.append(&mut east_face(x as f32, y as f32, z as f32, *texture_id).to_vec());
                            },
                            _ => {}
                        }
                        match west {
                            Block::Air => {
                                let texture_id = textures.get(&crate::Cardinal::West).unwrap();
                                vertices.append(&mut west_face(x as f32, y as f32, z as f32, *texture_id).to_vec());
                            },
                            _ => {}
                        }
                    }

                }
            }
        }
        println!("{:?}", start.elapsed().as_secs_f32());
        vertices
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self { block_data: [[[Block::default(); 16]; 16]; 256] }
    }
}

fn north_face(x: f32, y: f32, z: f32, texture_id: f32) -> [Vertex; 6] {
    // [x, y, z]
    
    let y = y - 60.0;
    let positions = [
        // North bottom left (0)
        [z, y, x],
        // North top right (1)
        [z + 1.0, y + 1.0, x],
        // North bottom right(2)
        [z + 1.0, y, x],
        // North top left (3)
        [z, y + 1.0, x],
        // South top left (4)
        [z + 1.0, y + 1.0, x + 1.0],
        // South top right (5)
        [z, y + 1.0, x + 1.0],
        // South bottom left (6)
        [z + 1.0, y, x + 1.0],
        // South bottom right (7)
        [z, y, x + 1.0],
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
fn south_face(x: f32, y: f32, z: f32, texture_id: f32) -> [Vertex; 6] {
    // [x, y, z]
    let y = y - 60.0;
    let positions = [
        // North bottom left (0)
        [z, y, x],
        // North top right (1)
        [z + 1.0, y + 1.0, x],
        // North bottom right(2)
        [z + 1.0, y, x],
        // North top left (3)
        [z, y + 1.0, x],
        // South top left (4)
        [z + 1.0, y + 1.0, x + 1.0],
        // South top right (5)
        [z, y + 1.0, x + 1.0],
        // South bottom left (6)
        [z + 1.0, y, x + 1.0],
        // South bottom right (7)
        [z, y, x + 1.0],
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
fn east_face(x: f32, y: f32, z: f32, texture_id: f32) -> [Vertex; 6] {
    // [x, y, z]
    let y = y - 60.0;
    let positions = [
        // North bottom left (0)
        [z, y, x],
        // North top right (1)
        [z + 1.0, y + 1.0, x],
        // North bottom right(2)
        [z + 1.0, y, x],
        // North top left (3)
        [z, y + 1.0, x],
        // South top left (4)
        [z + 1.0, y + 1.0, x + 1.0],
        // South top right (5)
        [z, y + 1.0, x + 1.0],
        // South bottom left (6)
        [z + 1.0, y, x + 1.0],
        // South bottom right (7)
        [z, y, x + 1.0],
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
fn west_face(x: f32, y: f32, z: f32, texture_id: f32) -> [Vertex; 6] {
    // [x, y, z]
    let y = y - 60.0;
    let positions = [
        // North bottom left (0)
        [z, y, x],
        // North top right (1)
        [z + 1.0, y + 1.0, x],
        // North bottom right(2)
        [z + 1.0, y, x],
        // North top left (3)
        [z, y + 1.0, x],
        // South top left (4)
        [z + 1.0, y + 1.0, x + 1.0],
        // South top right (5)
        [z, y + 1.0, x + 1.0],
        // South bottom left (6)
        [z + 1.0, y, x + 1.0],
        // South bottom right (7)
        [z, y, x + 1.0],
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
fn top_face(x: f32, y: f32, z: f32, texture_id: f32) -> [Vertex; 6] {
    // [x, y, z]
    let y = y - 60.0;
    let positions = [
        // North bottom left (0)
        [z, y, x],
        // North top right (1)
        [z + 1.0, y + 1.0, x],
        // North bottom right(2)
        [z + 1.0, y, x],
        // North top left (3)
        [z, y + 1.0, x],
        // South top left (4)
        [z + 1.0, y + 1.0, x + 1.0],
        // South top right (5)
        [z, y + 1.0, x + 1.0],
        // South bottom left (6)
        [z + 1.0, y, x + 1.0],
        // South bottom right (7)
        [z, y, x + 1.0],
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
fn bottom_face(x: f32, y: f32, z: f32, texture_id: f32) -> [Vertex; 6] {
    // [x, y, z]
    let y = y - 60.0;
    let positions = [
        // North bottom left (0)
        [z, y, x],
        // North top right (1)
        [z + 1.0, y + 1.0, x],
        // North bottom right(2)
        [z + 1.0, y, x],
        // North top left (3)
        [z, y + 1.0, x],
        // South top left (4)
        [z + 1.0, y + 1.0, x + 1.0],
        // South top right (5)
        [z, y + 1.0, x + 1.0],
        // South bottom left (6)
        [z + 1.0, y, x + 1.0],
        // South bottom right (7)
        [z, y, x + 1.0],
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
