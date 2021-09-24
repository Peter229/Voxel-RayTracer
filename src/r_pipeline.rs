use crate::texture;

pub fn pipeline(device: &wgpu::Device, format: wgpu::TextureFormat, render_pipeline_layout: &wgpu::PipelineLayout, vs_module_s: wgpu::ShaderModuleDescriptor, fs_module_s: wgpu::ShaderModuleDescriptor, vertex_format: wgpu::VertexBufferLayout, primitive: wgpu::PrimitiveTopology) -> wgpu::RenderPipeline {
    
    let vs_module = device.create_shader_module(&vs_module_s);
    let fs_module = device.create_shader_module(&fs_module_s);

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vs_module,
            entry_point: "main",
            buffers: &[vertex_format],
        },
        fragment: Some(wgpu::FragmentState {
            module: &fs_module,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add
                    },
                    alpha: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add
                    },
                }),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Front),
            clamp_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
    });
    render_pipeline
}

/*depth
Some(wgpu::DepthStencilState {
    format: texture::Texture::DEPTH_FORMAT,
    depth_write_enabled: true,
    depth_compare: wgpu::CompareFunction::Less,
    stencil: wgpu::StencilState::default(),
    bias: wgpu::DepthBiasState {
        constant: 0,
        slope_scale: 0.0,
        clamp: 0.0,
    },
})*/