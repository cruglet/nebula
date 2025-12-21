use godot::{classes::{control::{LayoutPreset, SizeFlags}, notify::ControlNotification, tween::{EaseType, TransitionType}, CanvasLayer, ColorRect, Control, Engine, InputEventMouseButton, Label, MarginContainer, PanelContainer, ProgressBar, Shader, ShaderMaterial, Tween, VBoxContainer}, global::MouseButton, prelude::*};

use crate::module::Module;

/// Global singleton class that handles the global runtime of Nebula.
#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Singleton {
    loaded_modules_dict: VarDictionary,
    loaded_modules_arr: Array<Gd<Module>>,
    pub loaded_project_path: GString,
    /// The UI Layer responsible for displaying global UI, such as global windows & toast notifications.
    #[var] pub ui_canvas_layer: Gd<CanvasLayer>,
    screen_canvas_layer: Gd<CanvasLayer>,
    screen_blur_rect: Gd<ColorRect>,
    loaded_shaders_dict: VarDictionary,
    base: Base<Node>
}


#[godot_api]
impl INode for Singleton {
    fn init(base: Base<Node>) -> Self {
        let mut loaded_shaders_dict = VarDictionary::new();

        let mut blur_shader: Gd<Shader> = Shader::new_gd();
        blur_shader.set_code(Self::SHADER_BLUR_CODE);

        let mut editor_2d_grid_shader: Gd<Shader> = Shader::new_gd();
        editor_2d_grid_shader.set_code(Self::SHADER_EDITOR_2D_GRID_CODE);

        loaded_shaders_dict.set(Singleton::SHADER_BLUR, blur_shader);
        loaded_shaders_dict.set(Singleton::SHADER_EDITOR_2D_GRID, editor_2d_grid_shader);

        Self {
            loaded_modules_dict: VarDictionary::new(),
            loaded_modules_arr: Array::new(),
            loaded_project_path: GString::new(),
            ui_canvas_layer: CanvasLayer::new_alloc(),
            screen_canvas_layer: CanvasLayer::new_alloc(),
            screen_blur_rect: ColorRect::new_alloc(),
            loaded_shaders_dict,
            base,
        }
    }
    fn ready(&mut self) {
        let mut ui_canvas = self.ui_canvas_layer.to_godot_owned();
        ui_canvas.set_layer(10);
        self.base_mut().add_child(&ui_canvas);

        let mut screen_canvas = self.screen_canvas_layer.to_godot_owned();
        screen_canvas.set_layer(5);
        self.base_mut().add_child(&screen_canvas);

        let mut scr_blur_rect = self.screen_blur_rect.to_godot_owned();
        scr_blur_rect.set_anchors_preset(LayoutPreset::FULL_RECT);
        scr_blur_rect.notify(ControlNotification::RESIZED);
        scr_blur_rect.hide();

        let mut scr_blur_mat: Gd<ShaderMaterial> = ShaderMaterial::new_gd();
        let scr_blur_shader: Gd<Shader> = self.get_shader(Singleton::SHADER_BLUR);
        scr_blur_mat.set_shader(&scr_blur_shader);
        scr_blur_rect.set_material(&scr_blur_mat);

        screen_canvas.add_child(&scr_blur_rect);
    }
}

#[allow(dead_code)]
#[godot_api]
impl Singleton {
    /// The enumerated constant for a blur shader, used in shader helper functions.
    #[constant] pub const SHADER_BLUR: i32 = 0;
    /// The enumerated constant for a 2d grid shader, used in shader helper functions.
    #[constant] pub const SHADER_EDITOR_2D_GRID: i32 = 1;

    /// Returns an instance of a Singleton shader used throughout Nebula.
    #[func]
    pub fn get_shader(&mut self, shader: i32) -> Gd<Shader> {
        if let Some(shd) = self.loaded_shaders_dict.get(shader) {
            return shd.try_to().unwrap()
        };
        godot_error!("Could not load shader!");
        Shader::new_gd()
    }

    /// Returns the raw code of a Singleton shader as a String.
    #[func]
    pub fn get_shader_code(&mut self, shader: i32) -> GString {
        match shader {
            Self::SHADER_BLUR => Self::SHADER_BLUR_CODE.into(),
            Self::SHADER_EDITOR_2D_GRID => Self::SHADER_EDITOR_2D_GRID_CODE.into(),
            _ => "".into()
        }
    }

    /// Applies the screen blur to everything below the Screen FX CanvasLayer (which exists on layer 5).
    #[func]
    pub fn show_screen_blur(&mut self) {
        let fade_in_time = 0.25;
        let blur_amount = 2.5;

        let mut shader_mat = self.screen_blur_rect.get_material().unwrap().try_cast::<ShaderMaterial>().unwrap();
        
        shader_mat.set_shader_parameter(&StringName::from("blur_amount"), &0.0.to_variant());

        if let Some(mut blur_in_tween) = self.base_mut().create_tween() {
            blur_in_tween.tween_property(&shader_mat, "shader_parameter/blur_amount", &blur_amount.to_variant(), fade_in_time);
        }

        self.screen_blur_rect.show();
    }

    /// Hides the screen blur from the ScreenFX CanvasLayer (which exists on layer 5).
    #[func]
    pub fn hide_screen_blur(&mut self) {
        let fade_out_time = 0.25;

        let shader_mat = self.screen_blur_rect.get_material().unwrap().try_cast::<ShaderMaterial>().unwrap();

        let mut rect = self.screen_blur_rect.to_godot_owned();
        
        if let Some(mut blur_in_tween) = self.base_mut().create_tween() {
            blur_in_tween.tween_property(&shader_mat, "shader_parameter/blur_amount", &0.0.to_variant(), fade_out_time);
            blur_in_tween.signals().finished().connect(move || {
                rect.hide();
            });
        }
    }
    
    /// Makes a notification appear for a short amount of time. Useful for alerting the user towards an error
    /// or an important piece of information.
    #[func]
    pub fn send_notification(
        &mut self,
        title: GString,
        description: GString,
        #[opt(default = 4.5)] time: f32,
        #[opt(default = true)] show_progress: bool,
        ) {
        let mut notification_panel: Gd<PanelContainer> = PanelContainer::new_alloc();
        notification_panel.set_custom_minimum_size(Vector2 { x: 400.0, y: 70.0 });
        notification_panel.set_theme_type_variation(&StringName::from("nPanelNotification"));
        let panel_ref = notification_panel.to_godot_owned();
        notification_panel.signals().gui_input().connect(move |event| {
            if let Ok(e) = event.try_cast::<InputEventMouseButton>() && e.get_button_index() == MouseButton::LEFT && e.is_pressed() {
                Singleton::animate_notification_out(panel_ref.to_godot_owned());
            }
        });

        let mut main_vbox: Gd<VBoxContainer> = VBoxContainer::new_alloc();
        main_vbox.set_v_size_flags(SizeFlags::EXPAND_FILL);
        notification_panel.add_child(&main_vbox);

        let mut notification_margin_container: Gd<MarginContainer> = MarginContainer::new_alloc();
        main_vbox.add_child(&notification_margin_container);

        let mut notification_vbox: Gd<VBoxContainer> = VBoxContainer::new_alloc();
        notification_margin_container.add_theme_constant_override("margin_left", 8);
        notification_margin_container.add_theme_constant_override("margin_top", 8);
        notification_margin_container.add_theme_constant_override("margin_right", 8);
        notification_margin_container.add_child(&notification_vbox);

        let mut notification_title_header: Gd<Label> = Label::new_alloc();
        notification_title_header.set_text(&title);
        notification_title_header.add_theme_font_size_override("font_size", 20);
        notification_vbox.add_child(&notification_title_header);

        let mut notification_description: Gd<Label> = Label::new_alloc();
        notification_description.set_text(&description);
        notification_vbox.add_child(&notification_description);

        let mut filler: Gd<Control> = Control::new_alloc();
        filler.set_v_size_flags(SizeFlags::EXPAND_FILL);
        main_vbox.add_child(&filler);

        let mut notification_progress_bar: Gd<ProgressBar> = ProgressBar::new_alloc();
        notification_progress_bar.set_max(time.into());
        notification_progress_bar.set_show_percentage(false);
        notification_progress_bar.set_custom_minimum_size(Vector2 { x: 0.0, y: 3.0 });
        if !show_progress {
            notification_progress_bar.hide();
        }
        main_vbox.add_child(&notification_progress_bar);

        // signal on timeout
        notification_panel.set_anchor(Side::LEFT, 1.0);
        notification_panel.set_anchor(Side::TOP, 0.0);
        notification_panel.set_anchor(Side::RIGHT, 1.0);
        notification_panel.set_anchor(Side::BOTTOM, 0.0);

        let pos = notification_panel.get_position();
        notification_panel.set_position(Vector2 { x: pos.x, y: 30.0 });

        let mut canvas_layer = self.ui_canvas_layer.to_godot_owned();
        canvas_layer.add_child(&notification_panel);

        let notification_time_tween_op: Option<Gd<Tween>> = self.base_mut().create_tween();

        if let Some(mut notification_tween) = notification_time_tween_op {
            notification_tween.tween_property(&notification_progress_bar, "value",&time.to_variant(), time.into());
            notification_tween.signals().finished().connect({
            let panel_ref = notification_panel.to_godot_owned();
            move || {
                if panel_ref.is_instance_valid() {
                    Singleton::animate_notification_out(panel_ref.to_godot_owned());
                }
            }
        });
        }

        Singleton::animate_notification_in(notification_panel);
    }

    fn animate_notification_in(&mut notification_panel: Gd<PanelContainer>) {
        const NOTIFICATION_MARGIN_RIGHT: f32 = 30.0;
        const ANIMATION_TIME: f32 = 0.267;

        let tween_op: Option<Gd<Tween>> = notification_panel.create_tween();
        let notification_size = notification_panel.get_size();

        notification_panel.set_modulate(Color::TRANSPARENT_WHITE);
        
        if let Some(mut tween) = tween_op {
            tween.set_parallel();
            tween.set_ease(EaseType::OUT);
            tween.set_trans(TransitionType::CUBIC);
            tween.tween_property(&notification_panel, "offset_left", &(-notification_size.x - NOTIFICATION_MARGIN_RIGHT).to_variant(), ANIMATION_TIME as f64);
            tween.tween_property(&notification_panel, "offset_right", &(-NOTIFICATION_MARGIN_RIGHT).to_variant(), ANIMATION_TIME as f64);
            tween.tween_property(&notification_panel, "modulate", &Color::WHITE.to_variant(), ANIMATION_TIME as f64);
        }
    }

    fn animate_notification_out(&mut notification_panel: Gd<PanelContainer>) {
        const ANIMATION_TIME: f32 = 0.267;

        let tween_op: Option<Gd<Tween>> = notification_panel.create_tween();
        let pos = notification_panel.get_position();
        
        if let Some(mut tween) = tween_op {
            tween.set_parallel();
            tween.set_ease(EaseType::OUT);
            tween.set_trans(TransitionType::CUBIC);
            tween.tween_property(&notification_panel, "position", &(Vector2 { x: pos.x, y: pos.y - 30.0 }).to_variant(), ANIMATION_TIME.into());
            tween.tween_property(&notification_panel, "modulate", &Color::TRANSPARENT_WHITE.to_variant(), ANIMATION_TIME.into());
            tween.signals().finished().connect(move || {
                let _ = &notification_panel.queue_free();
            });
        } 
    }

    /// Registers a module within the engine, adding it to a list to be able to be grabbed later.
    #[func]
    pub fn register_module(&mut self, module: Gd<Module>) {
        self.loaded_modules_dict.set(module.bind().get_module_id(), self.loaded_modules_arr.len() as i32);
        self.loaded_modules_arr.push(&module);
    }

    /// Gets a registered/loaded module. You can grab a module from either it's "id" or the "index" 
    /// of the module in the stored list.
    #[func]
    pub fn get_module(&mut self, id_or_index: Variant) -> Gd<Module> {
        let index_opt: Option<usize> = if let Ok(module_id) = id_or_index.try_to::<GString>() {
            self.loaded_modules_dict
                .get(module_id)
                .and_then(|v| v.try_to::<i32>().ok())
                .map(|i| i as usize)
        } else if let Ok(module_index) = id_or_index.try_to::<i32>() {
            if module_index >= 0 {
                Some(module_index as usize)
            } else {
                None
            }
        } else {
            None
        };

        if let Some(index) = index_opt {
            if let Some(module) = self.loaded_modules_arr.get(index) {
                return module.to_godot_owned();
            }
        }

        Module::new()
    }

    /// Removes a module from the internal stored list, ether by "id" or by its "index" in said list.
    #[func]
    pub fn remove_module(&mut self, id_or_index: Variant) {
        let mut index: i32 = 0;
        if let Ok(module_id) = id_or_index.try_to::<GString>() {
            if let Some(i) = self.loaded_modules_dict.get(module_id) {
                index = i.try_to::<i32>().unwrap();
            }
        } else if let Ok(module_index) = id_or_index.try_to::<i32>() {
            index = module_index;
        }

        if let Some(module) = self.loaded_modules_arr.get(index as usize) {
            self.loaded_modules_arr.remove(index as usize);
            self.loaded_modules_dict.remove(module.bind().get_module_id());
        }
    }

    /// Returns the list of all currently registered/loaded modules.
    #[func]
    pub fn get_modules(&mut self) -> Array<Gd<Module>> {
        self.loaded_modules_arr.to_godot_owned()
    }

    /// Returns a list of all module id's in the order that the modules are registered in.
    #[func]
    pub fn get_module_ids(&self) -> Array<GString> {
        self.loaded_modules_dict
            .keys_array()
            .iter_shared()
            .map(|v| v.to::<GString>())
            .collect()
    }

    pub fn singleton() -> Gd<Self> {
        Engine::singleton().get_singleton(&Self::class_id().to_string_name()).unwrap().cast::<Self>()
    }

    pub fn register(&self) {
        Engine::singleton().register_singleton(&Self::class_id().to_string_name(), &self.to_gd());
    }

    pub fn unregister(&mut self) {
        Engine::singleton().unregister_singleton(&Self::class_id().to_string_name());
    }

    pub fn get_tree() -> Gd<SceneTree> {
        let engine = Engine::singleton();
        let tree = engine.get_main_loop().unwrap();
        let scene_tree: Gd<SceneTree> = tree.try_cast::<SceneTree>().unwrap();
        if !Singleton::singleton().is_inside_tree() {
            scene_tree.get_root().unwrap().add_child(&Singleton::singleton());
        };
        
        scene_tree
    }

    pub fn get_window_size() -> Vector2i {
        if let Some(r) = Singleton::get_tree().get_root() {
            return r.get_size();
        }

        Vector2i::from_tuple((0, 0))
    }

    pub fn get_scaled_window_size() -> Vector2i {
        if let Some(r) = Singleton::get_tree().get_root() {
            return (Singleton::get_window_size().cast_float() / r.get_content_scale_factor()).cast_int();
        }

        Singleton::get_window_size()
    }

    pub fn get_window_center() -> Vector2i {
        let window_size: Vector2i = Singleton::get_window_size();
        Vector2i { x: window_size.x / 2, y: window_size.y / 2 }
    }


    pub fn get_scale_factor() -> f32 {
        if let Some(w) = Singleton::get_tree().get_root() {
            return w.get_content_scale_factor();
        }
        return 1.0
    }

    pub fn centerize(container_size: Vector2i, rect_size: Vector2i) -> Vector2i {
        let center = Vector2i {x: container_size.x / 2, y: container_size.y / 2};
        let offset = Vector2i {x: rect_size.x / 2, y: rect_size.y / 2};

        Vector2i {x: center.x - offset.x, y: center.y - offset.y}
    }

const SHADER_BLUR_CODE: &'static str = r#"
shader_type canvas_item;

uniform sampler2D screen_tex : hint_screen_texture, filter_linear;
uniform float blur_amount : hint_range(0.0, 10.0) = 2.0;
uniform int blur_quality : hint_range(1, 4) = 3;
uniform float dither_strength : hint_range(0.0, 1.0) = 0.1;

void fragment() {
    vec2 tex_size = 1.0 / vec2(textureSize(screen_tex, 0));
    vec4 col = vec4(0.0);
    float total_weight = 0.0;

    int half_kernel = blur_quality;
    float sigma = blur_amount * 0.4;
    float sigma_sq = sigma * sigma;

    for (int x = -half_kernel; x <= half_kernel; x++) {
        for (int y = -half_kernel; y <= half_kernel; y++) {
            vec2 offset = vec2(float(x), float(y)) * tex_size * blur_amount;

            float distance_sq = float(x * x + y * y);
            float weight = exp(-distance_sq / (2.0 * sigma_sq));

            col += texture(screen_tex, SCREEN_UV + offset) * weight;
            total_weight += weight;
        }
    }

    col = col / total_weight;

    float noise = fract(sin(dot(SCREEN_UV, vec2(12.9898, 78.233))) * 43758.5453);
    col.rgb += (noise - 0.5) * dither_strength / 255.0;

    COLOR = col;
}
"#;


const SHADER_EDITOR_2D_GRID_CODE: &'static str = r#"
shader_type canvas_item;
uniform float zoom = 1.0;
uniform float scale_factor = 1.0;
uniform vec2 minor_spacing = vec2(32.0, 32.0);
uniform vec2 major_spacing = vec2(256.0, 256.0);
uniform float minor_line_width = 1.0;
uniform float major_line_width = 2.0;
uniform int grid_pattern = 1;
uniform bool fade_minor = true;
uniform bool fade_major = false;
uniform vec4 grid_minor : source_color = vec4(0.2, 0.2, 0.2, 0.5);
uniform vec2 grid_offset = vec2(0.0);
uniform vec4 grid_major : source_color = vec4(0.2, 0.2, 0.2, 1.0);
uniform vec2 position = vec2(0.0);
uniform float dot_minor_radius_px = 2.0;
uniform float dot_major_radius_px = 4.0;
uniform int dot_major_step = 4;
uniform bool draw_grid_outside_bounds = false;
uniform float bound_left = -1000.0;
uniform float bound_right = 1000.0;
uniform float bound_bottom = -1000.0;
uniform float bound_top = 1000.0;
uniform vec4 bound_outside_color : source_color = vec4(0.0, 0.0, 0.0, 0.5);
uniform vec4 background_color : source_color = vec4(0.0, 0.0, 0.0, 0.0);

void fragment() {
    vec2 scaled_fragcoord = FRAGCOORD.xy / scale_factor;
    vec2 world_pos = (scaled_fragcoord + position) / zoom;
    vec2 minor_uv = ((world_pos + grid_offset) / vec2(minor_spacing.x, minor_spacing.y));
    vec2 major_uv = ((world_pos + grid_offset) / vec2(major_spacing.x, major_spacing.y));
    vec2 minor_cell = fract(minor_uv);
    vec2 major_cell = fract(major_uv);
    
    vec4 col = background_color;
    
    bool out_of_bounds = (world_pos.x < bound_left || world_pos.x > bound_right ||
                          world_pos.y < -bound_top || world_pos.y > -bound_bottom);
    
    col = out_of_bounds ? bound_outside_color : background_color;
    
    if (!out_of_bounds || draw_grid_outside_bounds) {
        float minor_alpha = fade_minor ? smoothstep(0.3, 0.8, zoom) : 1.0;
        float major_alpha = fade_major ? smoothstep(0.3, 0.8, zoom) : 1.0;
        
        if (grid_pattern == 1) {
            vec2 minor_thickness = vec2(minor_line_width / zoom) / vec2(minor_spacing.x, minor_spacing.y);
            vec2 major_thickness = vec2(major_line_width / zoom) / vec2(major_spacing.x, major_spacing.y);
            
            float minor_x = step(minor_cell.x, minor_thickness.x);
            float minor_y = step(minor_cell.y, minor_thickness.y);
            float minor_mask = max(minor_x, minor_y);
            if (minor_mask > 0.0) {
                float alpha = fade_minor ? minor_alpha : 1.0;
                col = mix(col, grid_minor, grid_minor.a * alpha);
            }
            
            float major_x = step(major_cell.x, major_thickness.x);
            float major_y = step(major_cell.y, major_thickness.y);
            float major_mask = max(major_x, major_y);
            if (major_mask > 0.0) {
                float alpha = fade_major ? major_alpha : 1.0;
                col = mix(col, grid_major, grid_major.a * alpha);
            }
        }
        else if (grid_pattern == 2) {
            float dot_minor_radius = dot_minor_radius_px / zoom;
            float dot_major_radius = dot_major_radius_px / zoom;
            vec2 minor_grid_pos = (world_pos + grid_offset) / vec2(minor_spacing.x, minor_spacing.y);
            vec2 dot_cell = fract(minor_grid_pos);
            vec2 dot_center = vec2(0.5);
            float dist = distance(dot_cell, dot_center);
            
            ivec2 dot_index = ivec2(floor(minor_grid_pos));
            bool is_major = ((dot_index.x % dot_major_step) == 0) &&
                            ((dot_index.y % dot_major_step) == 0);
            
            if (is_major && dist < (dot_major_radius / min(minor_spacing.x, minor_spacing.y))) {
                float alpha = fade_major ? major_alpha : 1.0;
                col = mix(col, grid_major, grid_major.a * alpha);
            } else if (!is_major && dist < (dot_minor_radius / min(minor_spacing.x, minor_spacing.y))) {
                float alpha = fade_minor ? minor_alpha : 1.0;
                col = mix(col, grid_minor, grid_minor.a * alpha);
            }
        }
    }
    
    COLOR = col;
}
"#;
}
