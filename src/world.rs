use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    block::{get_block_texture_ids, Block},
    texture::TextureManager,
    Cardinal, Vertex,
};
use cgmath::{Vector2, Vector3};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use wgpu::util::DeviceExt;

#[derive(Debug)]
pub struct Chunk {
    block_data: [[[Block; 16]; 16]; 256],
    position: Vector2<f32>,
    buffer: Option<(Arc<wgpu::Buffer>, usize)>,
    buffers_created: u32,
}

impl Chunk {
    pub fn new<T: Into<Vector2<f32>>>(position: T) -> Self {
        let block_data = [[[Block::Air; 16]; 16]; 256];
        Self {
            block_data,
            position: position.into(),
            buffer: None,
            buffers_created: 0
        }
    }


    pub fn get_side_blocks(&self, side: Cardinal) -> Box<[[Block; 16]; 256]> {
        let mut blocks = Box::new([[Block::Air; 16]; 256]);
        match side {
            Cardinal::North => {
                for (i, plane) in self.block_data.iter().enumerate() {
                    blocks[i] = *plane.last().unwrap();
                }
            }
            Cardinal::South => {
                for (i, plane) in self.block_data.iter().enumerate() {
                    blocks[i] = plane[0];
                }
            }
            Cardinal::East => {
                for (i, plane) in self.block_data.iter().enumerate() {
                    for (j, row) in plane.iter().enumerate() {
                        blocks[i][j] = row[row.len() - 1];
                    }
                }
            }
            Cardinal::West => {
                for (i, plane) in self.block_data.iter().enumerate() {
                    for (j, row) in plane.iter().enumerate() {
                        blocks[i][j] = row[0];
                    }
                }
            }
            _ => {}
        }
        blocks
    }
    pub async fn generate_mesh(
        &mut self,
        texture_manager: Arc<TextureManager>,
        side_blocks: &[[[Block; 16]; 256]; 4],
        device: &wgpu::Device
    ) -> (Arc<wgpu::Buffer>, usize) {
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
                    let north;
                    let south;
                    let east;
                    let west;
                    if y < 255 {
                        top = self.block_data[y + 1][x][z];
                    }
                    if y > 0 {
                        bottom = self.block_data[y - 1][x][z];
                    }

                    if x < 15 {
                        north = self.block_data[y][x + 1][z];
                    } else {
                        north = side_blocks[0][y][z];
                        // println!("north {:?}", side_blocks[0][y][z]);
                    }

                    if x > 0 {
                        south = self.block_data[y][x - 1][z];
                    } else {
                        south = side_blocks[1][y][z];
                        // println!("south {:?}", side_blocks[1][y][z]);
                    }

                    if z > 0 {
                        east = self.block_data[y][x][z - 1];
                    } else {
                        east = side_blocks[2][y][x];
                        // println!("east {:?}", side_blocks[2][y][x]);
                    }

                    if z < 15 {
                        west = self.block_data[y][x][z + 1];
                    } else {
                        west = side_blocks[3][y][x];
                        // println!("west {:?}", side_blocks[3][y][x]);
                    }

                    let position: Vector3<f32> = (
                        x as f32 + self.position.x * 16.0,
                        y as f32,
                        z as f32 + self.position.y * 16.0,
                    )
                        .into();

                    if let None = texture_id_cache.get(&current) {
                        let textures = get_block_texture_ids(current, texture_manager.clone()).await;
                        texture_id_cache.insert(current, textures);
                    }
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

        self.buffers_created += 1;
        self.buffer = Some((Arc::new(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("vertex_buffer, x: {}, z: {}", self.position.x, self.position.y).as_str()),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX
        })), vertices.len())); 
        self.buffer.as_ref().unwrap().clone()
        
    }
    pub async fn get_or_generate_mesh(&mut self, texture_manager: Arc<TextureManager>, side_blocks: &[[[Block; 16]; 256]; 4], device: &wgpu::Device) -> (Arc<wgpu::Buffer>, usize) {
        match self.buffer.as_ref() {
            Some((buffer, len)) => {
                (buffer.clone(), *len)
            }
            None => {
                self.generate_mesh(texture_manager, side_blocks, device).await
            }
        }
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            block_data: [[[Block::default(); 16]; 16]; 256],
            position: (0.0, 0.0).into(),
            buffer: None,
            buffers_created: 0
        }
    }
}

pub struct World {
    pub chunks: HashMap<(i64, i64), Chunk>,
    seed: [u8; 32],
    seed_string: String,
    pub render_distance: u32,
    buffers_created: u32
}

impl World {
    pub fn new(seed: String, render_distance: u32) -> Self {
        let seed_string = seed.clone();
        let mut rng = rand_seeder::SipHasher::from(seed).into_rng();
        let mut seed = [0; 32];
        rng.fill(&mut seed);

        Self {
            chunks: HashMap::new(),
            seed,
            seed_string,
            render_distance,
            buffers_created: 0
        }
    }
    
    pub async fn generate_mesh(
        texture_manager: Arc<TextureManager>, 
        device: Arc<wgpu::Device>, 
        world: Arc<Mutex<World>>,
        cam_pos: (f32, f32)
    ) -> Vec<(Arc<wgpu::Buffer>, usize)> {
        log::info!("mesh gen: waiting on world lock");
        let mut world_lock = world.lock().await;
        log::info!("mesh gen: got world lock");
        let base_x = cam_pos.0 / 16.0;
        let base_z = cam_pos.1 / 16.0;
        let mut buffers = vec![];
        
        let mut side_blocks = [[[Block::Stone; 16]; 256]; 4];
        for i in (base_x as i32 - (world_lock.render_distance + 10) as i32)..(base_x as i32 + (world_lock.render_distance + 10) as i32) {
            for j in (base_z as i32 - (world_lock.render_distance + 10) as i32)..(base_z as i32 + (world_lock.render_distance + 10) as i32) {
                if (i as f32 - base_x).powf(2.0) + (j as f32 - base_z).powf(2.0) > (world_lock.render_distance as f32).powf(2.0) {
                    continue;
                }
                if (i as f32 - base_x).powf(2.0) + (j as f32 - base_z).powf(2.0) > ((world_lock.render_distance + 5) as f32).powf(2.0) {
                    let chunk = world_lock.get_chunk_mut(Vector2::new(j as f32, i as f32));
                    chunk.buffer.as_ref().unwrap().0.destroy();
                    chunk.buffer = None;
                    
                }
                let position = Vector2::new(j as f32, i as f32);
                // North side
                let position2 = position + Vector2::unit_x();
                let chunk = world_lock.get_chunk(position2);
                side_blocks[0] = *chunk.get_side_blocks(Cardinal::South);
                
                // South side
                let position2 = position - Vector2::unit_x();
                let chunk = world_lock.get_chunk(position2);
                side_blocks[1] = *chunk.get_side_blocks(Cardinal::North);
                
                // East side
                let position2 = position + Vector2::unit_y();
                let chunk = world_lock.get_chunk(position2);
                side_blocks[3] = *chunk.get_side_blocks(Cardinal::West);
                
                // West side
                let position2 = position - Vector2::unit_y();
                let chunk = world_lock.get_chunk(position2);
                side_blocks[2] = *chunk.get_side_blocks(Cardinal::East);

                let buffer_num = world_lock.get_chunk_mut(position).get_or_generate_mesh(texture_manager.clone(), &side_blocks, &device).await;
                buffers.push(buffer_num);
           }
        }
        world_lock.buffers_created = world_lock.chunks.iter().map(|x| x.1.buffers_created).sum();
        log::info!("buffers created: {}", world_lock.buffers_created);
        log::info!("returning mesh");
        return buffers;
    }

    fn is_chunk_available(&self, position: &(i64, i64)) -> bool {
        match self.chunks.get(position) {
            Some(_) => {
                true
            }
            None => {
                false
            }
        }
    }
    fn get_chunk(&mut self, position: Vector2<f32>) -> &Chunk {
        let i_position = (position.x as i64, position.y as i64);
        if self.is_chunk_available(&i_position) {
            return self.chunks.get(&i_position).unwrap();
        }
        else {
            self.generate_chunk(position);
            return self.chunks.get(&i_position).unwrap();
        }
    }
    fn get_chunk_mut(&mut self, position: Vector2<f32>) -> &mut Chunk {
        let i_position = (position.x as i64, position.y as i64);
        if self.is_chunk_available(&i_position) {
            return self.chunks.get_mut(&i_position).unwrap();
        }
        else {
            self.generate_chunk(position);
            return self.chunks.get_mut(&i_position).unwrap();
        }
    }   
    pub fn generate_chunk(&mut self, at_position: Vector2<f32>) {
        let perlin = Perlin::new(self.seed[0] as u32);
        let mut chunk = Chunk::new(at_position);
        for x in 0..16 {
            for z in 0..16 {
                let y =
                    10.0 * perlin.get([
                        3.0e-1 * (x as f32 / 16.0 + chunk.position.x) as f64,
                        3.0e-1 * (z as f32 / 16.0 + chunk.position.y) as f64,
                    ]) 
                    + 50.0
                        * perlin.get([
                            5.0e-2 * (x as f32 / 16.0 + chunk.position.x) as f64,
                            5.0e-2 * (z as f32 / 16.0 + chunk.position.y) as f64,
                        ])
                    + 75.0
                        * perlin.get([
                            5.0e-3 * (x as f32 / 16.0 + chunk.position.x) as f64,
                            5.0e-3 * (z as f32 / 16.0 + chunk.position.y) as f64,
                        ])
                    + 100.0
                        * perlin.get([
                            5.0e-7 * (x as f32 / 16.0 + chunk.position.x) as f64,
                            5.0e-7 * (z as f32 / 16.0 + chunk.position.y) as f64,
                        ]);
                            
                // println!("{:?}", y);
                let y = (y / (10.0 + 50.0 + 75.0 + 100.0) + 1.0) / 2.0;
                let y = (y).powf(2.5);
                let y = (y * 190.0 + 20.0).floor().clamp(0.0, 255.0);
                let y = y as usize;
                chunk.block_data[y][x][z] = Block::Grass;
                for height in (0..y).rev() {
                    chunk.block_data[height][x][z] = Block::Dirt;
                    if y < 4 {
                        chunk.block_data[height][x][z] = Block::Stone;
                    } else if height < y - 4 {
                        chunk.block_data[height][x][z] = Block::Stone;
                    }
                }
            }
        }
        chunk.block_data[0] = [[Block::Bedrock; 16]; 16];
        let position = (at_position.x as i64, at_position.y as i64);
        self.chunks.insert(position,chunk);
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
