#![allow(unused_imports)]

use crate::camera;

use cgmath::SquareMatrix;
use cgmath::InnerSpace;
use cgmath::Rotation3;
use cgmath::Zero;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    screen_dimensions: [i32; 2],
    junk: [i32; 2],
    view_pos: [f32; 3],
    junk2: i32,
    view_dir: [f32; 3],
    junk3: i32,
    right: [f32; 3],
    junk4: i32,
    up: [f32; 3],
}

impl Uniforms {
    pub fn new(screen_dimensions: [i32; 2]) -> Self {
        Self {
            screen_dimensions,
            junk: [0, 0],
            view_pos: [0.0, 0.0, 0.0],
            junk2: 0,
            view_dir: [0.0, 0.0, -1.0],
            junk3: 0,
            right: [1.0, 0.0, 0.0],
            junk4: 0,
            up: [0.0, 1.0, 0.0],
        }
    }

    pub fn update_cam(&mut self, camera_pos: cgmath::Point3<f32>, cam_dir: cgmath::Vector3<f32>) {

        self.view_pos = camera_pos.into();
        self.view_dir = cam_dir.into();
        let right = cgmath::Vector3::new(cam_dir.z, 0.0, -cam_dir.x).normalize();
        self.right = right.into();
        self.up = right.cross(cam_dir).into();
    }

    pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("uniform_bind_group_layout"),
        });

        uniform_bind_group_layout
    }

    pub fn get_buffers(&mut self, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::BindGroup) {

        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[*self]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let uniform_bind_group_layout = Uniforms::get_bind_group_layout(device);

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),
        });

        (uniform_buffer, uniform_bind_group)
    }
}
