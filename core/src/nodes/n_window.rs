use godot::obj::WithUserSignals;
use godot::prelude::*;
use godot::global::{Key, HorizontalAlignment, VerticalAlignment};
use godot::classes::{Control, PanelContainer, IPanelContainer, MarginContainer, Tween, SceneTree, tween, InputEvent, InputEventKey, VBoxContainer, Label, control::{FocusMode, SizeFlags}};

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

#[derive(GodotClass)]
#[class(base=PanelContainer)]
struct NebulaWindow {
    #[export] title_text: GString,
    #[export] title_text_size: i32,
    #[export] title_margin: i32,
    #[export] border_margin: i32,
    #[export] show_animation: ShowAnimation,
    #[export] hide_animation: HideAnimation,
    #[export] animation_in_speed: f64,
    #[export] animation_out_speed: f64,
    #[export] start_origin: Vector2,
    #[export] close_on_escape: bool,
    #[export] keep_centered: bool,

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
            animation_in_speed: 0.25,
            animation_out_speed: 0.25,
            close_on_escape: true,
            keep_centered: true,
            
            start_origin: Vector2 { x: 0.0, y: 0.0 },
            base
        }
    }

    fn ready(&mut self) {
        self.signals().hide_request().connect_self(Self::on_hide_request);
        self.base_mut().set_focus_mode(FocusMode::ALL);
        
        let mut vb: Gd<VBoxContainer> = VBoxContainer::new_alloc(); 

        let mut l: Gd<Label> = Label::new_alloc();
        l.set_text(&self.title_text);
        l.set_horizontal_alignment(HorizontalAlignment::CENTER);
        l.set_vertical_alignment(VerticalAlignment::CENTER);
        l.add_theme_font_size_override("font_size", self.title_text_size);
        l.set_custom_minimum_size(Vector2 {x: 0.0, y: (self.title_text_size + self.title_margin * 2) as f32});
        vb.add_child(&l);

        let mut c: Gd<MarginContainer> = MarginContainer::new_alloc();
        c.set_h_size_flags(SizeFlags::EXPAND);
        c.set_v_size_flags(SizeFlags::EXPAND);
        c.add_theme_constant_override("margin_left", self.border_margin);
        c.add_theme_constant_override("margin_right", self.border_margin);
        c.add_theme_constant_override("margin_bottom", self.border_margin);
        vb.add_child(&c);
        
        let self_ref = self.to_gd();

        for i in 0..self.base().get_children().len() {
            if let Some(mut node) = self.base().get_child(i as i32) {
                node.reparent(&c);
                if let Ok(n) = node.try_cast::<Control>() {
                    if !n.is_connected("item_rect_changed", &Callable::from_object_method(&self_ref, "on_item_rect_changed")) {
                        n.signals().item_rect_changed().connect_other(&self_ref, NebulaWindow::on_item_rect_changed);
                    }
                };
            };
        };

        self.base_mut().add_child(&vb);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if !self.close_on_escape || !self.base().is_visible_in_tree() {
            return;
        }
        
        if let Ok(e) = event.try_cast::<InputEventKey>() {
            if e.get_keycode() == Key::ESCAPE {
                self.signals().hide_request().emit();
            };
        }
    }

    fn process(&mut self, _delta: f64) {
        if self.keep_centered {
            let mut base_mut = self.base_mut();
            NebulaWindow::reposition_to_center(&mut base_mut);
        }
    }
}


#[godot_api]
impl NebulaWindow {
    #[signal]
    fn hide_request();

    #[func]
    fn show(&mut self) {
        self.base_mut().set_as_top_level(true);
        self.base_mut().set_size(Vector2::splat(0.0));
        self.animate_in(self.show_animation);
    }

    #[func]
    fn hide(&mut self) {
        self.signals().hide_request().emit();
    }

    fn on_item_rect_changed(&mut self) {
        self.base_mut().set_size(Vector2::splat(0.0));
    }

    fn animate_in(&mut self, animation: ShowAnimation) {
        if let Ok(mut res) = Singleton::get_tree().try_cast::<SceneTree>() {
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
                    tween.tween_property(&base_gd, "scale", &Vector2::ZERO.to_variant(), self.animation_in_speed);
                    tween.signals().finished().connect(move || {
                        base_gd.hide();
                        base_gd.set_scale(Vector2::ONE);
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
    }
}
