use godot::classes::control::LayoutPreset;
use godot::obj::{NewAlloc, WithBaseField, WithUserSignals};
use godot::prelude::*;
use godot::global::{Key, HorizontalAlignment, VerticalAlignment};
use godot::classes::{Control, PanelContainer, IPanelContainer, MarginContainer, Tween, SceneTree, tween, InputEvent, InputEventKey, VBoxContainer, Label, Button, control::{FocusMode, SizeFlags}};

use crate::utils::singleton::Singleton;

#[derive(GodotConvert, Var, Export, Clone, Copy)]
#[godot(via = i32)]
enum ShowAnimation {
    ScaleInCenter,
    None,
}

#[derive(GodotConvert, Var, Export, Clone, Copy)]
#[godot(via = i32)]
enum HideAnimation {
    ScaleOutCenter,
    None,
}

#[derive(GodotConvert, Var, Export, Clone, Copy)]
#[godot(via = i32)]
enum ScreenFX {
    None,
    Blur,
}

impl From<i32> for ScreenFX {
    fn from(value: i32) -> Self {
        match value {
            1 => ScreenFX::Blur,
            _ => ScreenFX::None,
        }
    }
}

/// A customizable window with animations, title, and optional screen effects.
///
/// [member NebulaWindow] can show/hide itself with scale animations, keep itself centered,
/// and optionally apply blur effects to the background. It supports keyboard input
/// for closing with the Escape key and allows flexible layout for child controls.
#[derive(GodotClass)]
#[class(base=PanelContainer)]
struct NebulaWindow {
    /// Window title text displayed at the top.
    #[export] title_text: GString,

    /// Font size for the window title.
    #[export] title_text_size: i32,

    /// Margin around the title text.
    #[export] title_margin: i32,

    /// Margin around the window border.
    #[export] border_margin: i32,

    /// Animation to use when showing the window.
    #[export] show_animation: ShowAnimation,

    /// Animation to use when hiding the window.
    #[export] hide_animation: HideAnimation,

    /// Speed of the show animation (seconds).
    #[export] animation_in_speed: f64,

    /// Speed of the hide animation (seconds).
    #[export] animation_out_speed: f64,

    /// Initial position offset for the window.
    #[export] start_origin: Vector2,

    /// Whether the window can go fullscreen
    #[export] can_fullscreen: bool,

    /// If true, pressing Escape will close the window.
    #[export] close_on_escape: bool,

    /// If true, window stays centered during processing.
    #[export] keep_centered: bool,

    /// Optional screen effects when window is visible.
    #[export] screen_fx: ScreenFX,

    scene_origin: Option<Gd<Node>>,
    fullscreen_on: bool,
    original_size: Vector2,
    original_position: Vector2,

    vbox: Gd<VBoxContainer>,
    fullscreen_button: Gd<Button>,
    base: Base<PanelContainer>
}

#[godot_api]
impl IPanelContainer for NebulaWindow {
    fn init(base: Base<PanelContainer>) -> Self {
        Self {
            title_text: GString::new(),
            title_text_size: 16,
            title_margin: 4,
            border_margin: 4,
            show_animation: ShowAnimation::ScaleInCenter,
            hide_animation: HideAnimation::ScaleOutCenter,
            can_fullscreen: false,
            animation_in_speed: 0.25,
            animation_out_speed: 0.25,
            close_on_escape: true,
            keep_centered: true,
            screen_fx: ScreenFX::Blur,
            scene_origin: None,
            
            start_origin: Vector2 { x: 0.0, y: 0.0 },
            fullscreen_on: false,
            original_size: Vector2::ZERO,
            original_position: Vector2::ZERO,

            vbox: VBoxContainer::new_alloc(),
            fullscreen_button: Button::new_alloc(),
            base
        }
    }

    fn ready(&mut self) {
        let tree = Singleton::get_tree().try_cast::<SceneTree>().unwrap();

        self.scene_origin = tree.get_current_scene();

        self.signals().hide_request().connect_self(Self::on_hide_request);
        self.base_mut().set_focus_mode(FocusMode::ALL);
        self.base_mut().set_h_size_flags(SizeFlags::SHRINK_BEGIN);
        self.base_mut().set_v_size_flags(SizeFlags::SHRINK_BEGIN);
        
        let mut vb: Gd<VBoxContainer> = VBoxContainer::new_alloc();
        vb.set_anchors_preset(LayoutPreset::FULL_RECT);

        let mut l: Gd<Label> = Label::new_alloc();
        l.set_text(&self.title_text);
        l.set_horizontal_alignment(HorizontalAlignment::CENTER);
        l.set_vertical_alignment(VerticalAlignment::CENTER);
        l.add_theme_font_size_override("font_size", self.title_text_size);
        l.set_custom_minimum_size(Vector2 {x: 0.0, y: (self.title_text_size + self.title_margin * 2) as f32});
        vb.add_child(&l);

        if self.can_fullscreen {
            let mut cb: Gd<Button> = Button::new_alloc();
            cb.set_anchors_preset(LayoutPreset::CENTER_RIGHT);
            cb.set_offset(Side::LEFT, -43.0);
            cb.set_offset(Side::RIGHT, -19.0);
            cb.set_offset(Side::TOP, -12.0);
            cb.set_custom_minimum_size(Vector2 { x: 0.0, y: 22.0 });
            cb.set_theme_type_variation("nWindowMaximizeButton");
            cb.set_toggle_mode(true);
            
            let self_ref = self.to_gd();
            cb.signals().toggled().connect_other(&self_ref, NebulaWindow::on_fullscreen_toggled);

            self.fullscreen_button = cb.to_godot();
            
            l.add_child(&cb);
        }

        let mut c: Gd<MarginContainer> = MarginContainer::new_alloc();
        c.set_h_size_flags(SizeFlags::EXPAND_FILL);
        c.set_v_size_flags(SizeFlags::EXPAND_FILL);
        c.add_theme_constant_override("margin_left", self.border_margin);
        c.add_theme_constant_override("margin_right", self.border_margin);
        c.add_theme_constant_override("margin_bottom", self.border_margin);
        vb.add_child(&c);
        
        let self_ref = self.to_gd();

        for i in 0..self.base().get_children().len() {
            if let Some(mut node) = self.base().get_child(i as i32) {
                node.reparent(&c);
                if let Ok(mut n) = node.try_cast::<Control>() && !n.is_connected("item_rect_changed", &Callable::from_object_method(&self_ref, "on_item_rect_changed")) {
                        n.set_h_size_flags(SizeFlags::FILL);
                        n.set_v_size_flags(SizeFlags::FILL);
                        n.signals().item_rect_changed().connect_other(&self_ref, NebulaWindow::on_item_rect_changed);
                };
            };
        };
    
        self.vbox = vb.to_godot();
        vb.set_h_size_flags(SizeFlags::SHRINK_BEGIN);
        vb.set_v_size_flags(SizeFlags::SHRINK_BEGIN);
        self.base_mut().add_child(&vb);

        let mut self_ref = self.to_gd();
        
        if self.get_screen_fx() != (ScreenFX::None as i32) {
            let singleton: Gd<Singleton> = Singleton::singleton();
            let ui_layer = singleton.bind().get_ui_canvas_layer();
            self_ref.call_deferred("reparent", &[ui_layer.to_variant()]);

            let win_holder: Gd<Node> = Node::new_alloc();
            self_ref.get_parent().expect("No parent").call_deferred("add_child", &[win_holder.to_variant()]);
            win_holder.signals().tree_exiting().connect(move || {
                self_ref.queue_free();
            });
        }
    }
        
    

    fn input(&mut self, event: Gd<InputEvent>) {
        if !self.close_on_escape || !self.base().is_visible_in_tree() {
            return;
        }
        
        if let Ok(e) = event.try_cast::<InputEventKey>() && e.get_keycode() == Key::ESCAPE {
                self.signals().hide_request().emit();
        }
    }

    fn process(&mut self, _delta: f64) {
        if !self.fullscreen_on && self.can_fullscreen {
            self.original_size = self.base().get_size();
            self.original_position = self.base().get_global_position();
        }

        if self.keep_centered {
            NebulaWindow::reposition_to_center(&mut self.base_mut());
        }

        if self.fullscreen_on {
            self.base_mut().set_size(Singleton::get_scaled_window_size().cast_float());
        }
    }
}


#[godot_api]
impl NebulaWindow {
    /// Emitted when the window is requested to hide.
    #[signal]
    fn hide_request();

    /// Shows the window with the configured animation.
    #[func]
    fn show(&mut self) {
        self.base_mut().set_as_top_level(true);
        self.base_mut().set_size(Vector2::splat(0.0));
        self.animate_in(self.show_animation);
    }

    /// Hides the window, emitting the [signal hide_request] signal.
    #[func]
    fn hide(&mut self) {
        self.signals().hide_request().emit();
    }

    fn on_item_rect_changed(&mut self) {
        if !self.fullscreen_on {
            self.base_mut().set_size(Vector2::splat(0.0));
        }
    }

    fn animate_in(&mut self, animation: ShowAnimation) {
        if let Ok(mut res) = Singleton::get_tree().try_cast::<SceneTree>() {
            
            match ScreenFX::from(self.get_screen_fx()) {
                ScreenFX::Blur => Singleton::singleton().bind_mut().show_screen_blur(),
                _ => {}
            }

            match animation {
                ShowAnimation::None => {
                    let mut c = self.base_mut();
                    NebulaWindow::reposition_to_center(&mut c);

                    let base: &mut PanelContainer = c.upcast_mut();
                    base.show();
                }
                ShowAnimation::ScaleInCenter => {
                    let mut base_gd = self.base().to_godot();
                    NebulaWindow::reposition_to_center(&mut base_gd);
                    
                    let mut tween: Gd<Tween> = res.create_tween().unwrap();

                    let size = base_gd.get_size();
                    base_gd.set_pivot_offset(Vector2::from_tuple((size.x / 2.0, size.y / 2.0)));
                    base_gd.set_scale(Vector2::splat(0.25));
                    tween.set_ease(tween::EaseType::OUT);
                    tween.set_trans(tween::TransitionType::QUINT);
                    tween.tween_property(&base_gd, "scale", &Vector2::from_tuple((1.0, 1.0)).to_variant(), self.animation_in_speed);
                    base_gd.show();
                }
            }

            self.base_mut().grab_focus();
        };
    }

    fn animate_out(&mut self, animation: HideAnimation) {
        if let Ok(mut res) = Singleton::get_tree().try_cast::<SceneTree>() {

            match ScreenFX::from(self.get_screen_fx()) {
                ScreenFX::Blur => Singleton::singleton().bind_mut().hide_screen_blur(),
                _ => {}
            }

            match animation {
                HideAnimation::None => {
                    let mut c = self.base_mut();
                    let base: &mut PanelContainer = c.upcast_mut();
                    base.hide();
                }
                HideAnimation::ScaleOutCenter => {
                    let mut base_gd = self.base().to_godot();
                    let mut tween: Gd<Tween> = res.create_tween().unwrap();
                    tween.set_ease(tween::EaseType::OUT);
                    tween.set_trans(tween::TransitionType::QUINT);
                    tween.tween_property(&base_gd, "scale", &Vector2::ZERO.to_variant(), self.animation_out_speed);
                    tween.signals().finished().connect(move || {
                        if base_gd.is_instance_valid() {
                            base_gd.hide();
                            base_gd.set_scale(Vector2::ONE);
                        }
                    });
                }
            }
        };
    }

    fn reposition_to_center(base_gd: &mut Gd<PanelContainer>) {
        let size = base_gd.get_size() * base_gd.get_scale();
        base_gd.set_global_position(Singleton::centerize(Singleton::get_scaled_window_size(), Vector2::cast_int(size)).cast_float());
    }

    pub fn on_hide_request(&mut self) {
        self.animate_out(self.hide_animation);
        self.fullscreen_on = false;
        self.fullscreen_button.set_pressed_no_signal(false);
    }

    fn on_fullscreen_toggled(&mut self, toggled_on: bool) {
        if let Ok(mut res) = Singleton::get_tree().try_cast::<SceneTree>() {
            let base_gd = self.base().to_godot();
            let mut tween: Gd<Tween> = res.create_tween().unwrap();
            
            tween.set_ease(tween::EaseType::OUT);
            tween.set_trans(tween::TransitionType::QUINT);
            tween.set_parallel();

            if toggled_on {
                self.original_size = base_gd.get_size();
                self.original_position = base_gd.get_global_position();
                self.fullscreen_on = true;

                self.vbox.set_h_size_flags(SizeFlags::FILL);
                self.vbox.set_v_size_flags(SizeFlags::FILL);

                let screen_size = Singleton::get_scaled_window_size();
                tween.tween_property(&base_gd, "global_position", &Vector2::ZERO.to_variant(), 0.3);
                tween.tween_property(&base_gd, "size", &screen_size.cast_float().to_variant(), 0.3);
                
            } else {
                self.fullscreen_on = false;
                
                let target_position = if self.keep_centered {
                    let size = self.original_size;
                    Singleton::centerize(Singleton::get_scaled_window_size(), Vector2::cast_int(size)).cast_float()
                } else {
                    self.original_position
                };
                
                tween.tween_property(&base_gd, "global_position", &target_position.to_variant(), 0.3);
                tween.tween_property(&base_gd, "size", &self.original_size.to_variant(), 0.3);
            }
        }
    }
}
