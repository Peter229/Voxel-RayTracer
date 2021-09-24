#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::texture;
use crate::r_pipeline;
use crate::camera;
use crate::uniform;
use crate::chunk;
use crate::game;
use crate::svo;
use crate::level;

use wgpu::util::DeviceExt;
use noise::{NoiseFn, Perlin, Seedable};
use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};

pub struct Material {
    tex: texture::Texture,
    bind_group: wgpu::BindGroup,
}

pub struct State {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    pub camera: camera::Camera,
    pub camera_controller: camera::CameraController,
    uniforms: uniform::Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    game: game::Game,
    world: Material,
    sun_pos: cgmath::Vector3<f32>,
    sun_col: cgmath::Vector3<f32>,
    svo: svo::Svo,
    svo_buffer: wgpu::Buffer,
    svo_bind_group: wgpu::BindGroup,
}

impl State {

    pub async fn new(window: &Window) -> Self {

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();

        //Fifo or Immediate (vsync on and off)
        let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

        let mut config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        surface.configure(&device, &config);

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D3,
                        sample_type: wgpu::TextureSampleType::Float {
                            filterable: false,
                        },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false, filtering: false },
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let svo_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage {
                            read_only: false,
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform::Uniforms::get_bind_group_layout(&device), &texture_bind_group_layout, &svo_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = r_pipeline::pipeline(&device, swapchain_format, &layout, wgpu::include_spirv!("shader.vert.spv"), wgpu::include_spirv!("shader.frag.spv"), chunk::Vertex::desc(), wgpu::PrimitiveTopology::TriangleList);

        let mut camera = camera::Camera::new();
        let camera_controller = camera::CameraController::new(20.0, 0.002);

        let mut uniforms = uniform::Uniforms::new([config.width as i32, config.height as i32]);

        let (uniform_buffer, uniform_bind_group) = uniforms.get_buffers(&device);

        let mut game = game::Game::new(&device);

        const width: u32 = 256;
        const height: u32 = 256;
        const depth: u32 = 256;
        const radius: f64 = 50.0;

        const freq: f64 = 60.0;
        const ampl: f64 = 240.0;

        let mut world_data: Vec<u8> = Vec::new();
        let mut perlin = noise::Perlin::new();
        let start = std::time::SystemTime::now();
        let since_the_epoch = start
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
        //println!("{}", since_the_epoch.as_millis() as u32);
        perlin = perlin.set_seed(since_the_epoch.as_millis() as u32);
        for y in 0..height {
            for z in 0..width {
                for x in 0..width {
                    let he: f64 = (y as f64) - (perlin.get([x as f64 / freq, y as f64 / freq, z as f64 / freq]) + 1.0 / 2.0) * ampl;
                    if (he < 0.0) {
                        world_data.push(230 - (3 * y as u8));
                        world_data.push(244);
                        world_data.push(10);
                        world_data.push(255);
                    }
                    else {
                        world_data.push(0);
                        world_data.push(0);
                        world_data.push(0);
                        world_data.push(0);
                    }
                }
            }
        }

        let world_tex = texture::Texture::world_array(&device, &queue, &world_data, width, height, "world").unwrap();

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&world_tex.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&world_tex.sampler),
                },
            ],
            label: None,
        });

        let world = Material { tex: world_tex, bind_group };

        let sun_pos = cgmath::Vector3::new(0.0, 100.0, 0.0);
        let sun_col = cgmath::Vector3::new(1.0, 1.0, 1.0);

        let svo = svo::Svo::new(&world_data, width as i32, height as i32);

        //let bytes = [0i32, 1, 2, 3, 255, 1, 2, 3, 255, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];

        let mut svo_data = Vec::new();
        svo.compile_svo_data(&mut svo_data);

        let svo_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("svo"),
            size: svo_data.len() as u64 * 4,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(&svo_buffer, 0, bytemuck::cast_slice(&svo_data));

        let svo_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &svo_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(svo_buffer.as_entire_buffer_binding()),
                },
            ],
            label: None,
        });

        let mut count = 0i32;
        svo.num_nodes(&mut count);

        println!("{}", count);

        //level::Level::new("LostValleyCTF");

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            camera,
            camera_controller,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            game,
            world,
            sun_pos,
            sun_col,
            svo,
            svo_buffer,
            svo_bind_group,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {

        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {

        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                //self.camera.process_keyboard(*keycode, *state);
                self.camera_controller.process_keyboard(*keycode, *state);
                if *state == ElementState::Pressed && *keycode == VirtualKeyCode::Up {
                    self.sun_pos.x += 10.0;
                }
                if *state == ElementState::Pressed && *keycode == VirtualKeyCode::Down {
                    self.sun_pos.x -= 10.0;
                }
                if *state == ElementState::Pressed && *keycode == VirtualKeyCode::Right {
                    self.sun_pos.z += 10.0;
                }
                if *state == ElementState::Pressed && *keycode == VirtualKeyCode::Left {
                    self.sun_pos.z -= 10.0;
                }
                if *state == ElementState::Pressed && *keycode == VirtualKeyCode::N {
                    self.sun_pos.y += 10.0;
                }
                if *state == ElementState::Pressed && *keycode == VirtualKeyCode::M {
                    self.sun_pos.y -= 10.0;
                }

                if *state == ElementState::Pressed && *keycode == VirtualKeyCode::R {
                    self.sun_col.x += 0.05;
                }
                if *state == ElementState::Pressed && *keycode == VirtualKeyCode::E {
                    self.sun_col.x -= 0.05;
                }
                if *state == ElementState::Pressed && *keycode == VirtualKeyCode::G {
                    self.sun_col.y += 0.05;
                }
                if *state == ElementState::Pressed && *keycode == VirtualKeyCode::F {
                    self.sun_col.y -= 0.05;
                }
                if *state == ElementState::Pressed && *keycode == VirtualKeyCode::B {
                    self.sun_col.z += 0.05;
                }
                if *state == ElementState::Pressed && *keycode == VirtualKeyCode::V {
                    self.sun_col.z -= 0.05;
                }
                if self.sun_col.x < 0.0 {
                    self.sun_col.x = 0.0;
                }
                if self.sun_col.y < 0.0 {
                    self.sun_col.y = 0.0;
                }
                if self.sun_col.z < 0.0 {
                    self.sun_col.z = 0.0;
                }
                if self.sun_col.x > 1.0 {
                    self.sun_col.x = 0.0;
                }
                if self.sun_col.y > 1.0 {
                    self.sun_col.y = 0.0;
                }
                if self.sun_col.z > 1.0 {
                    self.sun_col.z = 0.0;
                }
                true
            }
            WindowEvent::CursorMoved  { position, .. } => {
                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                true
            }
            _ => false,
        }
    }

    pub fn raw_mouse(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion {
                delta
            } => {
                self.camera_controller.process_mouse(delta.0 as f32, -delta.1 as f32, &mut self.camera);
            }
            _ => (),
        }
    }

    pub fn update(&mut self, delta_time: f32) {

        self.camera_controller.update_camera(&mut self.camera, delta_time);
        let mut yo = 0;
        let old_x = self.camera.position.x;
        let old_z = self.camera.position.z;
        //self.svo.check_solid(&cgmath::Vector3::new(self.camera.position.x, self.camera.position.y, self.camera.position.z), 0.0, 0.0, 0.0, &mut yo);
        if yo == 1 {
            //println!("Collide");
            //println!("pos {} {} {}", self.camera.position.x as i32, self.camera.position.y as i32, self.camera.position.z as i32);
            self.camera.position.y += 0.5;
        }
        else {
            //println!("Empty");
        }
        //println!("{:?}", self.camera.position);
        self.uniforms.update_cam(self.camera.position, self.camera.forward);
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));
    }

    pub fn render(&mut self) {
        //println!("{:?}", self.camera.forward);
        let frame = match self.surface.get_current_frame() {
            Ok(frame) => frame,
            Err(_) => {
                self.surface.configure(&self.device, &self.config);
                self.surface
                    .get_current_frame()
                    .expect("Failed to acquire next surface texture!")
            }
        };

        let view = frame
            .output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_bind_group(1, &self.world.bind_group, &[]);
            render_pass.set_bind_group(2, &self.svo_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.game.chunk.buf.slice(..));
            render_pass.draw(0..(self.game.chunk.verts.len() as u32), 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}