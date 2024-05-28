use std::sync::Arc;

use wgpu::{ShaderModule, VertexBufferLayout};

pub struct State {
    pub window: Arc<winit::window::Window>,
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'static>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub render_pipeline: wgpu::RenderPipeline,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,

    pub instances: Vec<crate::instances::Instance>,
}

impl State {
    pub fn new(window: winit::window::Window) -> Self {
        let window = Arc::new(window);
        let window_size = window.inner_size();
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = create_adapter(&instance, &surface);
        let surface_config = surface
            .get_default_config(&adapter, window_size.width, window_size.height)
            .unwrap();

        let (device, queue) = create_device(&adapter);

        surface.configure(&device, &surface_config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/shader.wgsl"));

        let render_pipeline = create_render_pipeline_with_fragment(
            &shader,
            "vs",
            "fs",
            &surface_config,
            &device,
            Some("Test triangle pipeline"),
            &[crate::models::Vertex::desc()],
        );

        let vertex_buffer = create_buffer(
            &device,
            Some("Vertex Buffer"),
            1_000_000,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        );

        let index_buffer = create_buffer(
            &device,
            Some("Index Buffer"),
            10_000,
            wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        );

        State {
            window,
            instance,
            surface,
            adapter,
            device,
            queue,
            render_pipeline,
            surface_config,
            vertex_buffer,
            index_buffer,

            instances: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        if self.instances.is_empty() {
            self.add_instance(Some("./models/cube.obj"));
        }
    }

    pub fn add_instance(&mut self, mesh_path: Option<&str>) {
        let instance = crate::instances::Instance::new(mesh_path);
        self.instances.push(instance);
    }
}

fn create_render_pipeline_with_fragment(
    shader: &ShaderModule,
    vertex_entry_name: &str,
    frag_entry_name: &str,
    surface_config: &wgpu::SurfaceConfiguration,
    device: &wgpu::Device,
    label: Option<&str>,
    vertex_buffers: &[VertexBufferLayout],
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: None,
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: vertex_entry_name,
            compilation_options: Default::default(),
            buffers: vertex_buffers,
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: frag_entry_name,
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                blend: Some(wgpu::BlendState::REPLACE),
                format: surface_config.format,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multiview: None,
        multisample: wgpu::MultisampleState::default(),
    })
}

fn create_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
    pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(surface),
    }))
    .unwrap()
}

fn create_buffer(
    device: &wgpu::Device,
    label: Option<&str>,
    size: u64,
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label,
        mapped_at_creation: false,
        size,
        usage,
    })
}

fn create_device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("Device"),
            required_features: wgpu::Features::empty(),
            required_limits:
                wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
        },
        None,
    ))
    .unwrap()
}
