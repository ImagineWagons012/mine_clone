use crate::{block::Block, Vertex};

pub struct Chunk {
    pub block_data: [[[Block; 16];16];256],
}

impl Chunk {
    pub fn new() -> Self {
        let mut block_data = [[[Block::Air;16];16];256];
        // let mut rng = rand::rng();
        // for plane in block_data.iter_mut() {
        //     for _ in 0..30 {
        //         let (x, z) = (rng.random_range(0..16), rng.random_range(0..16) as usize);
        //         plane[x][z] = Block::Grass;
        //     }
        // }

        
        block_data[60][0][0] = Block::Grass;
        block_data[60][0][1] = Block::Grass;
        block_data[60][1][0] = Block::Grass;
        block_data[60][1][1] = Block::Grass;
        block_data[60][2][0] = Block::Grass;
        block_data[60][2][1] = Block::Grass;
        block_data[60][3][0] = Block::Grass;
        block_data[60][3][1] = Block::Grass;


        Self { block_data }
    }
    pub fn generate_meshes(&self) -> Vec<Vertex> {

        // merge rows of blocks, and merge columns of rows

        let mut vertices = vec![];
        let mut columns = 0;
        let mut rows = 0;
        for (height, plane) in self.block_data.iter().enumerate() {
            if plane != &[[Block::Air;16];16] {
                let search_for = Block::Grass;
                for (i, row) in plane.iter().enumerate() {
                    for (j, block) in row.iter().enumerate() {
                        if *block == search_for {
                            columns = j + 1;
                        }
                        else {
                            break;
                        }
                    }
                    if row[0..columns].iter().all(|x| *x == search_for) {
                        rows = i + 1;
                    }
                    else {
                        break;
                    }
                }
                generate_mesh(&mut vertices, rows as f32, columns as f32, height as f32 - 60.0);
            }
        }
        vertices
    }
}

fn generate_mesh(vertices:&mut Vec<Vertex>, rows: f32, columns: f32, height: f32) {

    let positions = vec![
            // North bottom left (0)
            [0.0, height, 0.0],
            // North top right (1)
            [rows, height + 1.0, 0.0], 
            // North bottom right(2)
            [rows, height, 0.0], 
            // North top left (3)
            [0.0, height + 1.0, 0.0],
            // South top left (4)
            [rows, height + 1.0, -columns], 
            // South top right (5)
            [0.0, height + 1.0, -columns],
            // South bottom left (6)
            [rows, height, -columns],
            // South bottom right (7)
            [0.0, height, -columns],
        ];

        // // 2, 0, 7     2, 7, 6

        
        
        // North face
        vertices.push(Vertex {
            position: positions[0],
            tex_coord: [0.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[1],
            tex_coord: [rows, 0.0, 0.0],
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
            tex_coord: [rows, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[1],
            tex_coord: [rows, 0.0, 0.0],
        });
        
        //South face
        vertices.push(Vertex {
            position: positions[6],
            tex_coord: [0.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[7],
            tex_coord: [rows, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[5],
            tex_coord: [rows, 0.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[6],
            tex_coord: [0.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[5],
            tex_coord: [rows, 0.0, 0.0],
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
            tex_coord: [columns, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[3],
            tex_coord: [columns, 0.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[7],
            tex_coord: [0.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[3],
            tex_coord: [columns, 0.0, 0.0],
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
            tex_coord: [columns, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[4],
            tex_coord: [columns, 0.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[2],
            tex_coord: [0.0, 1.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[4],
            tex_coord: [columns, 0.0, 0.0],
        });
        vertices.push(Vertex {
            position: positions[1],
            tex_coord: [0.0, 0.0, 0.0],
        });


        // Bottom face
        vertices.push(Vertex {
            position: positions[2],
            tex_coord: [0.0, columns, -1.0],
        });
        vertices.push(Vertex {
            position: positions[0],
            tex_coord: [rows, columns, -1.0],
        });
        vertices.push(Vertex {
            position: positions[7],
            tex_coord: [rows, 0.0, -1.0],
        });
        vertices.push(Vertex {
            position: positions[2],
            tex_coord: [0.0, columns, -1.0],
        });
        vertices.push(Vertex {  
            position: positions[7],
            tex_coord: [rows, 0.0, -1.0],
        });
        vertices.push(Vertex {
            position: positions[6],
            tex_coord: [0.0, 0.0, -1.0],
        });

        // Top face
        vertices.push(Vertex {
            position: positions[3],
            tex_coord: [0.0, columns, 1.0],
        });
        vertices.push(Vertex {
            position: positions[4],
            tex_coord: [rows, 0.0, 1.0],
        });
        vertices.push(Vertex {
            position: positions[5],
            tex_coord: [0.0, 0.0, 1.0],
        });
        vertices.push(Vertex {
            position: positions[3],
            tex_coord: [0.0, columns, 1.0],
        });
        vertices.push(Vertex {
            position: positions[1],
            tex_coord: [rows, columns, 1.0],
        });
        vertices.push(Vertex {
            position: positions[4],
            tex_coord: [rows, 0.0, 1.0],
        });
}