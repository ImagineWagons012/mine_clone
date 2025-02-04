use wgpu::{Device, util::DeviceExt};

use cgmath::{vec3, Vector3};

use crate::texture::Texture;
use crate::Vertex;

pub const VERTICES: &[Vertex] = &[
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

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct TransformUnifrom {
    trans: [[f32; 4]; 4],
}

pub struct Block {
    pos: Vector3<f32>,
    texture: Texture,
    pub vertex_buffer: wgpu::Buffer,
    pub block_uniform: TransformUnifrom,
}

impl Block {
    pub fn new(device: &Device, pos: Vector3<f32>, texture: Texture) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let trans = cgmath::Matrix4::from_translation(pos);
        let block_uniform = TransformUnifrom { trans: trans.into() };
        Block { pos, texture, vertex_buffer, block_uniform }
    }
}