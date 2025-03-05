use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use anyhow::*;
use image::GenericImageView;

pub struct Texture {
    #[allow(unused)]
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }

    pub fn from_3d_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: Vec<&[u8]>,
        label: Option<&str>,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: 16,
            height: 16,
            depth_or_array_layers: bytes.len() as u32,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        for (i, bytes) in bytes.iter().enumerate() {
            let img = image::load_from_memory(bytes).unwrap();
            let rgba = img.to_rgba8();
            let dimensions = img.dimensions();
            let size = wgpu::Extent3d {
                width: 16,
                height: 16,
                depth_or_array_layers: 1,
            };
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    aspect: wgpu::TextureAspect::All,
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: i as u32,
                    },
                },
                &rgba,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * dimensions.0),
                    rows_per_image: Some(dimensions.1),
                },
                size,
            );
        }

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }

    pub fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        label: &str,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: config.width.max(1),
            height: config.height.max(1),
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }
}


pub struct TextureManager {
    depth_texture: Texture,
    block_textures: Texture,
    name_to_id: HashMap<String, f32>,
}

impl TextureManager {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, queue: &wgpu::Queue) -> Self {
        let (block_textures, name_to_id) = Self::load_textures("./assets/textures", device, queue, Some("block_textures"));
        Self { 
            depth_texture: Texture::create_depth_texture(device, config, "Depth Texture"), 
            block_textures,
            name_to_id 
        }
    }
    pub fn depth_texture(&self) -> &Texture {
        &self.depth_texture
    }
    pub fn depth_texture_mut(&mut self) -> &mut Texture {
        &mut self.depth_texture
    }

    pub fn block_textures(&self) -> &Texture {
        &self.block_textures
    }

    pub fn get_id(&self, name: String) -> f32 {
        *self.name_to_id.get(&name).unwrap()
    }

    fn load_textures(path: &str, device: &wgpu::Device, queue: &wgpu::Queue, label: Option<&str>) -> (Texture, HashMap<String, f32>) {
        let mut file_paths_names: Vec<(PathBuf, String)> = vec![];
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            
            if let std::result::Result::Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    let file_name = entry.file_name();
                    let as_string = file_name.into_string().unwrap();
                    let (name, extension) = as_string.split_once('.').unwrap();
                    
                    if extension == "png" {
                        file_paths_names.push((entry.path(), name.to_string()));
                    }
                }
            }
            else {
                log::error!("Could not find type for {:?}", entry.path())
            }
        }
        let mut bytes:Vec<Vec<u8>> = vec![];
        let mut name_to_id = HashMap::new();

        for (i, (path, name)) in file_paths_names.iter().enumerate() {
            let mut handle = fs::File::open(path).unwrap();
            bytes.push(vec![]);
            let _ = handle.read_to_end(bytes.last_mut().unwrap());
            name_to_id.insert(name.clone(), i as f32 / file_paths_names.len() as f32 + 0.0001);
        }

        let bytes = bytes.iter().map(|x| x.as_slice()).collect();
        (Texture::from_3d_bytes(device, queue, bytes, label), name_to_id)
    }
}