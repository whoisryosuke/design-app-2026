use vello_common::fearless_simd::Level;
use vello_gpu::{RenderSettings, RenderSize, RenderTargetConfig, Renderer, Scene};
use web_sys::HtmlCanvasElement;
use wgpu::{CurrentSurfaceTexture, TextureFormat};

/// WebGPU and Vello rendering context
pub struct RendererContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>, // Note: lifetime management in WASM can be tricky
    renderer: vello_gpu::Renderer,
    resources: vello_gpu::Resources,
    depth_texture_view: wgpu::TextureView,
    surface_format: TextureFormat,
    pub render_size: RenderSize,
}

impl RendererContext {
    /// Initializes WebGPU context, allocates buffers + memory, and finally initializes Vello renderer
    pub async fn new(canvas: HtmlCanvasElement) -> Self {
        let width = canvas.width();
        let height = canvas.height();

        // Setup wgpu backend
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            display: None,
            backends: wgpu::Backends::BROWSER_WEBGPU,
            flags: wgpu::InstanceFlags::from_build_config().with_env(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: wgpu::BackendOptions::from_env_or_default(),
        });
        // Wire up canvas to WebGPU surface
        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
            .expect("Canvas surface to be valid");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect("Adapter to be valid");

        // Configure surface
        let capabilities = surface.get_capabilities(&adapter);
        let surface_format = capabilities
            .formats
            .iter()
            .copied()
            .find(|format| !format.is_srgb())
            .unwrap_or(capabilities.formats[0]);

        // Create the device
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            })
            .await
            .expect("Device to be valid");

        surface.configure(
            &device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width,
                height,
                present_mode: wgpu::PresentMode::Fifo,
                color_space: wgpu::SurfaceColorSpace::Auto,
                alpha_mode: wgpu::CompositeAlphaMode::Opaque,
                desired_maximum_frame_latency: 2,
                view_formats: vec![],
            },
        );

        // Setup Vello GPU
        let render_size = RenderSize {
            width: width.try_into().unwrap_or(1),
            height: height.try_into().unwrap_or(1),
        };
        let mut settings = RenderSettings::default();
        settings.level = Level::try_detect().unwrap_or(Level::baseline());
        let (renderer, resources) = Renderer::new_with(
            &device,
            &RenderTargetConfig {
                format: surface_format,
                width: render_size.width,
                height: render_size.height,
            },
            settings,
        );
        let depth_texture_view = Renderer::create_depth_texture_view(&device, &render_size);

        RendererContext {
            device,
            queue,
            surface,
            renderer,
            resources,
            depth_texture_view,
            render_size,
            surface_format,
        }
    }

    pub fn render(&mut self, scene: &Scene) {
        // Render (once) using wgpu + Vello
        // Standard WebGPU command encoder flow.
        let surface_texture = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(texture) => texture,
            other => panic!("Error getting surface texture: {other:?}"),
        };
        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // @TODO: Handle renderer errors (`RenderError`)
        let _ = self.renderer.render(
            scene,
            &mut self.resources,
            &self.device,
            &self.queue,
            &mut encoder,
            &self.render_size,
            &surface_view,
            Some(&self.depth_texture_view),
            &vello_gpu::TextureBindings::new(),
            vello_gpu::TargetInit::Clear(vello_gpu::ClearSettings::default()),
        );

        self.queue.submit([encoder.finish()]);
        self.queue.present(surface_texture);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let surface_config = &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            color_space: wgpu::SurfaceColorSpace::Auto,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };
        self.surface.configure(&self.device, surface_config);

        self.render_size = RenderSize {
            width: width.try_into().unwrap_or(1),
            height: height.try_into().unwrap_or(1),
        };
        self.depth_texture_view =
            Renderer::create_depth_texture_view(&self.device, &self.render_size);
    }
}
