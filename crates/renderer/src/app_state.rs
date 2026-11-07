use std::{cell::RefCell, collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Serializer;
use slotmap::{DefaultKey, Key, KeyData, SlotMap};
use ts_rs::TS;
use vello_gpu::{
    Scene,
    color::palette::css,
    kurbo::{Affine, Circle, Point, Rect, RoundedRect, Shape, Stroke},
    peniko::Color,
};
use wasm_bindgen::{JsError, JsValue};
use web_sys::{DomRect, HtmlCanvasElement, js_sys::Function};

use crate::{
    command_queue::design_event::{DesignEvent, MouseCursors},
    design::layer::{
        layer::{DraftLayer, Layer, UpdateLayer},
        layers::Layers,
        layers_service::LayersService,
    },
    utils::wasm::create_serializer,
};

#[derive(Default, Serialize, PartialEq, Clone)]
pub enum ObjectCorner {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Default, Serialize, PartialEq, Clone)]
pub enum InteractionMode {
    #[default]
    Idle,
    Selecting,
    Dragging,
    ResizingHorizontal {
        // Right side of object? If not, left.
        right_side: bool,
    },
    ResizingVertical {
        // Bottom? If not, top.
        bottom_side: bool,
    },
    ResizingCorner {
        // Bottom right? If not, it's top left.
        corner: ObjectCorner,
    },
    Rotating {
        corner: ObjectCorner,
    },
}

#[derive(Default, Serialize, Clone)]
pub struct MouseState {
    pub clicked: bool,
    pub last_position: (f64, f64),
    // When user is interacting (aka "click"/"press"), what "mode" are we in
    pub interaction_mode: InteractionMode,
    // When user is hovering over canvas, what "mode" are we in
    // We share `InteractionMode` here because of how much overlap there is, but we don't use all of them.
    pub hover_mode: InteractionMode,
}

#[derive(Serialize, Clone)]
#[repr(usize)]
pub enum AppColors {
    Primary = 0,
    White,
    Black,
    CanvasBG,
    // Not a color. Required for array allocation in `PALETTE`.
    Count,
}

// Hardcoded color palette for Vello rendering.
// Global, zero-allocation lookup table using enum (cast to usize) as index.
static PALETTE: [Color; AppColors::Count as usize] = {
    // We have to initialize via a trick since we can't index by enum directly in const arrays easily.
    // We use a `Count` option that always sits at end of enum, this way it represents array size
    // (since enum starts at 0, adding 1 extra option makes it a true array length).
    let mut storage = [Color::TRANSPARENT; AppColors::Count as usize];
    storage[AppColors::Primary as usize] = Color::from_rgba8(12, 140, 233, 255);
    storage[AppColors::Black as usize] = Color::from_rgba8(100, 100, 100, 255);
    storage[AppColors::White as usize] = Color::from_rgba8(255, 255, 255, 255);
    storage[AppColors::CanvasBG as usize] = Color::from_rgba8(229, 229, 229, 255);
    storage
};

// Grabs a `Color` from the global palette cache based on the `AppColor` enum.
fn get_palette_color(name: AppColors) -> Color {
    // Converts enum to usize to index array
    PALETTE[name as usize]
}

const EDGE_INTERACTION_BLEED: f64 = 5.0;
const CORNER_SIZE: f64 = 10.0;

#[derive(Serialize, Clone)]
pub struct AppState {
    pub layers_service: LayersService,
    width: u16,
    height: u16,
    /// Resolution of user's device ("device pixel ratio")
    dpr: f64,
    mouse_state: MouseState,

    // Communication channel to frontend (also here to drop)
    #[serde(skip)]
    frontend_callback: Rc<RefCell<Option<Function>>>,
}

impl AppState {
    pub fn new(width: Option<u16>, height: Option<u16>) -> Self {
        let layers_service = LayersService::new();

        Self {
            layers_service,
            width: width.unwrap_or(0),
            height: height.unwrap_or(0),
            dpr: 1.0,
            mouse_state: MouseState::default(),
            frontend_callback: Rc::new(RefCell::new(None)),
        }
    }

    pub fn set_width_and_height(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn set_resolution(&mut self, resolution: f64) {
        self.dpr = resolution;
    }

    /// Build a simple scene: background, a circle, and an outlined rounded rect.
    pub fn build_scene(&self, timestamp: f64) -> Scene {
        let mut scene = Scene::new(self.width, self.height);

        // Background
        scene.set_paint(get_palette_color(AppColors::CanvasBG));
        scene.fill_rect(&Rect::new(0.0, 0.0, self.width as f64, self.height as f64));

        // Filled circle
        // let distance = 40.0;
        // let animation_offset = (timestamp / 420.0).sin() * distance;
        // scene.set_paint(css::ORANGE);
        // scene.fill_path(&Circle::new((400.0 + animation_offset, 300.0), 120.0).to_path(0.1));

        // // Stroked rounded rectangle
        // scene.set_transform(Affine::IDENTITY);
        // scene.set_stroke(Stroke::new(8.0));
        // scene.set_paint(css::WHITE);
        // scene.stroke_path(
        //     &RoundedRect::new(
        //         200.0 + animation_offset,
        //         120.0 + animation_offset,
        //         600.0 + animation_offset,
        //         480.0 + animation_offset,
        //         24.0,
        //     )
        //     .to_path(0.1),
        // );

        let layers = self.layers_service.layers.get_layers_by_parent(None);

        for (_, layer) in layers {
            scene.set_paint(Color::new(layer.color.get_components()));
            scene.fill_rect(&Rect {
                x0: layer.position.0 * self.dpr,
                y0: layer.position.1 * self.dpr,
                x1: (layer.position.0 + layer.width) * self.dpr,
                y1: (layer.position.1 + layer.height) * self.dpr,
            });
        }

        // Draw selection
        if let Some(selected_layer) = self.layers_service.layers.get_selected_layer() {
            scene.set_stroke(Stroke::new(3.0));
            scene.set_paint(get_palette_color(AppColors::Primary));

            // Stroke selected item
            scene.stroke_rect(&Rect {
                x0: selected_layer.position.0 * self.dpr,
                y0: selected_layer.position.1 * self.dpr,
                x1: (selected_layer.position.0 + selected_layer.width) * self.dpr,
                y1: (selected_layer.position.1 + selected_layer.height) * self.dpr,
            });

            // Add box corners
            let corners = create_corners_from_layer(&selected_layer, self.dpr);
            draw_corner(&mut scene, &corners.top_left);
            draw_corner(&mut scene, &corners.top_right);
            draw_corner(&mut scene, &corners.bottom_left);
            draw_corner(&mut scene, &corners.bottom_right);
        }

        scene
    }

    pub fn frame(&self, timestamp: f64) -> Scene {
        self.build_scene(timestamp)
    }

    /// Handle mouse down (aka click) on canvas.
    /// Updates local mouse position state, but also handles logic like
    /// selecting a layer or starting a drag sequence.
    /// Returns a selected layer ID if detected.
    pub fn handle_mouse_down(
        &mut self,
        canvas: &HtmlCanvasElement,
        x: f64,
        y: f64,
    ) -> Option<DefaultKey> {
        // Update mouse state
        self.mouse_state.clicked = true;

        let (x, y) = convert_mouse_to_canvas_position(&canvas, x, y);
        self.mouse_state.last_position = (x, y);

        // User clicked? Change interaction mode based on their hover mode.
        // For example, if user is hovering over resize area, we switch to that.
        match self.mouse_state.hover_mode {
            InteractionMode::Idle => {
                // Check if any layers can be selected
                let layer_id_result = self.find_layer_by_position(x, y);
                // Selected a layer? Change interaction mode + emit event to UI
                match layer_id_result {
                    Some(layer_id) => {
                        // @TODO: Check if user is clicking selected layer twice within
                        // short time to activate editing mode. Maybe use `Selecting` branch?

                        // Let frontend know layer was selected
                        let key = layer_id.data().as_ffi();
                        let event = DesignEvent::LayerSelected { layer_id: key };
                        self.emit(event);

                        // Let system know new interaction mode
                        self.mouse_state.interaction_mode = InteractionMode::Selecting;
                    }
                    None => self.mouse_state.interaction_mode = InteractionMode::Idle,
                }

                return layer_id_result;
            }
            InteractionMode::ResizingHorizontal { right_side } => {
                self.mouse_state.interaction_mode =
                    InteractionMode::ResizingHorizontal { right_side };
            }
            InteractionMode::ResizingVertical { bottom_side } => {
                self.mouse_state.interaction_mode =
                    InteractionMode::ResizingVertical { bottom_side };
            }
            InteractionMode::ResizingCorner { ref corner } => {
                self.mouse_state.interaction_mode = InteractionMode::ResizingCorner {
                    corner: corner.clone(),
                };
            }
            InteractionMode::Rotating { ref corner } => {
                self.mouse_state.interaction_mode = InteractionMode::Rotating {
                    corner: corner.clone(),
                };
            }

            // Selecting and Dragging omitted.
            _ => {}
        }

        None
    }

    pub fn handle_mouse_up(&mut self) {
        self.mouse_state.clicked = false;
        self.mouse_state.last_position = (0.0, 0.0);

        // Only reset mouse state if we were in a "action" state.
        // Idle and selecting
        if self.mouse_state.interaction_mode != InteractionMode::Idle
            && self.mouse_state.interaction_mode != InteractionMode::Selecting
        {
            self.mouse_state.interaction_mode = InteractionMode::Idle;
        }
    }

    fn handle_interaction_selecting(&mut self, x: f64, y: f64) {
        // Where are we on object (e.g. sides)? Should we change hover mode?
        if let Some(selected_layer) = self.layers_service.layers.get_selected_layer() {
            let point = Point::new(x, y);

            // We draw invisible hit boxes around selected object and check if the
            // position is within one of them.

            // Resizing - Corners
            let corners = create_corners_from_layer(&selected_layer, self.dpr);
            if corners.top_left.contains(point) {
                self.send_design_event_mouse_cursor(MouseCursors::ResizeDiagonalNwSe);

                self.mouse_state.hover_mode = InteractionMode::ResizingCorner {
                    corner: ObjectCorner::TopLeft,
                };
                return;
            }
            if corners.top_right.contains(point) {
                self.send_design_event_mouse_cursor(MouseCursors::ResizeDiagonalNeSw);

                self.mouse_state.hover_mode = InteractionMode::ResizingCorner {
                    corner: ObjectCorner::TopRight,
                };
                return;
            }
            if corners.bottom_left.contains(point) {
                self.send_design_event_mouse_cursor(MouseCursors::ResizeDiagonalNeSw);

                self.mouse_state.hover_mode = InteractionMode::ResizingCorner {
                    corner: ObjectCorner::BottomLeft,
                };
                return;
            }
            if corners.bottom_right.contains(point) {
                self.send_design_event_mouse_cursor(MouseCursors::ResizeDiagonalNwSe);

                self.mouse_state.hover_mode = InteractionMode::ResizingCorner {
                    corner: ObjectCorner::BottomRight,
                };
                return;
            }

            // Resizing - Left side
            let bleed = EDGE_INTERACTION_BLEED / 2.0;
            let edge = Rect {
                x0: (selected_layer.position.0 - bleed) * self.dpr,
                y0: selected_layer.position.1 * self.dpr,
                x1: (selected_layer.position.0 + bleed) * self.dpr,
                y1: (selected_layer.position.1 + selected_layer.height) * self.dpr,
            };
            if edge.contains(point) {
                self.send_design_event_mouse_cursor(MouseCursors::ResizeHorizontal);

                self.mouse_state.hover_mode =
                    InteractionMode::ResizingHorizontal { right_side: false };
                return;
            }

            // Resizing - Right side
            let edge = Rect {
                x0: (selected_layer.position.0 + selected_layer.width - bleed) * self.dpr,
                y0: selected_layer.position.1 * self.dpr,
                x1: (selected_layer.position.0 + selected_layer.width + bleed) * self.dpr,
                y1: (selected_layer.position.1 + selected_layer.height) * self.dpr,
            };
            if edge.contains(point) {
                self.send_design_event_mouse_cursor(MouseCursors::ResizeHorizontal);

                self.mouse_state.hover_mode =
                    InteractionMode::ResizingHorizontal { right_side: true };
                return;
            }

            // Resizing - Top side
            let edge = Rect {
                x0: (selected_layer.position.0) * self.dpr,
                y0: (selected_layer.position.1 - bleed) * self.dpr,
                x1: (selected_layer.position.0 + selected_layer.width) * self.dpr,
                y1: (selected_layer.position.1 + bleed) * self.dpr,
            };
            if edge.contains(point) {
                self.send_design_event_mouse_cursor(MouseCursors::ResizeVertical);

                self.mouse_state.hover_mode =
                    InteractionMode::ResizingVertical { bottom_side: false };

                return;
            }

            // Resizing - Bottom side
            let edge = Rect {
                x0: (selected_layer.position.0) * self.dpr,
                y0: (selected_layer.position.1 + selected_layer.height - bleed) * self.dpr,
                x1: (selected_layer.position.0 + selected_layer.width) * self.dpr,
                y1: (selected_layer.position.1 + selected_layer.height + bleed) * self.dpr,
            };
            if edge.contains(point) {
                self.send_design_event_mouse_cursor(MouseCursors::ResizeVertical);

                self.mouse_state.hover_mode =
                    InteractionMode::ResizingVertical { bottom_side: true };
                return;
            }

            // Didn't find an interaction? Revert back to selecting.
            if self.mouse_state.hover_mode != InteractionMode::Idle {
                self.mouse_state.hover_mode = InteractionMode::Idle;
                // Change icon in frontend
                let event = DesignEvent::MouseCursorChanged {
                    cursor: MouseCursors::Default,
                };
                self.emit(event);
            }
        }
    }

    fn send_design_event_layer_updated(&self, layer_id: u64, data: UpdateLayer) {
        let event = DesignEvent::LayerUpdated { layer_id, data };
        self.emit(event);
    }

    fn send_design_event_mouse_cursor(&self, cursor: MouseCursors) {
        let event = DesignEvent::MouseCursorChanged { cursor };
        self.emit(event);
    }

    fn handle_resizing_horizontal(&mut self, x: f64, right_side: bool) {
        let layer_id = self.layers_service.get_selected_layer_id().unwrap_or(0);

        if let Some(selected_layer) = self.layers_service.layers.get_mutable_selected_layer() {
            let distance_x = (x - self.mouse_state.last_position.0) / self.dpr;

            let mut event_data = UpdateLayer::default();

            // Right side? Just increase width.
            if right_side {
                selected_layer.width += distance_x;

                event_data.width = Some(selected_layer.width);
                self.send_design_event_layer_updated(layer_id, event_data);
                return;
            }

            // Otherwise it's left side, which is more complex.
            // We need to increase width while simultaneously moving object
            // to simulate it growing in place.
            // @TODO: Maybe in future we store layers like a `Rect` - and derive width when needed?
            selected_layer.position.0 += distance_x;
            // This means that dragging left on left side increase size, which means
            // distance needs to be reversed (e.g. negative distance = growing).
            selected_layer.width -= distance_x;

            event_data.width = Some(selected_layer.width);
            event_data.position_x = Some(selected_layer.position.0);
            self.send_design_event_layer_updated(layer_id, event_data);
        }
    }

    fn handle_resizing_vertical(&mut self, y: f64, bottom_side: bool) {
        let layer_id = self.layers_service.get_selected_layer_id().unwrap_or(0);

        if let Some(selected_layer) = self.layers_service.layers.get_mutable_selected_layer() {
            let distance_y = (y - self.mouse_state.last_position.1) / self.dpr;

            let mut event_data = UpdateLayer::default();

            // Bottom side? Just increase height.
            if bottom_side {
                selected_layer.height += distance_y;

                event_data.height = Some(selected_layer.height);
                self.send_design_event_layer_updated(layer_id, event_data);

                return;
            }

            // Otherwise it's left side, which is more complex.
            // We need to increase width while simultaneously moving object
            // to simulate it growing in place.
            // @TODO: Maybe in future we store layers like a `Rect` - and derive width when needed?
            selected_layer.position.1 += distance_y;
            // This means that dragging left on left side increase size, which means
            // distance needs to be reversed (e.g. negative distance = growing).
            selected_layer.height -= distance_y;

            event_data.height = Some(selected_layer.width);
            event_data.position_y = Some(selected_layer.position.1);
            self.send_design_event_layer_updated(layer_id, event_data);
        }
    }

    /// Handle mouse move on canvas.
    /// Also handles dragging of layers based on `InteractionMode`.
    pub fn handle_mouse_move(&mut self, canvas: &HtmlCanvasElement, x: f64, y: f64) {
        let (x, y) = convert_mouse_to_canvas_position(&canvas, x, y);

        // Handle "dragging" functionality
        match self.mouse_state.interaction_mode {
            // Draw selection box
            InteractionMode::Idle => self.handle_interaction_selecting(x, y),
            // Enable dragging/movement of layer
            InteractionMode::Selecting => {
                self.handle_interaction_selecting(x, y);

                // Object is selected and we're clicked? Must be dragging.
                if self.mouse_state.clicked {
                    self.mouse_state.interaction_mode = InteractionMode::Dragging;
                }
            }
            // Move layer/object
            InteractionMode::Dragging => {
                // Get distance of drag (aka "delta") and remove resolution (aka scale to CSS-space)
                let delta_x = (x - self.mouse_state.last_position.0) / self.dpr;
                let delta_y = (y - self.mouse_state.last_position.1) / self.dpr;
                let movement_result = self.layers_service.drag_selected_layer(delta_x, delta_y);

                // Update UI with new position (useful for editor sidebar)
                // @TODO: Can we throttle this? Either here or in frontend.
                if let Some(data) = movement_result {
                    let event = DesignEvent::LayerMoved {
                        layer_id: data.layer_id,
                        position: data.position,
                    };
                    self.emit(event);
                }
            }
            InteractionMode::ResizingHorizontal { right_side } => {
                // Object is selected and we're clicked? Must be dragging.
                self.handle_resizing_horizontal(x, right_side);
            }
            InteractionMode::ResizingVertical { bottom_side } => {
                self.handle_resizing_vertical(y, bottom_side);
            }
            InteractionMode::ResizingCorner { ref corner } => match corner {
                ObjectCorner::TopLeft => {
                    self.handle_resizing_horizontal(x, false);
                    self.handle_resizing_vertical(y, false);
                }
                ObjectCorner::TopRight => {
                    self.handle_resizing_horizontal(x, true);
                    self.handle_resizing_vertical(y, false);
                }
                ObjectCorner::BottomLeft => {
                    self.handle_resizing_horizontal(x, false);
                    self.handle_resizing_vertical(y, true);
                }
                ObjectCorner::BottomRight => {
                    self.handle_resizing_horizontal(x, true);
                    self.handle_resizing_vertical(y, true);
                }
            },
            InteractionMode::Rotating { ref corner } => {}
        }

        // Update last position last (in case anything needs to use it for delta math)
        self.mouse_state.last_position = (x, y);
    }

    /// Finds layer using position data. Returns layer or `None`.
    /// Assumes the position data is scaled to resolution (like mouse position).
    /// Don't use this to measure a CSS-based coordinated to find a layer
    /// (e.g. finding a layer using it's own position -- that's scaled to CSS).
    fn find_layer_by_position(&mut self, x: f64, y: f64) -> Option<DefaultKey> {
        // We get mouse coordinates scaled to resolution
        // so we convert it to "CSS" coordinates (same as design data)
        let x = x / self.dpr;
        let y = y / self.dpr;
        if let Some((key, _)) = self.layers_service.layers.find_layer_by_position(x, y) {
            self.layers_service.layers.select_layer(key);
            return Some(key);
        }
        return None;
    }

    pub fn setup_communication_callback(&self, callback: Function) -> Result<JsValue, JsError> {
        match self.frontend_callback.try_borrow_mut() {
            Ok(mut frontend_callback) => {
                *frontend_callback = Some(callback);
                Ok(JsValue::from_bool(true))
            }
            Err(_) => Err(JsError::new("Can't set callback, memory is locked")),
        }
    }

    pub fn stop_communication_callback(&self) -> Result<JsValue, JsError> {
        match self.frontend_callback.try_borrow_mut() {
            Ok(mut frontend_callback) => {
                *frontend_callback = None;
                Ok(JsValue::from_bool(true))
            }
            Err(_) => Err(JsError::new("Can't remove callback, memory is locked")),
        }
    }

    /// Emit events (`DesignEvent`) to the UI.
    /// Keeps the UI in sync with changes in backend.
    fn emit(&self, event: DesignEvent) {
        if let Ok(callback) = self.frontend_callback.try_borrow_mut().as_deref_mut() {
            if let Some(cb) = callback {
                let serializer = create_serializer();
                if let Ok(js_event) = event.serialize(&serializer) {
                    cb.call1(&JsValue::NULL, &js_event);
                }
            }
        }
    }
}

fn convert_mouse_to_canvas_position(
    canvas: &HtmlCanvasElement,
    mouse_x: f64,
    mouse_y: f64,
) -> (f64, f64) {
    let rect = canvas.get_bounding_client_rect();
    let width = rect.width().max(1.0);
    let height = rect.height().max(1.0);
    let x = ((mouse_x - rect.left()) / width).clamp(0.0, 1.0);
    let y = ((mouse_y - rect.top()) / height).clamp(0.0, 1.0);
    (x * canvas.width() as f64, y * canvas.height() as f64)
}

// Drawing functions
fn draw_corner(scene: &mut Scene, rect: &Rect) {
    scene.set_paint(get_palette_color(AppColors::White));
    scene.fill_rect(&rect);
    scene.set_paint(get_palette_color(AppColors::Primary));
    scene.stroke_rect(&rect);
}

pub struct LayerCornerRects {
    top_left: Rect,
    top_right: Rect,
    bottom_left: Rect,
    bottom_right: Rect,
}

fn create_corners_from_layer(layer: &Layer, dpr: f64) -> LayerCornerRects {
    let corner_size = CORNER_SIZE * dpr;
    let corner_padding = CORNER_SIZE / 2.0;
    let top_left_x = (layer.position.0 - corner_padding) * dpr;
    let top_left_y = (layer.position.1 - corner_padding) * dpr;
    let top_left_corner = Rect {
        x0: top_left_x,
        y0: top_left_y,
        x1: top_left_x + corner_size,
        y1: top_left_y + corner_size,
    };

    // Top Right
    let top_right_x = top_left_x + (layer.width * dpr);
    let top_right_corner = Rect {
        x0: top_right_x,
        y0: top_left_y,
        x1: top_right_x + corner_size,
        y1: top_left_y + corner_size,
    };

    // Bottom left
    let bottom_left_y = top_left_y + (layer.height * dpr);
    let bottom_left_corner = Rect {
        x0: top_left_x,
        y0: bottom_left_y,
        x1: top_left_x + corner_size,
        y1: bottom_left_y + corner_size,
    };

    // Bottom right
    let bottom_right_corner = Rect {
        x0: top_right_x,
        y0: bottom_left_y,
        x1: top_right_x + corner_size,
        y1: bottom_left_y + corner_size,
    };

    LayerCornerRects {
        top_left: top_left_corner,
        top_right: top_right_corner,
        bottom_left: bottom_left_corner,
        bottom_right: bottom_right_corner,
    }
}
