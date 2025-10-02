use std::collections::HashSet;

use godot::{classes::{control::{LayoutPreset, MouseFilter}, Control, Engine, IControl, Input, InputEvent, InputEventMagnifyGesture, InputEventMouseButton, InputEventMouseMotion, InputEventPanGesture, Panel, ShaderMaterial}, global::{Key, MouseButton}, meta::PropertyInfo, obj::{NewAlloc, WithBaseField}, prelude::*};
use godot::global::MouseButtonMask;

use crate::utils::singleton::Singleton;

/// 2D editor viewport control for Nebula.
///
/// `Nebula2DEditor` provides a pan/zoomable canvas with optional grid overlay.
/// Supports drag-selection, touch gestures, mouse warp, bounds clamping, and shader-based grid rendering.
/// Useful for building level editors, map editors, or any 2D workspace within Godot.
#[derive(GodotClass)]
#[class(base=Control, tool)]
struct Nebula2DEditor {
    /// Current position of the viewport in world coordinates.
    #[var] viewport_position: Vector2,

    /// Enable dragging a selection rectangle with left mouse button.
    #[var] enable_drag_selection: bool,

    /// Warp the mouse when reaching viewport edges during panning.
    #[var] warp_mouse: bool,

    /// Minimum allowed zoom factor.
    #[var] zoom_minimum: f32,

    /// Maximum allowed zoom factor.
    #[var] zoom_maximum: f32,

    /// Current zoom factor.
    #[var] zoom_amount: f32,

    /// Step factor for mouse wheel zooming.
    #[var] zoom_step: f32,

    /// Grid pattern to display: none, lines, or dots.
    #[var] grid_pattern: GridPattern,

    /// Grid offset in world units.
    #[var] grid_offset: Vector2,

    /// Spacing between major lines/dots.
    #[var] grid_major_spacing: Vector2,

    /// Spacing between minor lines/dots.
    #[var] grid_minor_spacing: Vector2,

    /// Fade out major grid lines/dots when zooming out.
    #[var] grid_fade_out_major: bool,

    /// Fade out minor grid lines/dots when zooming out.
    #[var] grid_fade_out_minor: bool,

    /// Whether to draw the grid outside editor bounds.
    #[var] grid_draw_outside: bool,
    
    /// Line width for major grid lines in pixels.
    #[var] grid_major_line_width: f32,

    /// Line width for minor grid lines in pixels.
    #[var] grid_minor_line_width: f32,

    /// Radius of major grid dots in pixels.
    #[var] grid_dot_major_radius: f32,

    /// Radius of minor grid dots in pixels.
    #[var] grid_dot_minor_radius: f32,

    /// How far apart the major dots should be for every Nth dot.
    #[var] grid_dot_major_step: i32,

    /// Leftwards viewport boundary, represented as pixels.
    #[var] bound_left: i32,
    /// Rightwards viewport boundary, represented as pixels.
    #[var] bound_right: i32,    
    /// Upwards viewport boundary, represented as pixels.
    #[var] bound_top: i32,
    /// Downwards viewport boundary, represented as pixels.
    #[var] bound_bottom: i32,

    ctrl_zoom_anchor: Option<Vector2>,

    drag_start: Option<Vector2>,
    drag_current: Option<Vector2>,

    viewport_control: Gd<Control>,
    viewport_panel: Gd<Panel>,
    selection_panel: Gd<Panel>,
    viewport_shader_material: Gd<ShaderMaterial>,

    selected_objects: HashSet<Gd<Control>>,
    selected_objects_panel: Gd<Panel>,
    
    // Individual highlight panels during drag
    highlight_panels: Vec<Gd<Panel>>,

    base: Base<Control>,
}


#[derive(GodotConvert, Var, Export, Clone, Copy, PartialEq)]
#[godot(via = i32)]
enum GridPattern {
    None,
    Lines,
    Dots
}

#[godot_api]
impl IControl for Nebula2DEditor {
    fn init(base: Base<Control>) -> Self {
        Self {
            viewport_position: Vector2::ZERO,
            enable_drag_selection: true,
            warp_mouse: true,
            
            zoom_minimum: 0.2,
            zoom_maximum: 4.0,
            zoom_amount: 1.0,
            zoom_step: 0.15,
            
            grid_pattern: GridPattern::Lines,
            grid_offset: Vector2::ZERO,
            grid_major_spacing: Vector2 { x: 256.0, y: 256.0 },
            grid_minor_spacing: Vector2 { x: 32.0, y: 32.0 },

            grid_fade_out_major: false,
            grid_fade_out_minor: true,
            grid_draw_outside: false,

            grid_major_line_width: 3.0,
            grid_minor_line_width: 2.0,

            grid_dot_major_radius: 4.0,
            grid_dot_minor_radius: 3.0,
            grid_dot_major_step: 4,

            bound_left: -999999,
            bound_right: 999999,
            bound_top: 999999,
            bound_bottom: -999999,

            ctrl_zoom_anchor: None,

            drag_start: None,
            drag_current: None,

            viewport_control: Control::new_alloc(),
            viewport_panel: Panel::new_alloc(),
            selection_panel: Panel::new_alloc(),
            viewport_shader_material: ShaderMaterial::new_gd(),

            selected_objects: HashSet::new(),
            selected_objects_panel: Panel::new_alloc(),
            highlight_panels: Vec::new(),

            base
        }
    }

    fn ready(&mut self) {
        let mut base_control = self.base_mut().to_godot();
        
        let mut pn: Gd<Panel> = Panel::new_alloc();
        self.viewport_panel = pn.to_godot();
        pn.set_anchors_preset(LayoutPreset::FULL_RECT);
        pn.set_mouse_filter(MouseFilter::IGNORE);
        
        let vc: Gd<Control> = Control::new_alloc();
        self.viewport_control = vc.to_godot();
        pn.add_child(&vc);
        
        
        for mut n in base_control.get_children().iter_shared() {
            if let Ok(mut c) = n.to_godot().try_cast::<Control>() {
                c.set_mouse_filter(MouseFilter::IGNORE);
            }
            if !Engine::singleton().is_editor_hint() {
                n.reparent(&vc);
            }
        };
        
        base_control.add_child(&pn);
        base_control.move_child(&pn, 0);
        
        let shader = Singleton::singleton().bind_mut().get_shader(Singleton::SHADER_EDITOR_2D_GRID);
        let mut material = ShaderMaterial::new_gd();
        self.viewport_shader_material = material.to_godot();
        material.set_shader(&shader);
        pn.set_material(&material);
        
        self.selection_panel.set_mouse_filter(MouseFilter::IGNORE);
        let mut selection_panel = self.selection_panel.to_godot();
        self.viewport_control.add_child(&selection_panel);
        selection_panel.set_meta("ignore_select", &true.to_variant());
        selection_panel.hide();
        
        self.selected_objects_panel.set_mouse_filter(MouseFilter::IGNORE);
        let mut selected_objects_panel = self.selected_objects_panel.to_godot();
        self.viewport_control.add_child(&selected_objects_panel);
        selected_objects_panel.set_meta("ignore_select", &true.to_variant());
        selected_objects_panel.hide();
       
        self.signals().selection_finished().connect_self(Nebula2DEditor::on_drag_end);

        self.reload_theme();
    }

	fn gui_input(&mut self, event: Gd<InputEvent>) {
		let mut vref = self.viewport_control.to_godot();

		// Mouse motion
		if let Ok(e) = event.to_godot().try_cast::<InputEventMouseMotion>() {
			let ctrl = Input::singleton().is_key_pressed(Key::CTRL);

			if e.get_button_mask() == MouseButtonMask::MIDDLE {
				if ctrl {
					// Ctrl + Middle drag → zoom at anchor
					if let Some(anchor) = self.ctrl_zoom_anchor {
						let dy = -e.get_relative().y;
						if dy.abs() > 0.0 {
							let zoom_factor = 1.0 + dy * 0.01;

							let old_zoom = self.zoom_amount;
							let new_zoom = (old_zoom * zoom_factor)
								.clamp(self.zoom_minimum, self.zoom_maximum);

							if (new_zoom - old_zoom).abs() > 0.001 {
								let world_point = (anchor + self.viewport_position) / old_zoom;

								self.zoom_amount = new_zoom;
								vref.set_scale(Vector2::splat(new_zoom));

								self.viewport_position = world_point * new_zoom - anchor;
							}
						}
					}
				} else {
					// Normal panning
					self.viewport_position -= e.get_relative();

					// Warp mouse if enabled
					if self.warp_mouse {
						let viewport_rect = vref.get_viewport_rect();
						let mut mouse_pos = e.get_position();
						let mut warped = false;

						if mouse_pos.x <= viewport_rect.position.x {
							mouse_pos.x = viewport_rect.position.x + viewport_rect.size.x - 2.0;
							warped = true;
						} else if mouse_pos.x >= viewport_rect.position.x + viewport_rect.size.x - 1.0 {
							mouse_pos.x = viewport_rect.position.x + 1.0;
							warped = true;
						}

						if mouse_pos.y <= viewport_rect.position.y {
							mouse_pos.y = viewport_rect.position.y + viewport_rect.size.y - 2.0;
							warped = true;
						} else if mouse_pos.y >= viewport_rect.position.y + viewport_rect.size.y - 1.0 {
							mouse_pos.y = viewport_rect.position.y + 1.0;
							warped = true;
						}

						if warped {
							Input::singleton().warp_mouse(mouse_pos);
						}
					}
				}
			}

			// Dragging update
			if e.get_button_mask() == MouseButtonMask::LEFT
				&& self.enable_drag_selection
				&& let Some(start) = self.drag_start
			{
				self.drag_current = Some(e.get_global_position());
				let current = self.drag_current.unwrap();
				let rect = Rect2::new(start, current - start).abs();
				
				// Convert to local/viewport coordinates
				let local_rect = self.rect_to_local(rect);

				// Show drag rectangle
				self.selection_panel.show();
				self.selection_panel.set_position(local_rect.position);
				self.selection_panel.set_size(local_rect.size);

				// Update individual highlight panels
				self.update_highlight_panels(local_rect);

				self.signals().selection_dragged().emit(local_rect);
			}
		}

		// Mouse button press/release
		else if let Ok(e) = event.to_godot().try_cast::<InputEventMouseButton>() {
			if e.is_pressed() {
				if e.get_button_index() == MouseButton::LEFT {
					// Begin drag
					let pos = e.get_global_position();
					self.drag_start = Some(pos);
					self.drag_current = Some(pos);
				} else if e.get_button_index() == MouseButton::MIDDLE {
					// If ctrl held, capture zoom anchor
					if Input::singleton().is_key_pressed(Key::CTRL) {
						self.ctrl_zoom_anchor = Some(e.get_position());
					}
				}
			} else if e.get_button_index() == MouseButton::LEFT {
				// End drag
				if let (Some(start), Some(end)) = (self.drag_start, self.drag_current) {
					let rect = Rect2::new(start, end - start).abs();
					let local_rect = self.rect_to_local(rect);

					self.signals().selection_finished().emit(local_rect);

					// Hide drag rectangle
					self.selection_panel.hide();
					self.selection_panel.set_size(Vector2::ZERO);
				}

				self.drag_start = None;
				self.drag_current = None;
			} else if e.get_button_index() == MouseButton::MIDDLE {
				self.ctrl_zoom_anchor = None;
			}

			if e.is_pressed() {
				if e.get_button_index() != MouseButton::WHEEL_UP
					&& e.get_button_index() != MouseButton::WHEEL_DOWN
				{
					return;
				}

				let zoom_factor = match e.get_button_index() {
					MouseButton::WHEEL_UP => 1.0 + self.zoom_step,
					MouseButton::WHEEL_DOWN => 1.0 / (1.0 + self.zoom_step),
					_ => return,
				};

				let mouse_pos = e.get_position();

				let world_point = (mouse_pos + self.viewport_position) / self.zoom_amount;

				let old_zoom = self.zoom_amount;
				let new_zoom = (old_zoom * zoom_factor).clamp(self.zoom_minimum, self.zoom_maximum);

				if (new_zoom - old_zoom).abs() > 0.001 {
					self.zoom_amount = new_zoom;
					vref.set_scale(Vector2::splat(new_zoom));

					self.viewport_position = world_point * new_zoom - mouse_pos;
				}
			}
		}

		// Touch Panning
		else if let Ok(e) = event.to_godot().try_cast::<InputEventPanGesture>() {
			self.viewport_position += e.get_delta() * 35.0;
		}

		// Touch/trackpad magnify gesture zooming
		else if let Ok(e) = event.to_godot().try_cast::<InputEventMagnifyGesture>() {
			let mouse_pos = e.get_position();
			let zoom_factor = e.get_factor();

			let old_zoom = self.zoom_amount;
			let new_zoom = (old_zoom * zoom_factor).clamp(self.zoom_minimum, self.zoom_maximum);

			if (new_zoom - old_zoom).abs() > 0.001 {
				let world_point = (mouse_pos + self.viewport_position) / old_zoom;

				self.zoom_amount = new_zoom;
				vref.set_scale(Vector2::splat(new_zoom));

				self.viewport_position = world_point * new_zoom - mouse_pos;
			}
		}

		self.clamp_viewport_position();
		self.viewport_shader_material
			.set_shader_parameter("position", &self.viewport_position.to_variant());
		self.viewport_shader_material
			.set_shader_parameter("zoom", &self.zoom_amount.to_variant());
		vref.set_position(-self.viewport_position);
	}

    fn get_property_list(&mut self) -> Vec<PropertyInfo> {
        let mut props = Vec::new();

        
        props.append(&mut vec![
            PropertyInfo::new_group("Editor", ""),
            PropertyInfo::new_export::<Vector2>("viewport_position"),
            PropertyInfo::new_export::<bool>("enable_drag_selection"),
            PropertyInfo::new_export::<bool>("warp_mouse"),
            
            PropertyInfo::new_group("Zoom", "zoom_"),
            PropertyInfo::new_export::<f32>("zoom_minimum"),
            PropertyInfo::new_export::<f32>("zoom_maximum"),
            PropertyInfo::new_export::<f32>("zoom_amount"),
            PropertyInfo::new_export::<f32>("zoom_step"),

            PropertyInfo::new_group("Grid", "grid_"),
            PropertyInfo::new_export::<GridPattern>("grid_pattern"),
            ]);
            
            if self.grid_pattern != GridPattern::None {
                props.append(&mut vec![
                    PropertyInfo::new_export::<Vector2>("grid_offset"),
                ]);
                if self.grid_pattern == GridPattern::Lines {
                    props.append(&mut vec![
                        PropertyInfo::new_export::<Vector2>("grid_major_spacing"),
                    ]);
                }
                props.append(&mut vec![
                    PropertyInfo::new_export::<Vector2>("grid_minor_spacing"),
                    PropertyInfo::new_export::<bool>("grid_fade_out_major"),
                    PropertyInfo::new_export::<bool>("grid_fade_out_minor"),
                    PropertyInfo::new_export::<bool>("grid_draw_outside"),
                ]);

            match self.grid_pattern {
              GridPattern::Lines => {
                props.append(&mut vec![
                    PropertyInfo::new_export::<f32>("grid_major_line_width"),
                    PropertyInfo::new_export::<f32>("grid_minor_line_width"),
                ])},
              GridPattern::Dots => {
                props.append(&mut vec![
                    PropertyInfo::new_export::<f32>("grid_dot_major_radius"),
                    PropertyInfo::new_export::<f32>("grid_dot_minor_radius"),
                    PropertyInfo::new_export::<f32>("grid_dot_major_step"),
                ])}
              _ => {}
            }  
        }

        props.append(&mut vec![
            PropertyInfo::new_group("Bounds", "bound_"),
            PropertyInfo::new_export::<i32>("bound_left"),
            PropertyInfo::new_export::<i32>("bound_right"),
            PropertyInfo::new_export::<i32>("bound_top"),
            PropertyInfo::new_export::<i32>("bound_bottom"),
        ]);

        props
    }

    fn set_property(&mut self, name: StringName, _value: Variant) -> bool {
        if name.eq(&StringName::from("grid_pattern")) {
            self.base_mut().notify_property_list_changed();
        }

        self.base_mut().call_deferred("_update_shader", &[]);

        false
    }
}

#[godot_api]
impl Nebula2DEditor {
    /// Emitted when a drag-selection rectangle is updated.
    #[signal] fn selection_dragged(rect: Rect2);

    /// Emitted when a drag-selection rectangle is finished.
    #[signal] fn selection_finished(rect: Rect2);

    /// Adds a Control node to the editor's viewport.
    #[func] pub fn add_control(&mut self, mut control: Gd<Control>) {
        control.set_mouse_filter(MouseFilter::IGNORE);
        
        if !Engine::singleton().is_editor_hint() {
            if let Some(_p) = control.get_parent() {
                control.reparent(&self.viewport_control);
            } else {
                self.viewport_control.add_child(&control);
            }
        }
    }

    fn update_highlight_panels(&mut self, rect: Rect2) {
        // Clear old highlight panels
        self.clear_highlight_panels();

        let children = self.viewport_control.get_children();
        let base = self.base().to_godot();
        let highlight_style = base.get_theme_stylebox_ex("object_preselect_box")
            .theme_type("Nebula2DEditor")
            .done();

        for child in children.iter_shared() {
            if child.has_meta("ignore_select") {
                continue;
            }

            if let Ok(c) = child.try_cast::<Control>() {
                let child_pos = c.get_position();
                let child_rect = Rect2::new(child_pos, c.get_size());

                if rect.intersects(child_rect) {
                    // Create highlight panel for this object
                    let mut panel = Panel::new_alloc();
                    panel.set_mouse_filter(MouseFilter::IGNORE);
                    panel.set_position(child_pos);
                    panel.set_size(c.get_size());
                    panel.add_theme_stylebox_override("panel", highlight_style.as_ref());
                    panel.set_meta("ignore_select", &true.to_variant());
                    
                    self.viewport_control.add_child(&panel);
                    self.highlight_panels.push(panel);
                }
            }
        }
    }

    fn clear_highlight_panels(&mut self) {
        for mut panel in self.highlight_panels.drain(..) {
            panel.queue_free();
        }
    }

    pub fn on_drag_end(&mut self, rect: Rect2) {
        // Clear individual highlight panels
        self.clear_highlight_panels();

        let children = self.viewport_control.get_children();

        for child in children.iter_shared() {
            if child.has_meta("ignore_select") {
                continue;
            }

            if let Ok(c) = child.try_cast::<Control>() {
                let child_pos = c.get_position();
                let child_rect = Rect2::new(child_pos, c.get_size());

                if rect.intersects(child_rect) {
                    self.add_to_selection(c);
                } else {
                    self.remove_from_selection(c);
                }
            }
        }
        
        self.update_selection_panel_bounds();
    }


    /// Adds a control to the current selection.
    #[func] pub fn add_to_selection(&mut self, control: Gd<Control>) {
        self.selected_objects.insert(control);
        self.update_selection_panel_bounds();
    }

    /// Removes a control from the selection.
    #[func] pub fn remove_from_selection(&mut self, control: Gd<Control>) {
        self.selected_objects.retain(|obj| !obj.eq(&control));
        self.update_selection_panel_bounds();
    }


    fn update_selection_panel_bounds(&mut self) {
        if self.selected_objects.is_empty() {
            self.selected_objects_panel.hide();
            self.selected_objects_panel.set_size(Vector2::ZERO);
            return;
        }

        let mut iter = self.selected_objects.iter();
        let first = iter.next().unwrap();
        let first_pos = first.get_position();
        let first_size = first.get_size();
        let mut min_x = first_pos.x;
        let mut min_y = first_pos.y;
        let mut max_x = first_pos.x + first_size.x;
        let mut max_y = first_pos.y + first_size.y;

        for obj in iter {
            let p = obj.get_position();
            let s = obj.get_size();
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x + s.x);
            max_y = max_y.max(p.y + s.y);
        }

        let pos = Vector2::new(min_x, min_y);
        let size = Vector2::new(max_x - min_x, max_y - min_y);

        let base = self.base().to_godot();
        let selected_style = base.get_theme_stylebox_ex("selected_objects_box")
            .theme_type("Nebula2DEditor")
            .done();

        self.selected_objects_panel.set_position(pos);
        self.selected_objects_panel.set_size(size);
        self.selected_objects_panel.add_theme_stylebox_override("panel", selected_style.as_ref());
        self.selected_objects_panel.show();
    }

    #[func] pub fn clear_selection(&mut self) {
        self.selected_objects.clear();
        self.update_selection_panel_bounds();
    }

    pub fn rect_to_local(&self, rect: Rect2) -> Rect2 {
        let local_pos = (rect.position + self.viewport_position) / self.zoom_amount;
        let local_br = (rect.position + rect.size + self.viewport_position) / self.zoom_amount;

        Rect2::new(local_pos, local_br - local_pos).abs()
    }

    #[func]
    pub fn _on_theme_changed(&mut s_ref: Gd<Nebula2DEditor>) {
        s_ref.bind_mut().reload_theme();
    }

    pub fn reload_theme(&mut self) {
        let mut shader_material = self.viewport_shader_material.to_godot();
        let base = self.base().to_godot();
        let mut selection_panel = self.selection_panel.to_godot();

        let background_color = base.get_theme_color_ex("background_color").theme_type("Nebula2DEditor").done();
        let major_color = base.get_theme_color_ex("major_color").theme_type("Nebula2DEditor").done();
        let minor_color = base.get_theme_color_ex("minor_color").theme_type("Nebula2DEditor").done();
        let outside_region_color = base.get_theme_color_ex("outside_region_color").theme_type("Nebula2DEditor").done();

        selection_panel.add_theme_stylebox_override("panel", base.get_theme_stylebox_ex("selection_box").theme_type("Nebula2DEditor").done().as_ref());

        shader_material.set_shader_parameter("background_color", &Variant::from(background_color));
        shader_material.set_shader_parameter("grid_major", &Variant::from(major_color));
        shader_material.set_shader_parameter("grid_minor", &Variant::from(minor_color));
        shader_material.set_shader_parameter("bound_outside_color", &Variant::from(outside_region_color));
    }

    #[func]
    pub fn _update_shader(&mut self) {
        let mat = &mut self.viewport_shader_material;
        mat.set_shader_parameter("position", &Variant::from(self.viewport_position));
        mat.set_shader_parameter("grid_offset", &Variant::from(self.grid_offset));
        mat.set_shader_parameter("zoom", &Variant::from(self.zoom_amount));
        mat.set_shader_parameter("grid_pattern", &Variant::from(self.grid_pattern));
        
        mat.set_shader_parameter("major_spacing", &Variant::from(self.grid_major_spacing));
        mat.set_shader_parameter("minor_spacing", &Variant::from(self.grid_minor_spacing));
        
        mat.set_shader_parameter("fade_major", &Variant::from(self.grid_fade_out_major));
        mat.set_shader_parameter("fade_minor", &Variant::from(self.grid_fade_out_minor));
        mat.set_shader_parameter("draw_grid_outside_bounds", &Variant::from(self.grid_draw_outside));
        
        mat.set_shader_parameter("bound_top", &Variant::from(self.bound_top));
        mat.set_shader_parameter("bound_bottom", &Variant::from(self.bound_bottom));
        mat.set_shader_parameter("bound_left", &Variant::from(self.bound_left));
        mat.set_shader_parameter("bound_right", &Variant::from(self.bound_right));
        
        mat.set_shader_parameter("major_line_width", &Variant::from(self.grid_major_line_width));
        mat.set_shader_parameter("minor_line_width", &Variant::from(self.grid_minor_line_width));

        mat.set_shader_parameter("dot_major_radius_px", &Variant::from(self.grid_dot_major_radius));
        mat.set_shader_parameter("dot_minor_radius_px", &Variant::from(self.grid_dot_minor_radius));
        mat.set_shader_parameter("dot_major_step", &Variant::from(self.grid_dot_major_step));
    }

    pub fn clamp_viewport_position(&mut self) {
        let zoom = self.zoom_amount;
        let mut pos = (self.viewport_position + (self.base().get_size() / 2.0)) / zoom;
        
        let bounds = Rect2 {
            position: Vector2 {
                x: self.bound_left as f32,
                y: -self.bound_top as f32,
            },
            size: Vector2 {
                x: (i32::abs(self.bound_right - self.bound_left)) as f32,
                y: (i32::abs(self.bound_top - self.bound_bottom)) as f32,
            },
        };

        pos.x = pos.x.clamp(bounds.position.x, bounds.position.x + bounds.size.x);
        pos.y = pos.y.clamp(bounds.position.y, bounds.position.y + bounds.size.y);

        self.viewport_position = pos * zoom - (self.base().get_size() / 2.0);
    }
}