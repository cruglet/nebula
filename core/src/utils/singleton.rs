use godot::{classes::{control::{LayoutPreset, SizeFlags}, notify::ControlNotification, tween::{EaseType, TransitionType}, CanvasLayer, ColorRect, Control, Engine, InputEventMouseButton, Label, MarginContainer, PanelContainer, ProgressBar, Shader, ShaderMaterial, Tween, VBoxContainer}, global::MouseButton, prelude::*};

use crate::utils::module::Module;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Singleton {
    loaded_modules_dict: Dictionary,
    loaded_modules_arr: Array<Gd<Module>>,
    pub loaded_project_path: GString,
    ui_canvas_layer: Gd<CanvasLayer>,
    screen_canvas_layer: Gd<CanvasLayer>,
    screen_blur_rect: Gd<ColorRect>,
    loaded_shaders_dict: Dictionary,
    base: Base<Node>
}


#[godot_api]
impl INode for Singleton {
    fn init(base: Base<Node>) -> Self {
        let mut loaded_shaders_dict = Dictionary::new();

        let mut blur_shader: Gd<Shader> = Shader::new_gd();
        blur_shader.set_code(Self::SHADER_BLUR_CODE);

        loaded_shaders_dict.set(Singleton::SHADER_BLUR, blur_shader);

        Self {
            loaded_modules_dict: Dictionary::new(),
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
        let mut ui_canvas = self.ui_canvas_layer.to_godot();
        ui_canvas.set_layer(10);
        self.base_mut().add_child(&ui_canvas);

        let mut screen_canvas = self.screen_canvas_layer.to_godot();
        screen_canvas.set_layer(5);
        self.base_mut().add_child(&screen_canvas);

        let mut scr_blur_rect = self.screen_blur_rect.to_godot();
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
    #[constant] pub const SHADER_BLUR: i32 = 0;

    #[func]
    pub fn get_shader(&mut self, shader: i32) -> Gd<Shader> {
        if let Some(shd) = self.loaded_shaders_dict.get(shader) {
            return shd.try_to().unwrap()
        };
        godot_error!("Could not load shader!");
        Shader::new_gd()
    }

    #[func]
    pub fn get_shader_code(&mut self, shader: i32) -> GString {
        match shader {
            Self::SHADER_BLUR => Self::SHADER_BLUR_CODE.into(),
            _ => "".into()
        }
    }

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

    #[func]
    pub fn hide_screen_blur(&mut self) {
        let fade_out_time = 0.25;

        let shader_mat = self.screen_blur_rect.get_material().unwrap().try_cast::<ShaderMaterial>().unwrap();

        let mut rect = self.screen_blur_rect.to_godot();
        
        if let Some(mut blur_in_tween) = self.base_mut().create_tween() {
            blur_in_tween.tween_property(&shader_mat, "shader_parameter/blur_amount", &0.0.to_variant(), fade_out_time);
            blur_in_tween.signals().finished().connect(move || {
                rect.hide();
            });
        }
    }
    
    #[func]
    pub fn send_notification(&mut self, title: GString, description: GString) {
        const NOTIFICATION_TIME: f32 = 4.5;

        let mut notification_panel: Gd<PanelContainer> = PanelContainer::new_alloc();
        notification_panel.set_custom_minimum_size(Vector2 { x: 400.0, y: 70.0 });
        notification_panel.set_theme_type_variation(&StringName::from("nPanelNotification"));
        let panel_ref = notification_panel.to_godot();
        notification_panel.signals().gui_input().connect(move |event| {
            if let Ok(e) = event.try_cast::<InputEventMouseButton>() && e.get_button_index() == MouseButton::LEFT && e.is_pressed() {
                Singleton::animate_notification_out(panel_ref.to_godot());
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
        notification_progress_bar.set_max(NOTIFICATION_TIME.into());
        notification_progress_bar.set_show_percentage(false);
        notification_progress_bar.set_custom_minimum_size(Vector2 { x: 0.0, y: 3.0 });
        main_vbox.add_child(&notification_progress_bar);

        // signal on timeout
        notification_panel.set_anchor(Side::LEFT, 1.0);
        notification_panel.set_anchor(Side::TOP, 0.0);
        notification_panel.set_anchor(Side::RIGHT, 1.0);
        notification_panel.set_anchor(Side::BOTTOM, 0.0);

        let pos = notification_panel.get_position();
        notification_panel.set_position(Vector2 { x: pos.x, y: 30.0 });

        let mut canvas_layer = self.ui_canvas_layer.to_godot();
        canvas_layer.add_child(&notification_panel);

        let notification_time_tween_op: Option<Gd<Tween>> = self.base_mut().create_tween();

        if let Some(mut notification_tween) = notification_time_tween_op {
            notification_tween.tween_property(&notification_progress_bar, "value",&NOTIFICATION_TIME.to_variant(), NOTIFICATION_TIME.into());
            notification_tween.signals().finished().connect({
            let panel_ref = notification_panel.to_godot();
            move || {
                if panel_ref.is_instance_valid() {
                    Singleton::animate_notification_out(panel_ref.to_godot());
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

    #[func]
    pub fn register_module(&mut self, module: Gd<Module>) {
        self.loaded_modules_dict.set(module.bind().get_module_id(), self.loaded_modules_arr.len() as i32);
        self.loaded_modules_arr.push(&module);
    }

    #[func]
    pub fn get_module(&mut self, id_or_index: Variant) -> Gd<Module> {
        let mut index: i32 = 0;
        if let Ok(module_id) = id_or_index.try_to::<GString>() {
            if let Some(i) = self.loaded_modules_dict.get(module_id) {
                index = i.try_to::<i32>().unwrap();
            }
        } else if let Ok(module_index) = id_or_index.try_to::<i32>() {
            index = module_index;
        }

        if let Some(module) = self.loaded_modules_arr.get(index as usize) {
            return module.to_godot();
        }
        Module::new()
    }

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

    #[func]
    pub fn get_modules(&mut self) -> Array<Gd<Module>> {
        self.loaded_modules_arr.to_godot()
    }

    #[func]
    pub fn get_module_ids(&self) -> Array<GString> {
        self.loaded_modules_dict
            .keys_array()
            .iter_shared()
            .map(|v| v.to::<GString>())
            .collect()
    }

    pub fn singleton() -> Gd<Self> {
        Engine::singleton().get_singleton(&Self::class_name().to_string_name()).unwrap().cast::<Self>()
    }

    pub fn register(&self) {
        Engine::singleton().register_singleton(&Self::class_name().to_string_name(), &self.to_gd());
    }

    pub fn unregister(&mut self) {
        Engine::singleton().unregister_singleton(&Self::class_name().to_string_name());
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
}
