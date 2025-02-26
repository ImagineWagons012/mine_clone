use crate::{block::Block, Vertex};

pub struct Chunk {
    pub block_data: [[[Block; 16]; 16]; 256],
}

impl Chunk {
    pub fn new() -> Self {
        let mut block_data = [[[Block::Air; 16]; 16]; 256];

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
        let mut vertices = north_face(0.0, 0.0, 0.0).to_vec();
        for vertex in south_face(0.0, 0.0, 0.0) {
            vertices.push(vertex);
        }
        for vertex in east_face(0.0, 0.0, 0.0) {
            vertices.push(vertex);
        }
        for vertex in west_face(0.0, 0.0, 0.0) {
            vertices.push(vertex);
        }
        for vertex in top_face(0.0, 0.0, 0.0) {
            vertices.push(vertex);
        }
        for vertex in bottom_face(0.0, 0.0, 0.0) {
            vertices.push(vertex);
        }
        vertices
    }
}

fn north_face(x: f32, y: f32, z: f32) -> [Vertex; 6] {
    // [x, y, z]
    let positions = vec![
        // North bottom left (0)
        [x, y, z],
        // North top right (1)
        [x + 1.0, y + 1.0, z],
        // North bottom right(2)
        [x + 1.0, y, z],
        // North top left (3)
        [x, y + 1.0, z],
        // South top left (4)
        [x + 1.0, y + 1.0, z + 1.0],
        // South top right (5)
        [x, y + 1.0, z + 1.0],
        // South bottom left (6)
        [x + 1.0, y, z + 1.0],
        // South bottom right (7)
        [x, y, z + 1.0],
    ];
    [
        //first triangle
        Vertex {
            position: positions[6],
            tex_coord: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: positions[5],
            tex_coord: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: positions[7],
            tex_coord: [1.0, 1.0, 0.0],
        },

        // second triangle
        Vertex {
            position: positions[6],
            tex_coord: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: positions[4],
            tex_coord: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: positions[5],
            tex_coord: [1.0, 0.0, 0.0],
        },
    ]
}
fn south_face(x: f32, y: f32, z: f32) -> [Vertex; 6] {
    // [x, y, z]
    let positions = vec![
        // North bottom left (0)
        [x, y, z],
        // North top right (1)
        [x + 1.0, y + 1.0, z],
        // North bottom right(2)
        [x + 1.0, y, z],
        // North top left (3)
        [x, y + 1.0, z],
        // South top left (4)
        [x + 1.0, y + 1.0, z + 1.0],
        // South top right (5)
        [x, y + 1.0, z + 1.0],
        // South bottom left (6)
        [x + 1.0, y, z + 1.0],
        // South bottom right (7)
        [x, y, z + 1.0],
    ];
    [
        //first triangle
        Vertex {
            position: positions[0],
            tex_coord: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: positions[3],
            tex_coord: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: positions[1],
            tex_coord: [1.0, 0.0, 0.0],
        },

        // second triangle
        Vertex {
            position: positions[0],
            tex_coord: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: positions[1],
            tex_coord: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: positions[2],
            tex_coord: [1.0, 1.0, 0.0],
        },
    ]
}
fn east_face(x: f32, y: f32, z: f32) -> [Vertex; 6] {
    // [x, y, z]
    let positions = vec![
        // North bottom left (0)
        [x, y, z],
        // North top right (1)
        [x + 1.0, y + 1.0, z],
        // North bottom right(2)
        [x + 1.0, y, z],
        // North top left (3)
        [x, y + 1.0, z],
        // South top left (4)
        [x + 1.0, y + 1.0, z + 1.0],
        // South top right (5)
        [x, y + 1.0, z + 1.0],
        // South bottom left (6)
        [x + 1.0, y, z + 1.0],
        // South bottom right (7)
        [x, y, z + 1.0],
    ];
    [
        //first triangle
        Vertex {
            position: positions[7],
            tex_coord: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: positions[3],
            tex_coord: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: positions[0],
            tex_coord: [1.0, 1.0, 0.0],
        },

        //second triangle
        Vertex {
            position: positions[7],
            tex_coord: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: positions[5],
            tex_coord: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: positions[3],
            tex_coord: [1.0, 0.0, 0.0],
        },
    ]
}
fn west_face(x: f32, y: f32, z: f32) -> [Vertex; 6] {
    // [x, y, z]
    let positions = vec![
        // North bottom left (0)
        [x, y, z],
        // North top right (1)
        [x + 1.0, y + 1.0, z],
        // North bottom right(2)
        [x + 1.0, y, z],
        // North top left (3)
        [x, y + 1.0, z],
        // South top left (4)
        [x + 1.0, y + 1.0, z + 1.0],
        // South top right (5)
        [x, y + 1.0, z + 1.0],
        // South bottom left (6)
        [x + 1.0, y, z + 1.0],
        // South bottom right (7)
        [x, y, z + 1.0],
    ];
    [
        //first triangle
        Vertex {
            position: positions[2],
            tex_coord: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: positions[4],
            tex_coord: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: positions[6],
            tex_coord: [1.0, 1.0, 0.0],
        },
        //second triangle
        Vertex {
            position: positions[2],
            tex_coord: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: positions[1],
            tex_coord: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: positions[4],
            tex_coord: [1.0, 0.0, 0.0],
        },
    ]
}
fn top_face(x: f32, y: f32, z: f32) -> [Vertex; 6] {
    // [x, y, z]
    let positions = vec![
        // North bottom left (0)
        [x, y, z],
        // North top right (1)
        [x + 1.0, y + 1.0, z],
        // North bottom right(2)
        [x + 1.0, y, z],
        // North top left (3)
        [x, y + 1.0, z],
        // South top left (4)
        [x + 1.0, y + 1.0, z + 1.0],
        // South top right (5)
        [x, y + 1.0, z + 1.0],
        // South bottom left (6)
        [x + 1.0, y, z + 1.0],
        // South bottom right (7)
        [x, y, z + 1.0],
    ];
    [
        //first triangle
        Vertex {
            position: positions[3],
            tex_coord: [0.0, 1.0, 1.0],
        },
        Vertex {
            position: positions[5],
            tex_coord: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: positions[4],
            tex_coord: [1.0, 0.0, 1.0],
        },

        //second triangle
        Vertex {
            position: positions[3],
            tex_coord: [0.0, 1.0, 1.0],
        },
        Vertex {
            position: positions[4],
            tex_coord: [1.0, 0.0, 1.0],
        },
        Vertex {
            position: positions[1],
            tex_coord: [1.0, 1.0, 1.0],
        },
    ]
}
fn bottom_face(x: f32, y: f32, z: f32) -> [Vertex; 6] {
    // [x, y, z]
    let positions = vec![
        // North bottom left (0)
        [x, y, z],
        // North top right (1)
        [x + 1.0, y + 1.0, z],
        // North bottom right(2)
        [x + 1.0, y, z],
        // North top left (3)
        [x, y + 1.0, z],
        // South top left (4)
        [x + 1.0, y + 1.0, z + 1.0],
        // South top right (5)
        [x, y + 1.0, z + 1.0],
        // South bottom left (6)
        [x + 1.0, y, z + 1.0],
        // South bottom right (7)
        [x, y, z + 1.0],
    ];
    [
        //first triangle
        Vertex {
            position: positions[2],
            tex_coord: [0.0, 1.0, -1.0],
        },
        Vertex {
            position: positions[7],
            tex_coord: [1.0, 0.0, -1.0],
        },
        Vertex {
            position: positions[0],
            tex_coord: [1.0, 1.0, -1.0],
        },

        //second triangle
        Vertex {
            position: positions[2],
            tex_coord: [0.0, 1.0, -1.0],
        },
        Vertex {
            position: positions[6],
            tex_coord: [0.0, 0.0, -1.0],
        },
        Vertex {
            position: positions[7],
            tex_coord: [1.0, 0.0, -1.0],
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
