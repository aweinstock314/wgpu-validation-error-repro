use std::borrow::Cow;
use wgpu_example::framework::Spawner;

pub const VERTEX_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: 0,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &[],
};

struct Example {
    shape_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
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

        let shape_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("shape_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let shape_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("shape_pipeline"),
            layout: Some(&shape_pipeline_layout),
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
                    format: config.format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Example {
            shape_pipeline,
            vertex_buffer,
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
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("frame_encoder") });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("shapes_rpass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: surface,
                    resolve_target: None, 
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::WHITE), store: true },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            rpass.set_pipeline(&self.shape_pipeline);
            rpass.draw(0..6, 0..4);
        }
        queue.submit(vec![encoder.finish()]);
    }
}

fn main() {
    wgpu_example::framework::run::<Example>("wgpu-validation-error-repro")
}
