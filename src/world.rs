use cgmath::Vector3;

use crate::{block::Block, Vertex, Instance};

pub struct Chunk {
    pub block_data: [[[Block;16];16];256],
}

impl Chunk {
    pub fn new() -> Self {
        let mut block_data = [[[Block::Air;16];16];256];

        block_data[60] = [[Block::Grass;16];16];

        Self { block_data }
    }
    pub fn generate_instances(&self) -> Vec<crate::Instance> {

        (0..16).flat_map(|z| {
            (0..16).map(move |x| {
                let position = cgmath::Vector3 { x: x as f32, y: 0.0, z: z as f32 } - Vector3::new(8.0, 0.0, 8.0);

                let rotation = cgmath::Quaternion::new(0.0, 0.0, 0.0, 0.0);

                Instance {
                    position, rotation,
                }
            })
        }).collect::<Vec<_>>()
    }
    pub fn generate_mesh(&self) -> Vec<Vertex> {
        let mut vertices = vec![];
        
        let positions = vec![
            // North bottom left (0)
            [0.0, 0.0, 0.0],
            // North top right (1)
            [1.0, 1.0, 0.0], 
            // North bottom right(2)
            [1.0, 0.0, 0.0], 
            // North top left (3)
            [0.0, 1.0, 0.0],
            // South top left (4)
            [1.0, 1.0, -1.0], 
            // South top right (5)
            [0.0, 1.0, -1.0],
            // South bottom left (6)
            [1.0, 0.0, -1.0],
            // South bottom right (7)
            [0.0, 0.0, -1.0],
        ];

        // 2, 0, 7     2, 7, 6

        
        
        // North facing face
        vertices.push(Vertex {
            position: positions[0],
            tex_coord: [0.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[1],
            tex_coord: [1.0, 0.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[3],
            tex_coord: [0.0, 0.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[0],
            tex_coord: [0.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[2],
            tex_coord: [1.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[1],
            tex_coord: [1.0, 0.0, 0.0],
        });
        
        //south facing face
        vertices.push(Vertex {
            position: positions[6],
            tex_coord: [0.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[7],
            tex_coord: [1.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[5],
            tex_coord: [1.0, 0.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[6],
            tex_coord: [0.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[5],
            tex_coord: [1.0, 0.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[4],
            tex_coord: [0.0, 0.0, 0.0],
        });

        // East face
        vertices.push(Vertex {
            position: positions[7],
            tex_coord: [0.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[0],
            tex_coord: [1.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[3],
            tex_coord: [1.0, 0.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[7],
            tex_coord: [0.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[3],
            tex_coord: [1.0, 0.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[5],
            tex_coord: [0.0, 0.0, 0.0],
        });


        // West face
        vertices.push(Vertex {
            position: positions[2],
            tex_coord: [0.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[6],
            tex_coord: [1.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[4],
            tex_coord: [1.0, 0.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[2],
            tex_coord: [0.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[4],
            tex_coord: [1.0, 0.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[1],
            tex_coord: [0.0, 0.0, 0.0],
        });


        // Bottom face
        vertices.push(Vertex {
            position: positions[2],
            tex_coord: [0.0, 1.0, -1.0],
        });
        vertices.push(Vertex {
            position: positions[0],
            tex_coord: [1.0, 1.0, -1.0],
        });
        vertices.push(Vertex {
            position: positions[7],
            tex_coord: [1.0, 0.0, -1.0],
        });
        vertices.push(Vertex {
            position: positions[2],
            tex_coord: [0.0, 1.0, -1.0],
        });
        vertices.push(Vertex {  
            position: positions[7],
            tex_coord: [1.0, 0.0, -1.0],
        });
        vertices.push(Vertex {
            position: positions[6],
            tex_coord: [0.0, 0.0, -1.0],
        });

        // Top face
        vertices.push(Vertex {
            position: positions[3],
            tex_coord: [0.0, 1.0, 1.0],
        });
        vertices.push(Vertex {
            position: positions[4],
            tex_coord: [1.0, 0.0, 1.0],
        });
        vertices.push(Vertex {
            position: positions[5],
            tex_coord: [0.0, 0.0, 1.0],
        });
        vertices.push(Vertex {
            position: positions[3],
            tex_coord: [0.0, 1.0, 1.0],
        });
        vertices.push(Vertex {
            position: positions[1],
            tex_coord: [1.0, 1.0, 1.0],
        });
        vertices.push(Vertex {
            position: positions[4],
            tex_coord: [1.0, 0.0, 1.0],
        });
        vertices
    }
}