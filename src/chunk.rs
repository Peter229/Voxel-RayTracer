use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ],
        }
    }
}

pub struct Chunk {
    pub verts: Vec<Vertex>,
    pub buf: wgpu::Buffer,
}

impl Chunk {
    
    pub fn new(device: &wgpu::Device) -> Chunk {

        let mut verts: Vec<Vertex> = Vec::new();
        verts.push(Vertex { pos: [-1.0, -1.0, 0.0] });
        verts.push(Vertex { pos: [1.0, 1.0, 0.0] });
        verts.push(Vertex { pos: [1.0, -1.0, 0.0] });

        verts.push(Vertex { pos: [-1.0, -1.0, 0.0] });
        verts.push(Vertex { pos: [-1.0, 1.0, 0.0] });
        verts.push(Vertex { pos: [1.0, 1.0, 0.0] });

        let buf = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        Chunk { verts, buf }
    }

    pub fn render(&mut self) {
        /*render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        for texture_vertices in self.renderer.render_info.values_mut() {
            render_pass.set_vertex_buffer(0, texture_vertices.buf.slice(..));
            render_pass.set_bind_group(1, &texture_vertices.bind_group, &[]);
            render_pass.draw(0..(texture_vertices.verts.len() as u32), 0..1);
        }*/
    }
}