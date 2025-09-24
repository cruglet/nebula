use godot::{classes::{control::{LayoutPreset, MouseFilter}, Control, Engine, IControl, Input, InputEvent, InputEventKey, InputEventMagnifyGesture, InputEventMouseButton, InputEventMouseMotion, InputEventPanGesture, Panel, Shader, ShaderMaterial}, global::{Key, MouseButton}, meta::PropertyInfo, obj::{NewAlloc, WithBaseField}, prelude::*};
use godot::global::MouseButtonMask;

use crate::utils::singleton::Singleton;

#[derive(GodotClass)]
#[class(base=Control, tool)]
struct Nebula2DEditor {
    #[var] zoom_minimum: f32,
    #[var] zoom_maximum: f32,
    #[var] zoom_amount: f32,
    #[var] zoom_step: f32,

    #[var] grid_type: GridType,

    #[var] bound_left: i32,
    #[var] bound_right: i32,
    #[var] bound_top: i32,
    #[var] bound_bottom: i32,
    #[var] bound_outside_color: Color,

    ctrl_zoom_anchor: Option<Vector2>,

    viewport_control: Gd<Control>,
    viewport_panel: Gd<Panel>,
    viewport_position: Vector2,
    viewport_shader_material: Gd<ShaderMaterial>,
    base: Base<Control>,
}


#[derive(GodotConvert, Var, Export, Clone, Copy)]
#[godot(via = i32)]
enum GridType {
    None,
    Lines,
    Dots
}

#[godot_api]
impl IControl for Nebula2DEditor {
    fn init(base: Base<Control>) -> Self {
        Self {
            zoom_minimum: 0.2,
            zoom_maximum: 4.0,
            zoom_amount: 1.0,
            zoom_step: 0.15,

            grid_type: GridType::Lines,

            bound_left: -999999,
            bound_right: 999999,
            bound_top: 999999,
            bound_bottom: -999999,
            bound_outside_color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.5 },

            ctrl_zoom_anchor: None,

            viewport_control: Control::new_alloc(),
            viewport_panel: Panel::new_alloc(),
            viewport_position: Vector2::ZERO,
            viewport_shader_material: ShaderMaterial::new_gd(),
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
        base_control.move_child(&pn, 0); // this will make stuff appear properly in the editor
    
        let shader = Singleton::singleton().bind_mut().get_shader(Singleton::SHADER_EDITOR_2D_GRID);
        let mut material = ShaderMaterial::new_gd();
        self.viewport_shader_material = material.to_godot();
        material.set_shader(&shader);

        if !Engine::singleton().is_editor_hint() {
            material.set_shader_parameter("bound_left", &Variant::from(self.get_bound_left()));
            material.set_shader_parameter("bound_right", &Variant::from(self.get_bound_right()));
            material.set_shader_parameter("bound_top", &Variant::from(self.get_bound_top()));
            material.set_shader_parameter("bound_bottom", &Variant::from(self.get_bound_bottom()));
            material.set_shader_parameter("bound_outside_color", &Variant::from(self.get_bound_outside_color()));
        }
        pn.set_material(&material);

    }
    
    fn gui_input(&mut self, event: Gd<InputEvent>) {
        let mut vref = self.viewport_control.to_godot();
        
        if let Ok(e) = event.to_godot().try_cast::<InputEventMouseMotion>() {
            if e.get_button_mask() == MouseButtonMask::MIDDLE {
                self.viewport_position += e.get_relative();
                vref.set_position(self.viewport_position);
            }
        }
        else if let Ok(e) = event.to_godot().try_cast::<InputEventPanGesture>() {
            self.viewport_position -= e.get_delta() * 35.0;
            vref.set_position(self.viewport_position);
        }
        else if let Ok(e) = event.to_godot().try_cast::<InputEventMouseButton>() && e.is_pressed() {
            if e.get_button_index() != MouseButton::WHEEL_UP && e.get_button_index() != MouseButton::WHEEL_DOWN {
                return;
            }
            
            let zoom_amount = match e.get_button_index() {
                MouseButton::WHEEL_UP => 1.0 + self.zoom_step,
                MouseButton::WHEEL_DOWN => 1.0 / (1.0 + self.zoom_step),
                _ => return,
            };
            
            let ctrl = Input::singleton().is_key_pressed(Key::CTRL);
            let anchor_local = if ctrl {
                if let Some(anchor) = self.ctrl_zoom_anchor {
                    anchor
                } else {
                    let anchor = Self::mouse_to_local(&vref, e.get_position());
                    self.ctrl_zoom_anchor = Some(anchor);
                anchor
            }
            } else {
                self.ctrl_zoom_anchor = None;
                Self::mouse_to_local(&vref, e.get_position())
            };

            let zoom_result = Self::zoom_anchored(
                vref,
                anchor_local,
                zoom_amount,
                self.zoom_minimum,
                self.zoom_maximum,
            );
            self.viewport_position += zoom_result.0;
            self.zoom_amount = zoom_result.1;
        }
        else if let Ok(e) = event.to_godot().try_cast::<InputEventMagnifyGesture>() {
            let anchor_local = Self::mouse_to_local(&vref, e.get_position());
            let zoom_result = Self::zoom_anchored(vref, anchor_local, e.get_factor(), self.zoom_minimum, self.zoom_maximum);
            self.viewport_position += zoom_result.0;
            self.zoom_amount = zoom_result.1;
        }
        
        self.viewport_shader_material.set_shader_parameter("position", &self.viewport_position.to_variant());
        self.viewport_shader_material.set_shader_parameter("zoom", &self.zoom_amount.to_variant());
    }

    fn get_property_list(&mut self) -> Vec<PropertyInfo> {
        let props = vec![
            PropertyInfo::new_group("Zoom", "zoom_"),
            PropertyInfo::new_export::<f32>("zoom_minimum"),
            PropertyInfo::new_export::<f32>("zoom_maximum"),
            PropertyInfo::new_export::<f32>("zoom_amount"),
            PropertyInfo::new_export::<f32>("zoom_step"),

            PropertyInfo::new_group("Grid", "grid_"),
            PropertyInfo::new_export::<GridType>("grid_type"),

            PropertyInfo::new_group("Bounds", "bound_"),
            PropertyInfo::new_export::<i32>("bound_left"),
            PropertyInfo::new_export::<i32>("bound_right"),
            PropertyInfo::new_export::<i32>("bound_top"),
            PropertyInfo::new_export::<i32>("bound_bottom"),
            PropertyInfo::new_export::<Color>("bound_outside_color"),
        ];

        props
    }

    fn set_property(&mut self, name: StringName, value: Variant) -> bool {
        if name.eq(&StringName::from("zoom_amount")) && !Engine::singleton().is_editor_hint() {
            self.zoom_amount = value.try_to().expect("Unable to convert f32 to type Variant");
            Self::zoom_anchored(self.base().to_godot().upcast(), (self.base().get_size() / 2.0) + self.base().get_position(), self.zoom_amount, self.zoom_minimum, self.zoom_maximum);
            return true;
        }

        if name.eq(&StringName::from("grid_type")) && !Engine::singleton().is_editor_hint() {
            let grid_type: i32 = value.try_to().unwrap();
            let mut material: Gd<ShaderMaterial> = self.viewport_panel.get_material().unwrap().try_cast::<ShaderMaterial>().unwrap();

            material.set_shader_parameter("grid_pattern", &(grid_type as f32).to_variant());
        }

        if name.contains("bound_") && !Engine::singleton().is_editor_hint() {
            let mut material: Gd<ShaderMaterial> = self.viewport_panel.get_material().unwrap().try_cast::<ShaderMaterial>().unwrap();
            material.set_shader_parameter(&name, &value);
        }

        false
    }
}

#[godot_api]
impl Nebula2DEditor {
    pub fn zoom_anchored(mut control: Gd<Control>, zoom_anchor_position: Vector2, zoom_amount: f32, zoom_minimum: f32, zoom_maximum: f32) -> (Vector2, f32) {
        let old_scale: Vector2 = control.get_scale();
        let old_pos: Vector2 = control.get_position();
        
        let global_anchor_before: Vector2 = old_pos + zoom_anchor_position * old_scale;
        
        let mut new_scale_value = old_scale.x * zoom_amount;
        new_scale_value = new_scale_value.clamp(zoom_minimum, zoom_maximum);
        let new_scale = Vector2::splat(new_scale_value);
        
        control.set_scale(new_scale);
        
        let global_anchor_after: Vector2 = control.get_position() + zoom_anchor_position * new_scale;
        let delta = global_anchor_before - global_anchor_after;
        
        control.to_godot().set_position(control.get_position() + delta);
        
        (delta, new_scale.x)
    }

    pub fn mouse_to_local(control: &Gd<Control>, mouse_pos: Vector2) -> Vector2 {
        (mouse_pos - control.get_position()) / control.get_scale()
    }
}
