//! "Hello world" for Vello GPU through wgpu's WebGPU backend in the browser via WASM (phew!)
//!
//! Creates a canvas, builds a simple scene, and renders it once.

#![cfg(target_arch = "wasm32")]

use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use fractional_index::FractionalIndex;
use serde::{Deserialize, Serialize};
use slotmap::{DefaultKey, Key, KeyData, SlotMap};
use vello_common::{
    color::palette::css,
    fearless_simd::Level,
    kurbo::{Affine, Circle, Rect, RoundedRect, Shape, Stroke},
};
use vello_gpu::{RenderSettings, RenderSize, RenderTargetConfig, Renderer, Scene};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wgpu::CurrentSurfaceTexture;

#[derive(Serialize, Clone)]
pub enum FrameLayout {
    Flex,
    Grid,
}

#[derive(Serialize, Clone)]
pub enum LayerKind {
    Group,
    Frame { layout: FrameLayout },
    Rect,
    Text(String),
}

#[derive(Serialize, Clone)]
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

#[derive(Serialize)]
pub struct LayerListResponse {
    pub id: u64,
    pub name: String,
    pub visible: bool,
}

#[derive(Serialize, Clone)]
pub struct AppState {
    layers: SlotMap<DefaultKey, Layer>,
    selected_layer: DefaultKey,
    width: u16,
    height: u16,
}

impl AppState {
    pub fn new(width: u16, height: u16) -> Self {
        let layers = SlotMap::new();
        Self {
            layers,
            selected_layer: Default::default(),
            width,
            height,
        }
    }

    pub fn set_width_and_height(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.insert(layer);
    }

    pub fn get_layers(&self) -> Option<JsValue> {
        let list: Vec<LayerListResponse> = self
            .layers
            .iter()
            .map(|(k, v)| LayerListResponse {
                id: k.data().as_ffi(),
                name: v.name.clone(),
                visible: v.visible,
            })
            .collect();
        serde_wasm_bindgen::to_value(&list).ok()
    }

    pub fn select_layer(&mut self, layer_key: u64) {
        let key_data = KeyData::from_ffi(layer_key);
        let slot_map_key: DefaultKey = key_data.into();

        self.selected_layer = slot_map_key;
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

    pub fn frame(&self, timestamp: f64) -> Scene {
        self.build_scene()
    }
}

#[derive(Serialize, Deserialize)]
pub enum AppCommand {
    StartRendering,
}

/// WebGPU and Vello rendering context
struct RendererContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>, // Note: lifetime management in WASM can be tricky
    renderer: vello_gpu::Renderer,
    resources: vello_gpu::Resources,
    depth_texture_view: wgpu::TextureView,
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
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
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

        self.renderer
            .render(
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
            )
            .unwrap();

        self.queue.submit([encoder.finish()]);
        self.queue.present(surface_texture);
    }
}

#[wasm_bindgen]
pub struct DesignRenderer {
    state: Rc<RefCell<AppState>>,
    canvas: Option<HtmlCanvasElement>,
    renderer_context: Rc<RefCell<RendererContext>>,
}

#[wasm_bindgen]
impl DesignRenderer {
    #[wasm_bindgen]
    pub async fn create(canvas_id: &str) -> Result<Self, JsValue> {
        let canvas = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(&canvas_id))
            .ok_or_else(|| JsValue::from_str(&format!("No element with id `{canvas_id}`")))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| JsValue::from_str("Element is not a <canvas>"))?;

        let state = AppState::new(canvas.width() as u16, canvas.height() as u16);
        let renderer_context = RendererContext::new(canvas.clone()).await;

        Ok(DesignRenderer {
            state: Rc::new(RefCell::new(state)),
            canvas: Some(canvas),
            renderer_context: Rc::new(RefCell::new(renderer_context)),
        })
    }

    #[wasm_bindgen]
    pub fn dispatch(&mut self, cmd_js: JsValue) -> Result<(), JsValue> {
        // Convert JS object directly to Rust Enum
        let cmd = serde_wasm_bindgen::from_value::<AppCommand>(cmd_js)
            .map_err(|_| JsValue::from_str("Element is not a <canvas>"))?;
        match cmd {
            AppCommand::StartRendering => {
                self.render();
                Ok(())
            }
        }
    }

    async fn render(&mut self) -> Result<(), JsValue> {
        let scene = self.state.borrow().build_scene();
        self.renderer_context.borrow_mut().render(&scene);

        Ok(())
    }

    #[wasm_bindgen]
    pub async fn run(&self) {
        {
            // Create a placeholder for the RAF closure to loop again
            let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut(f64)>>));
            // Closure for the initial/first RAF closure
            let g = f.clone();

            // Clone pointers to state (via `Rc`) to move into closure
            let app_state = self.state.clone();
            let renderer = self.renderer_context.clone();

            // Define our animation function / closure
            *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
                // Update state with latest time and get latest scene data
                let scene = app_state.borrow_mut().frame(timestamp);
                // Render scene data
                renderer.borrow_mut().render(&scene);

                // Keep looping
                request_animation_frame(f.borrow().as_ref().unwrap());
            }) as Box<dyn FnMut(f64)>));

            // Start animation using wasm_bindgen/browser's RAF
            request_animation_frame(g.borrow().as_ref().unwrap());
        }
    }
}

/// This runs automatically when WASM is initialized.
/// Currently used for debugging.
#[wasm_bindgen(start)]
pub fn main_js() {
    // This runs automatically on WASM load
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = requestAnimationFrame)]
    fn request_animation_frame(f: &Closure<dyn FnMut(f64)>);
}
