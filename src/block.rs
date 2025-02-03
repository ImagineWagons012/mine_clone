use wgpu::{Device, util::DeviceExt};

use cgmath::Vector3;

use crate::texture::Texture;
use crate::Vertex;

const VERTICES: &[Vertex] = &[
    // bottom right
    Vertex {
        position: [0.5, -0.5, 0.0],
        tex_coord: [1.0, 1.0],
    },
    // top right
    Vertex {
        position: [0.5, 0.5, 0.0],
        tex_coord: [1.0, 0.0],
    },
    // bottom left
    Vertex {
        position: [-0.5, -0.5, 0.0],
        tex_coord: [0.0, 1.0],
    },
    // bottom left
    Vertex {
        position: [-0.5, -0.5, 0.0],
        tex_coord: [0.0, 1.0],
    },
    // top right
    Vertex {
        position: [0.5, 0.5, 0.0],
        tex_coord: [1.0, 0.0],
    },
    // top left
    Vertex {
        position: [-0.5, 0.5, 0.0],
        tex_coord: [0.0, 0.0],
    },
];


pub struct Block {
    pos: Vector3<f32>,
    texture: Texture,
    vertex_buffer: wgpu::Buffer,
}

impl Block {
    pub fn new(device: &Device, pos: Vector3<f32>, texture: Texture) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });


        Block { pos, texture: texture, vertex_buffer }
    }
}