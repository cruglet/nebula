use std::collections::{HashMap, HashSet};

use godot::{classes::{control::{LayoutPreset, MouseFilter}, Control, Engine, IControl, Input, InputEvent, InputEventMagnifyGesture, InputEventMouseButton, InputEventMouseMotion, InputEventPanGesture, Node, Panel, ShaderMaterial}, global::{Key, MouseButton}, meta::PropertyInfo, obj::{NewAlloc, WithBaseField}, prelude::*};
use godot::global::MouseButtonMask;

use crate::runtime::utils::singleton::Singleton;

/// 2D editor editor control for Nebula.
///
/// `Nebula2DEditor` provides a pan/zoomable canvas with optional grid overlay.
/// Supports drag-selection, touch gestures, mouse warp, bounds clamping, and shader-based grid rendering.
/// Useful for building level editors, map editors, or any general 2D workspace.
#[derive(GodotClass)]
#[class(base=Control, tool)]
struct Nebula2DEditor {
    /// Current position of the editor's viewport in world coordinates. Does not account for zoom.  
    #[var] editor_global_position: Vector2,
    
    /// Derived variable from editor_global_position, which takes zoom into account.
    /// Setting this variable will not do anything to the viewport. To do so, use 
    /// [method set_editor_local_pos] or assign `editor_global_position`.
    #[var] editor_position: Vector2,

    /// Enable dragging a selection rectangle with left mouse button.
    #[var] enable_drag_selection: bool,

    /// Warp the mouse when reaching editor viewport edges during panning.
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

    /// Leftwards editor boundary, represented as pixels.
    #[var] bound_left: i32,
    /// Rightwards editor boundary, represented as pixels.
    #[var] bound_right: i32,    
    /// Upwards editor boundary, represented as pixels.
    #[var] bound_top: i32,
    /// Downwards editor boundary, represented as pixels.
    #[var] bound_bottom: i32,

    ctrl_zoom_anchor: Option<Vector2>,

    drag_start: Option<Vector2>,
    drag_current: Option<Vector2>,

    editor_control: Gd<Control>,
    editor_panel: Gd<Panel>,
    editor_bound_objects: HashMap<Gd<Control>, Vector2>,
    selection_panel: Gd<Panel>,
    editor_highlighted_objects_container: Gd<Control>,
    editor_shader_material: Gd<ShaderMaterial>,

    selected_objects: HashSet<Gd<Control>>,
    selected_objects_panel: Gd<Panel>,
    
    highlight_panels: Vec<Gd<Panel>>,
    active_highlight_count: usize,

    layers: HashMap<GString, Gd<Control>>,
    layer_order: Vec<GString>,
    
    layer_children_cache: Vec<Gd<Control>>,
    layer_children_dirty: bool,

    last_highlight_update: std::time::Instant,
    highlight_update_interval: std::time::Duration,

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
            editor_global_position: Vector2::ZERO,
            editor_position: Vector2::ZERO,
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

            editor_control: Control::new_alloc(),
            editor_panel: Panel::new_alloc(),
            selection_panel: Panel::new_alloc(),
            editor_shader_material: ShaderMaterial::new_gd(),
            editor_highlighted_objects_container: Control::new_alloc(),
            editor_bound_objects: HashMap::new(),

            selected_objects: HashSet::new(),
            selected_objects_panel: Panel::new_alloc(),
            highlight_panels: vec![],
            active_highlight_count: 0,

            layers: HashMap::new(),
            layer_order: Vec::new(),
            
            layer_children_cache: Vec::new(),
            layer_children_dirty: true,
            
            last_highlight_update: std::time::Instant::now(),
            highlight_update_interval: std::time::Duration::from_millis(16),

            base
        }
    }

    fn ready(&mut self) {
        let mut base_control = self.base_mut().to_godot_owned();
        
        let mut pn: Gd<Panel> = Panel::new_alloc();
        self.editor_panel = pn.to_godot_owned();
        pn.set_anchors_preset(LayoutPreset::FULL_RECT);
        pn.set_mouse_filter(MouseFilter::IGNORE);
        pn.set_clip_contents(true);
        
        let vc: Gd<Control> = Control::new_alloc();
        self.editor_control = vc.to_godot_owned();
        pn.add_child(&vc);
        
        for mut n in base_control.get_children().iter_shared() {
            if n.has_meta("ui_layer") {
                continue;
            }
            if let Ok(mut c) = n.to_godot_owned().try_cast::<Control>() {
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
        self.editor_shader_material = material.to_godot_owned();
        material.set_shader(&shader);
        pn.set_material(&material);

        let highlighted_obj_c = self.editor_highlighted_objects_container.to_godot_owned();
        self.base_mut().add_child(&highlighted_obj_c);
        
        self.selection_panel.set_mouse_filter(MouseFilter::IGNORE);
        let mut selection_panel = self.selection_panel.to_godot_owned();
        pn.add_child(&selection_panel);
        selection_panel.set_meta("ignore_select", &true.to_variant());
        selection_panel.hide();
        
        self.selected_objects_panel.set_mouse_filter(MouseFilter::IGNORE);
        let mut selected_objects_panel = self.selected_objects_panel.to_godot_owned();
        pn.add_child(&selected_objects_panel);
        selected_objects_panel.set_meta("ignore_select", &true.to_variant());
        selected_objects_panel.hide();
       
        self.signals().selection_drag_finished().connect_self(Nebula2DEditor::on_drag_end);
        self.signals().mouse_entered().connect_self(move |a| {
            if selection_panel.is_visible() {
                selection_panel.hide();
                a.hide_all_highlights();
            }
        });

        self.signals().resized().connect_self(Nebula2DEditor::_update_shader);

        self.reload_theme();
    }

    fn gui_input(&mut self, event: Gd<InputEvent>) {
        let mut vref = self.editor_control.to_godot_owned();
        let scale_factor = Singleton::get_scale_factor();

        if let Ok(e) = event.to_godot_owned().try_cast::<InputEventMouseMotion>() {
            let ctrl = Input::singleton().is_key_pressed(Key::CTRL);

            if e.get_button_mask() == MouseButtonMask::MIDDLE {
                if ctrl {
                    if let Some(anchor) = self.ctrl_zoom_anchor {
                        let dy = -e.get_relative().y;
                        if dy.abs() > 0.0 {
                            let zoom_factor = 1.0 + dy * 0.01;

                            let old_zoom = self.zoom_amount;
                            let new_zoom = (old_zoom * zoom_factor)
                                .clamp(self.zoom_minimum, self.zoom_maximum);

                            if (new_zoom - old_zoom).abs() > 0.001 {
                                let world_point = (anchor + self.editor_global_position) / old_zoom;

                                self.xset_zoom_amount(new_zoom);
                                vref.set_scale(Vector2::splat(new_zoom));

                                self.editor_global_position = world_point * new_zoom - anchor;
                            }
                        }
                    }
                } else {
                    self.editor_global_position -= e.get_relative();
                    
                    if self.warp_mouse {
                        let viewport = vref.get_viewport().unwrap();
                        let viewport_rect = viewport.get_visible_rect();

                        let mut mouse_pos = vref.get_global_mouse_position();
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

            if e.get_button_mask() == MouseButtonMask::LEFT
                && self.enable_drag_selection
            {
                if let Some(start) = self.drag_start {
                    self.drag_current = Some(e.get_position());
                    let current = self.drag_current.unwrap();
                    let rect = Rect2::new(start, current - start).abs();
                    
                    let local_rect = self.rect_to_local(rect);

                    self.selection_panel.show();
                    self.selection_panel.set_position(rect.position);
                    self.selection_panel.set_size(rect.size);

                    let now = std::time::Instant::now();
                    if now.duration_since(self.last_highlight_update) >= self.highlight_update_interval {
                        self.update_highlight_panels(Some(local_rect));
                        self.last_highlight_update = now;
                    }

                    self.signals().selection_dragged().emit(local_rect);
                }
            }
        }

        // Mouse button press/release
        else if let Ok(e) = event.to_godot_owned().try_cast::<InputEventMouseButton>() {
            if e.is_pressed() {
                if e.get_button_index() == MouseButton::LEFT {
                    let pos = e.get_position();
                    self.drag_start = Some(pos);
                    self.drag_current = Some(pos);
                } else if e.get_button_index() == MouseButton::MIDDLE {
                    if Input::singleton().is_key_pressed(Key::CTRL) {
                        self.ctrl_zoom_anchor = Some(e.get_position());
                    }
                }
            } else if e.get_button_index() == MouseButton::LEFT {
                if let (Some(start), Some(end)) = (self.drag_start, self.drag_current) {
                    let rect = Rect2::new(start, end - start).abs();
                    let local_rect = self.rect_to_local(rect);

                    self.signals().selection_drag_finished().emit(local_rect);
                    let selected_objects = Array::from_iter(self.selected_objects.iter().cloned());
                    self.signals().selection_updated().emit(&selected_objects);

                    self.selection_panel.hide();
                    self.hide_all_highlights();
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

                let world_point = (mouse_pos + self.editor_global_position) / self.zoom_amount;

                let old_zoom = self.zoom_amount;
                let new_zoom = (old_zoom * zoom_factor).clamp(self.zoom_minimum, self.zoom_maximum);

                if (new_zoom - old_zoom).abs() > 0.001 {
                    self.xset_zoom_amount(new_zoom);
                    vref.set_scale(Vector2::splat(new_zoom));

                    self.editor_global_position = world_point * new_zoom - mouse_pos;
                }
            }
        }

        // Touch Panning
        else if let Ok(e) = event.to_godot_owned().try_cast::<InputEventPanGesture>() {
            self.editor_global_position += e.get_delta() * 35.0;
        }

        // Touch/trackpad magnify gesture zooming
        else if let Ok(e) = event.to_godot_owned().try_cast::<InputEventMagnifyGesture>() {
            let mouse_pos = e.get_position();
            let zoom_factor = e.get_factor();

            let old_zoom = self.zoom_amount;
            let new_zoom = (old_zoom * zoom_factor).clamp(self.zoom_minimum, self.zoom_maximum);

            if (new_zoom - old_zoom).abs() > 0.001 {
                let world_point = (mouse_pos + self.editor_global_position) / old_zoom;

                self.xset_zoom_amount(new_zoom);
                vref.set_scale(Vector2::splat(new_zoom));

                self.editor_global_position = world_point * new_zoom - mouse_pos;
            }
        }

        let global_pos = self.base().get_global_position();
        self.clamp_editor_global_position();
        
        self.editor_shader_material
            .set_shader_parameter("position", &(self.editor_global_position - global_pos).to_variant());
        self.editor_shader_material
            .set_shader_parameter("zoom", &self.zoom_amount.to_variant());
        self.editor_shader_material
            .set_shader_parameter("scale_factor", &scale_factor.to_variant());
        
        vref.set_position(-self.editor_global_position);
    }



    fn get_property_list(&mut self) -> Vec<PropertyInfo> {
        let mut props = Vec::new();

        
        props.append(&mut vec![
            PropertyInfo::new_group("Editor", ""),
            PropertyInfo::new_export::<Vector2>("editor_global_position"),
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
    #[signal] fn selection_drag_finished(rect: Rect2);

    /// Similar to [signal selection_drag_finished], but returns the list of objects selected.
    #[signal] fn selection_updated(objects: Array<Gd<Control>>);

    /// Fires whenever the editor is moved, and provides the current [param position].
    #[signal] fn editor_panned(position: Vector2);

    /// Fires whenever the editor is zoomed, and provides the current [param zoom].
    #[signal] fn editor_zoomed(zoom: f32);

    /// Emitted when a layer is created.
    #[signal] fn layer_created(layer_path: GString);

    /// Emitted when a layer is renamed.
    #[signal] fn layer_renamed(old_path: GString, new_path: GString);

    /// Emitted when a layer is moved.
    #[signal] fn layer_moved(layer_path: GString, new_index: i32);

    /// Creates a new layer at the specified path. Parent layers are created automatically if they don't exist.
    /// Returns true if the layer was created, false if it already exists.
    #[func] pub fn create_layer(&mut self, layer_path: GString) -> bool {
        if self.layers.contains_key(&layer_path) {
            return false;
        }

        let path_str = layer_path.to_string();
        let parts: Vec<&str> = path_str.split('/').filter(|s| !s.is_empty()).collect();
        
        if parts.is_empty() {
            godot_warn!("Layer path cannot be empty");
            return false;
        }

        let mut current_path = String::new();
        let mut parent_node = self.editor_control.to_godot_owned();

        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                current_path.push('/');
            }
            current_path.push_str(part);
            let current_gstring = GString::from(&current_path);

            if !self.layers.contains_key(&current_gstring) {
                let mut layer_node = Control::new_alloc();
                layer_node.set_name(*part);
                layer_node.set_mouse_filter(MouseFilter::IGNORE);
                layer_node.set_meta("layer_node", &true.to_variant());
                layer_node.set_meta("ignore_select", &true.to_variant());
                
                parent_node.add_child(&layer_node);
                
                self.layers.insert(current_gstring.clone(), layer_node.clone());
                self.layer_order.push(current_gstring.clone());
                
                self.signals().layer_created().emit(&current_gstring);
                self.layer_children_dirty = true;
            }

            parent_node = self.layers.get(&current_gstring).unwrap().clone();
        }

        true
    }

    /// Renames a layer from old_path to new_path. The layer maintains its position in the hierarchy.
    #[func] pub fn rename_layer(&mut self, old_path: GString, new_path: GString) -> bool {
        if !self.layers.contains_key(&old_path) {
            godot_warn!("Layer '{}' does not exist", old_path);
            return false;
        }

        if self.layers.contains_key(&new_path) {
            godot_warn!("Layer '{}' already exists", new_path);
            return false;
        }

        if let Some(node) = self.layers.remove(&old_path) {
            let name = new_path.to_string();
            let new_name = name.split('/').filter(|s| !s.is_empty()).last().unwrap_or("");
            node.clone().set_name(new_name);
            
            self.layers.insert(new_path.clone(), node);
            
            if let Some(pos) = self.layer_order.iter().position(|x| x == &old_path) {
                self.layer_order[pos] = new_path.clone();
            }

            self.signals().layer_renamed().emit(&old_path, &new_path);
            self.layer_children_dirty = true;
            return true;
        }

        false
    }

    /// Moves a layer to a new index in the scene tree ordering.
    #[func] pub fn move_layer(&mut self, layer_path: GString, new_index: i32) -> bool {
        if !self.layers.contains_key(&layer_path) {
            godot_warn!("Layer '{}' does not exist", layer_path);
            return false;
        }

        if let Some(node) = self.layers.get(&layer_path) {
            let node_clone = node.clone();
            
            if let Some(mut parent) = node_clone.get_parent() {
                parent.move_child(&node_clone, new_index);
                self.signals().layer_moved().emit(&layer_path, new_index);
                self.layer_children_dirty = true;
                return true;
            }
        }

        false
    }

    /// Removes a layer and all its children from the editor.
    #[func] pub fn remove_layer(&mut self, layer_path: GString) -> bool {
        if !self.layers.contains_key(&layer_path) {
            godot_warn!("Layer '{}' does not exist", layer_path);
            return false;
        }

        let mut layers_to_remove = Vec::new();
        for key in self.layers.keys() {
            if key.to_string().starts_with(&format!("{}/", layer_path)) || key == &layer_path {
                layers_to_remove.push(key.clone());
            }
        }

        for key in layers_to_remove.iter() {
            if let Some(mut node) = self.layers.remove(key) {
                node.queue_free();
            }
            self.layer_order.retain(|x| x != key);
        }

        self.layer_children_dirty = true;
        true
    }

    /// Checks if a layer exists at the given path.
    #[func] pub fn has_layer(&self, layer_path: GString) -> bool {
        self.layers.contains_key(&layer_path)
    }

    /// Returns an array of all layer paths in creation order.
    #[func] pub fn get_layers(&self) -> Array<GString> {
        Array::from_iter(self.layer_order.iter().cloned())
    }

    /// Adds a Control node to the specified layer. Creates the layer hierarchy if it doesn't exist.
    #[func] pub fn add_object(&mut self, layer_path: GString, mut node: Gd<Control>) {
        if !self.layers.contains_key(&layer_path) {
            self.create_layer(layer_path.clone());
        }

        if let Some(layer_node) = self.layers.get(&layer_path) {
            let mut layer_node_clone = layer_node.clone();
            
            node.set_mouse_filter(MouseFilter::IGNORE);
            
            if !Engine::singleton().is_editor_hint() {
                if node.get_parent().is_some() {
                    node.reparent(&layer_node_clone);
                } else {
                    layer_node_clone.add_child(&node);
                }
            }
            
            self.layer_children_dirty = true;
        }
    }

    #[func]
    /// Returns the Control node for the given layer path, or null if it doesn't exist.
    pub fn get_layer(&self, layer_path: GString) -> Option<Gd<Control>> {
        self.layers.get(&layer_path).cloned()
    }


    /// Adds a [member Control] to the current selection.
    #[func] pub fn add_to_selection(&mut self, control: Gd<Control>) {
        self.selected_objects.insert(control);
    }

    /// Removes a given [member Control] from the current selection.
    #[func] pub fn remove_from_selection(&mut self, control: Gd<Control>) {
        self.selected_objects.retain(|obj| !obj.eq(&control));
    }

    /// Clears all nodes from the current selection.
    #[func] pub fn clear_selection(&mut self) {
        self.selected_objects.clear();
        self.update_selection_panel_bounds();
    }
    
    /// Attaches a node to the movement of the editor
    #[func] pub fn bind_to_editor(&mut self, control: Gd<Control>) {
        if !self.editor_bound_objects.contains_key(&control) {
            let local_pos = (control.get_position() + self.editor_global_position) / self.zoom_amount;
            self.editor_bound_objects.insert(control, local_pos);
        }
    }

    /// Detaches a node from the movement of the editor
    #[func] pub fn unbind_from_editor(&mut self, control: Gd<Control>) {
        if self.editor_bound_objects.contains_key(&control) {
            self.editor_bound_objects.retain(|c, _| {!c.eq(&control)});
        }
    }

    /// Primarily internal, handles updaing the editor when the theme is changed.
    #[func] pub fn _on_theme_changed(&mut s_ref: Gd<Nebula2DEditor>) {
        s_ref.bind_mut().reload_theme();
    }    

    /// Primarily internal, handles updaing the backend shader for the editor when it is updated.
    #[func] pub fn _update_shader(&mut self) {
        let scale_factor = Singleton::get_scale_factor();
        let global_pos: Vector2 = self.base().get_global_position();
        let mat = &mut self.editor_shader_material;
        
        mat.set_shader_parameter("position", &Variant::from(self.editor_global_position - global_pos));
        mat.set_shader_parameter("grid_offset", &Variant::from(self.grid_offset));
        mat.set_shader_parameter("zoom", &Variant::from(self.zoom_amount));
        mat.set_shader_parameter("scale_factor", &Variant::from(scale_factor));
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


    fn refresh_layer_children_cache(&mut self) {
        if !self.layer_children_dirty {
            return;
        }

        self.layer_children_cache.clear();
        
        self.layer_children_cache.reserve(256);
        
        let layer_order_snapshot = self.layer_order.clone();
        
        for layer_path in layer_order_snapshot.iter() {
            if let Some(layer_node) = self.layers.get(layer_path) {
                Self::collect_children_recursive_to_vec(layer_node.clone().upcast(), &mut self.layer_children_cache);
            }
        }
        
        self.layer_children_dirty = false;
    }

    fn collect_children_recursive_to_vec(node: Gd<Node>, result: &mut Vec<Gd<Control>>) {
        let children = node.get_children();
        
        let child_count = children.len();
        if result.capacity() - result.len() < child_count {
            result.reserve(child_count);
        }
        
        for child in children.iter_shared() {
            if child.has_meta("layer_node") {
                Self::collect_children_recursive_to_vec(child, result);
            } else if !child.has_meta("ignore_select") {
                if let Ok(control) = child.try_cast::<Control>() {
                    result.push(control);
                }
            }
        }
    }

    fn is_node_visible_in_tree(&self, node: &Gd<Control>) -> bool {
        if !node.is_visible_in_tree() {
            return false;
        }
        true
    }

    fn update_highlight_panels(&mut self, rect_op: Option<Rect2>) {
        if rect_op.is_none() {
            self.hide_all_highlights();
            return;
        }

        let rect = rect_op.unwrap();
        
        self.refresh_layer_children_cache();

        let base = self.base().to_godot_owned();
        let highlight_style = base.get_theme_stylebox_ex("object_preselect_box")
            .theme_type("Nebula2DEditor")
            .done();

        let mut highlight_idx = 0;

        let rect_min_x = rect.position.x;
        let rect_min_y = rect.position.y;
        let rect_max_x = rect.position.x + rect.size.x;
        let rect_max_y = rect.position.y + rect.size.y;

        for child in self.layer_children_cache.iter() {
            if !self.is_node_visible_in_tree(child) {
                continue;
            }

            let child_pos = child.get_position();
            let child_size = child.get_size();
            
            let child_max_x = child_pos.x + child_size.x;
            let child_max_y = child_pos.y + child_size.y;
            
            if child_pos.x > rect_max_x || child_max_x < rect_min_x ||
            child_pos.y > rect_max_y || child_max_y < rect_min_y {
                continue;
            }

            let local_rect = Rect2 { position: child_pos, size: child_size };
            let global_rect = self.rect_to_global(local_rect);

            if highlight_idx >= self.highlight_panels.len() {
                let mut panel = Panel::new_alloc();
                panel.set_mouse_filter(MouseFilter::IGNORE);
                panel.set_meta("ignore_select", &true.to_variant());
                self.editor_highlighted_objects_container.add_child(&panel);
                self.highlight_panels.push(panel);
            }

            let mut panel = self.highlight_panels[highlight_idx].clone();
            panel.set_position(global_rect.position);
            panel.set_size(global_rect.size);
            panel.add_theme_stylebox_override("panel", highlight_style.as_ref());
            panel.show();

            highlight_idx += 1;
        }

        for i in highlight_idx..self.highlight_panels.len() {
            self.highlight_panels[i].hide();
        }

        self.active_highlight_count = highlight_idx;
    }

    fn hide_all_highlights(&mut self) {
        for panel in self.highlight_panels.iter_mut() {
            panel.hide();
        }
        self.active_highlight_count = 0;
    }


    fn on_drag_end(&mut self, rect: Rect2) {
        self.hide_all_highlights();

        self.refresh_layer_children_cache();

        let rect_min_x = rect.position.x;
        let rect_min_y = rect.position.y;
        let rect_max_x = rect.position.x + rect.size.x;
        let rect_max_y = rect.position.y + rect.size.y;

        let mut new_selection = HashSet::with_capacity(
            self.layer_children_cache.len().min(256)
        );

        for child in self.layer_children_cache.iter() {
            if !self.is_node_visible_in_tree(child) {
                continue;
            }

            let child_pos = child.get_position();
            let child_size = child.get_size();
            
            let child_max_x = child_pos.x + child_size.x;
            let child_max_y = child_pos.y + child_size.y;
            
            if !(child_pos.x > rect_max_x || child_max_x < rect_min_x ||
                child_pos.y > rect_max_y || child_max_y < rect_min_y) {
                new_selection.insert(child.clone());
            }
        }

        self.selected_objects = new_selection;
        
        self.update_selection_panel_bounds();
    }


    fn update_selection_panel_bounds(&mut self) {
        let mut sop = self.selected_objects_panel.to_godot_owned();

        self.selected_objects.retain(|obj| obj.is_visible_in_tree());

        if self.selected_objects.is_empty() {
            sop.hide();
            sop.set_size(Vector2::ZERO);
            
            let sp = self.selection_panel.to_godot_owned();
            if !sp.is_visible() {
                self.hide_all_highlights();
            }
            return;
        }

        let mut first = true;
        let mut min_x = 0.0;
        let mut min_y = 0.0;
        let mut max_x = 0.0;
        let mut max_y = 0.0;

        for obj in self.selected_objects.iter() {
            let p = obj.get_position();
            let s = obj.get_size();
            
            if first {
                min_x = p.x;
                min_y = p.y;
                max_x = p.x + s.x;
                max_y = p.y + s.y;
                first = false;
            } else {
                min_x = min_x.min(p.x);
                min_y = min_y.min(p.y);
                max_x = max_x.max(p.x + s.x);
                max_y = max_y.max(p.y + s.y);
            }
        }

        let pos = Vector2::new(min_x, min_y);
        let size = Vector2::new(max_x - min_x, max_y - min_y);

        let base = self.base().to_godot_owned();
        let selected_style = base.get_theme_stylebox_ex("selected_objects_box")
            .theme_type("Nebula2DEditor")
            .done();

        let local_rect = Rect2 { position: pos, size };
        let global_rect = self.rect_to_global(local_rect);

        sop.set_position(global_rect.position);
        sop.set_size(global_rect.size);
        sop.add_theme_stylebox_override("panel", selected_style.as_ref());
        sop.show();
        
        let sp = self.selection_panel.to_godot_owned();
        if sp.is_visible() {
            let sp_local_rect = self.rect_to_local(sp.get_rect());
            self.update_highlight_panels(Some(sp_local_rect));
        }
    }
    
    
    fn reload_theme(&mut self) {
        let mut shader_material = self.editor_shader_material.to_godot_owned();
        let base = self.base().to_godot_owned();
        let mut selection_panel = self.selection_panel.to_godot_owned();
        
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
    
    
    fn rect_to_local(&self, rect: Rect2) -> Rect2 {
        let local_pos = (rect.position + self.editor_global_position) / self.zoom_amount;
        let local_br = (rect.position + rect.size + self.editor_global_position) / self.zoom_amount;

        Rect2::new(local_pos, local_br - local_pos).abs()
    }


    fn rect_to_global(&self, rect: Rect2) -> Rect2 {
        let global_pos = rect.position * self.zoom_amount - self.editor_global_position;
        let global_size = rect.size * self.zoom_amount;

        Rect2::new(global_pos, global_size)
    }


    fn clamp_editor_global_position(&mut self) {
        let zoom = self.zoom_amount;
        let mut pos = (self.editor_global_position + (self.base().get_size() / 2.0)) / zoom;
        
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

        let p = pos * zoom - (self.base().get_size() / 2.0);
        self.xset_editor_global_position(p);
        
        for (c, local_pos) in self.editor_bound_objects.iter_mut() {
            let global_pos = *local_pos * self.zoom_amount - self.editor_global_position;
            c.to_godot_owned().set_position(global_pos);
        }

        self.update_selection_panel_bounds();
        self.signals().editor_panned().emit(p);
    }


    fn xset_editor_global_position(&mut self, pos: Vector2) {
        self.editor_global_position = pos;
        self.editor_position = pos / self.zoom_amount;
        self.signals().editor_panned().emit(pos);
    }

    fn xset_zoom_amount(&mut self, zoom: f32) {
        self.zoom_amount = zoom;
        self.signals().editor_zoomed().emit(zoom);
    }

    #[func]
    pub fn set_zoom(&mut self, zoom: f32, #[opt(default=Vector2::ZERO)] anchor: Vector2) {
        let new_zoom = zoom.clamp(self.zoom_minimum, self.zoom_maximum);
        let old_zoom = self.zoom_amount;

        if (new_zoom - old_zoom).abs() < 0.001 {
            return;
        }

        let world_point = (anchor + self.editor_global_position) / old_zoom;

        self.xset_zoom_amount(new_zoom);
        self.editor_control
            .to_godot_owned()
            .set_scale(Vector2::splat(new_zoom));

        self.editor_global_position = world_point * new_zoom - anchor;

        self.base_mut().call_deferred("_update_shader", &[]);
    }
}