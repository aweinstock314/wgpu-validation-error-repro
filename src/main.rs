use std::borrow::Cow;
use wgpu_example::framework::Spawner;

pub const SPRITESHEET_RESOLUTION: u32 = 256;

pub const VERTEX_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: 0,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &[],
};

struct Example {
    quad_pipeline: wgpu::RenderPipeline,
    blit_pipeline: wgpu::RenderPipeline,
    blit_bind_group: wgpu::BindGroup,
    spritesheet_view: wgpu::TextureView,
    vertex_buffer: wgpu::Buffer,
    counter: u32,
}

impl wgpu_example::framework::Example for Example {
    fn init(
        config: &wgpu::SurfaceConfiguration,
        _: &wgpu::Adapter,
        device: &wgpu::Device,
        _: &wgpu::Queue,
    ) -> Self {
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vertex_buffer"),
            size: 0,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader_module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders.wgsl"))),
        });

        let quad_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("quad_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let quad_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("quad_pipeline"),
            layout: Some(&quad_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: &"vert_main",
                buffers: &[VERTEX_LAYOUT.clone()],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: &"frag_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });
        let blit_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("blit_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                }],
            });

        let blit_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("blit_pipeline_layout"),
            bind_group_layouts: &[&blit_bind_group_layout],
            push_constant_ranges: &[],
        });
        let blit_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("blit_pipeline"),
            layout: Some(&blit_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: &"blit_vert_main",
                buffers: &[VERTEX_LAYOUT.clone()],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: &"blit_frag_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let spritesheet = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("spritesheet"),
            size: wgpu::Extent3d {
                width: SPRITESHEET_RESOLUTION,
                height: SPRITESHEET_RESOLUTION,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let spritesheet_view = spritesheet.create_view(&wgpu::TextureViewDescriptor::default());

        let blit_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("blit_bind_group"),
            layout: &blit_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&spritesheet_view),
            }],
        });

        Example {
            quad_pipeline,
            blit_pipeline,
            blit_bind_group,
            vertex_buffer,
            spritesheet_view,
            counter: 0,
        }
    }
    fn resize(&mut self, _: &wgpu::SurfaceConfiguration, _: &wgpu::Device, _: &wgpu::Queue) {}
    fn update(&mut self, _: winit::event::WindowEvent<'_>) {}
    fn render(
        &mut self,
        surface: &wgpu::TextureView,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _: &Spawner<'_>,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("frame_encoder"),
        });
        {
            let load = if self.counter == 0 {
                wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
            } else {
                wgpu::LoadOp::Load
            };
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("quad_rpass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.spritesheet_view,
                    resolve_target: None,
                    ops: wgpu::Operations { load, store: true },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            rpass.set_pipeline(&self.quad_pipeline);
            let instance_index = self.counter % 4;
            rpass.draw(0..6, instance_index..instance_index + 1);
            self.counter = self.counter.saturating_add(1);
        }
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("blit_rpass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: surface,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rpass.set_pipeline(&self.blit_pipeline);
            rpass.set_bind_group(0, &self.blit_bind_group, &[]);
            rpass.draw(0..6, 0..1);
        }
        queue.submit(vec![encoder.finish()]);
    }
}

fn main() {
    wgpu_example::framework::run::<Example>("wgpu-validation-error-repro")
}
