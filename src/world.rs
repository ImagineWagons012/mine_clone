use crate::{block::Block, Vertex};

pub struct Chunk {
    pub block_data: [[[Block; 16]; 16]; 256],
}

impl Chunk {
    pub fn new() -> Self {
        let mut block_data = [[[Block::Air; 16]; 16]; 256];

        block_data[61][0][0] = Block::Grass;
        block_data[61][0][1] = Block::Grass;
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
        let mut vertices = vec![];
        for y in 0..self.block_data.len() {
            for x in 0..self.block_data[y].len() {
                for z in 0..self.block_data[y][x].len() {
                    if self.block_data[y][x][z] == Block::Air {
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

                    match top {
                        Block::Air => {
                            vertices.append(&mut top_face(x as f32, y as f32, z as f32, 0.0).to_vec());
                        },
                        _ => {}
                    }
                    match bottom {
                        Block::Air => {
                            vertices.append(&mut bottom_face(x as f32, y as f32, z as f32, 1.0).to_vec());
                        },
                        _ => {}
                    }
                    match north {
                        Block::Air => {
                            vertices.append(&mut north_face(x as f32, y as f32, z as f32, 0.5).to_vec());
                        },
                        _ => {}
                    }
                    match south {
                        Block::Air => {
                            vertices.append(&mut south_face(x as f32, y as f32, z as f32, 0.5).to_vec());
                        },
                        _ => {}
                    }
                    match east {
                        Block::Air => {
                            vertices.append(&mut east_face(x as f32, y as f32, z as f32, 0.5).to_vec());
                        },
                        _ => {}
                    }
                    match west {
                        Block::Air => {
                            vertices.append(&mut west_face(x as f32, y as f32, z as f32, 0.5).to_vec());
                        },
                        _ => {}
                    }
                }
            }
        }
        vertices
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

fn generate_mesh(vertices: &mut Vec<Vertex>, rows: f32, columns: f32, height: f32) {
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
