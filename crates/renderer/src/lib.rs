//! "Hello world" for Vello GPU through wgpu's WebGPU backend in the browser via WASM (phew!)
//!
//! Creates a canvas, builds a simple scene, and renders it once.

#![cfg(target_arch = "wasm32")]

use std::collections::BTreeMap;

use fractional_index::FractionalIndex;
use serde::Serialize;
use slotmap::{DefaultKey, SlotMap};
use vello_common::{
    color::palette::css,
    fearless_simd::Level,
    kurbo::{Affine, Circle, Rect, RoundedRect, Shape, Stroke},
};
use vello_gpu::{RenderSettings, RenderSize, RenderTargetConfig, Renderer, Scene};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wgpu::CurrentSurfaceTexture;

#[derive(Serialize)]
pub enum FrameLayout {
    Flex,
    Grid,
}

#[derive(Serialize)]
pub enum LayerKind {
    Group,
    Frame { layout: FrameLayout },
    Rect,
    Text(String),
}

#[derive(Serialize)]
pub struct Layer {
    pub name: String,
    pub kind: LayerKind,
    pub position: (f64, f64),
    pub width: f64,
    pub height: f64,
    pub visible: bool,
    pub opacity: f32,
    pub parent: Option<DefaultKey>,
    pub items: BTreeMap<FractionalIndex, DefaultKey>,
}

pub struct AppState {
    layers: SlotMap<DefaultKey, Layer>,
    width: u16,
    height: u16,
}

impl AppState {
    pub fn new(width: u16, height: u16) -> Self {
        let layers = SlotMap::new();
        Self {
            layers,
            width,
            height,
        }
    }

    /// Build a simple scene: background, a circle, and an outlined rounded rect.
    pub fn build_scene(&self) -> Scene {
        let mut scene = Scene::new(self.width, self.height);

        // Background
        scene.set_paint(css::BLUE);
        scene.fill_rect(&Rect::new(0.0, 0.0, self.width as f64, self.height as f64));

        // Filled circle
        scene.set_paint(css::ORANGE);
        scene.fill_path(&Circle::new((400.0, 300.0), 120.0).to_path(0.1));

        // Stroked rounded rectangle
        scene.set_transform(Affine::IDENTITY);
        scene.set_stroke(Stroke::new(8.0));
        scene.set_paint(css::WHITE);
        scene.stroke_path(&RoundedRect::new(200.0, 120.0, 600.0, 480.0, 24.0).to_path(0.1));

        scene
    }
}

/// Debug entry point.
/// Finds canvas by ID and renders scene inside it.
#[wasm_bindgen]
pub async fn run(canvas_id: &str) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let canvas = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(canvas_id))
        .ok_or_else(|| JsValue::from_str(&format!("No element with id `{canvas_id}`")))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("Element is not a <canvas>"))?;

    let width = canvas.width() as u16;
    let height = canvas.height() as u16;
    let app_state = AppState::new(width, height);
    let scene = app_state.build_scene();
    render_once(canvas, &scene).await;

    Ok(())
}

/// Set up wgpu (WebGPU backend) for the canvas and render the scene a single time.
async fn render_once(canvas: HtmlCanvasElement, scene: &Scene) {
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
        width: width.try_into().unwrap(),
        height: height.try_into().unwrap(),
    };
    let mut settings = RenderSettings::default();
    settings.level = Level::try_detect().unwrap_or(Level::baseline());
    let (mut renderer, mut resources) = Renderer::new_with(
        &device,
        &RenderTargetConfig {
            format: surface_format,
            width: render_size.width,
            height: render_size.height,
        },
        settings,
    );
    let depth_texture_view = Renderer::create_depth_texture_view(&device, &render_size);

    // Render (once) using wgpu + Vello
    // Standard WebGPU command encoder flow.
    let surface_texture = match surface.get_current_texture() {
        CurrentSurfaceTexture::Success(texture) => texture,
        other => panic!("Error getting surface texture: {other:?}"),
    };
    let surface_view = surface_texture
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    renderer
        .render(
            scene,
            &mut resources,
            &device,
            &queue,
            &mut encoder,
            &render_size,
            &surface_view,
            Some(&depth_texture_view),
            &vello_gpu::TextureBindings::new(),
            vello_gpu::TargetInit::Clear(vello_gpu::ClearSettings::default()),
        )
        .unwrap();

    queue.submit([encoder.finish()]);
    queue.present(surface_texture);
}
