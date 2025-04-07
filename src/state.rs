use std::sync::Arc;
use tokio::sync::Mutex;

use winit::{event::{KeyEvent, WindowEvent}, window::Window};
use wgpu::util::DeviceExt;
use crate::{camera::{self, Camera, CameraController, Projection}, texture::{self, Texture, TextureManager}, world::{self, World}, Vertex};
use tokio::task::spawn;
use tokio::task::JoinHandle;

pub struct State {
    surface: wgpu::Surface,
    pub window: Window,
    device: Arc<wgpu::Device>,
    queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    bind_groups: [wgpu::BindGroup; 2],
    camera: Camera,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    pub camera_controller: CameraController,
    world: Arc<Mutex<World>>,
    pub time: crate::time::Time,
    projection: Projection,
    texture_manager: Arc<TextureManager>,
    buffers: Vec<(Arc<wgpu::Buffer>, usize)>,
    active_buffer: usize,
    chunk_generation_handle: Option<JoinHandle<Vec<(Arc<wgpu::Buffer>, usize)>>>,
    depth_texture: texture::Texture,
    current_base_chunk: (f32, f32)
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> State {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window).unwrap() };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();
        let device = Arc::new(device);
        let surface_caps = surface.get_capabilities(&adapter);
        
        // we assume an srgb surface
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("texture bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D3,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });    
        
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc(), ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState{
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState {
                    constant: 2, // Corresponds to bilinear filtering
                    slope_scale: 2.0,
                    clamp: 0.0,
                },
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        
        let texture_manager = TextureManager::new(&device, &queue);
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { 
            label: Some("texture bind group"),
            layout: &texture_bind_group_layout, 
            entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_manager.block_textures().view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture_manager.block_textures().sampler),
                    },
                ],
            });
            
        let camera = Camera::new((0.0, 100.0, 2000.0), cgmath::Deg(90.0), cgmath::Deg(-20.0));
        let camera_controller = camera::CameraController::new(8.0, 0.8);
        let projection = Projection::new(size.width, size.height, cgmath::Deg(40.), 0.1, 100.0);
        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);
            
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let world = Arc::new(Mutex::new(world::World::new("seed".to_string(), 20)));
        
        let texture_manager = Arc::new(texture_manager);
        let active_buffer = 0;
        let buffers = World::generate_mesh(
            texture_manager.clone(), 
            device.clone(),
            world.clone(), 
            (camera.position.x, camera.position.z)).await;

        let time = crate::time::Time::new();
        let depth_texture = Texture::create_depth_texture(&device, &config, "Depth Texture");

        Self {
            surface,
            window,
            device,
            queue,
            config,
            size,
            render_pipeline,
            bind_groups: [texture_bind_group, camera_bind_group],
            camera,
            camera_buffer,
            camera_uniform,
            camera_controller,
            world,
            time,
            projection,
            texture_manager,
            buffers,
            active_buffer,
            chunk_generation_handle: None,
            depth_texture,
            current_base_chunk: (0.0, 0.0)
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.configure();
        }
        self.projection.resize(self.config.width, self.config.height);
        self.depth_texture = Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
    }

    pub fn configure(&mut self) {
        self.surface.configure(&self.device, &self.config);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                match event {
                    KeyEvent { physical_key, state, .. } => {
                        self.camera_controller.process_keyboard(*physical_key, *state)
                    }
                }
            }
            _ => false
        }
    }

    pub async fn update(&mut self) {
        log::info!("update");
        self.time.set_update_start_time();

        self.camera_controller.update_camera(&mut self.camera, self.time.delta_time());
        self.camera_uniform.update_view_proj(&self.camera, &self.projection);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
       
        self.update_mesh().await;

        self.time.update_update_time();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.time.set_render_start_time();
        log::info!("render");
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.3,
                            g: 0.5,
                            b: 0.8,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations{
                        load: wgpu::LoadOp::Clear(100.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_groups[0], &[]);
            render_pass.set_bind_group(1, &self.bind_groups[1], &[]);
            for (buffer, num) in self.buffers.iter() {
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..*num as u32, 0..1 as u32);
            }
        }
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.time.update_render_time();
        Ok(())
    }

    async fn update_mesh(&mut self) {
        log::info!("mesh update");
        let (camera_x, camera_z) = ((self.camera.position.x / 16.0).floor(), (self.camera.position.z / 16.0).floor());

        let handle = std::mem::take(&mut self.chunk_generation_handle);
        
        match handle {
            Some(handle) => {
                if handle.is_finished() {
                    log::info!("mesh finished");
                    let buffers = handle.await;
                    match buffers {
                        Ok(buffers) => self.buffers = buffers,
                        Err(e) => {
                            log::error!("{}", e);
                        }
                    }
                    self.chunk_generation_handle = None;
                }
                else {
                    log::info!("waiting on new mesh");
                    std::mem::swap(&mut self.chunk_generation_handle, &mut Some(handle));
                }
            }
            None => {
                log::info!("test if new mesh is needed");
                if self.current_base_chunk != (camera_x, camera_z).into() {
                    self.current_base_chunk = (camera_x, camera_z).into();
                    self.chunk_generation_handle = Some(spawn(
                        World::generate_mesh(
                            self.texture_manager.clone(), 
                            self.device.clone(), 
                            self.world.clone(),
                            (self.camera.position.x, self.camera.position.z)
                    )));
                }
            }
        }
    }
}
