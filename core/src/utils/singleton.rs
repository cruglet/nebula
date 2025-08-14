use godot::{classes::Engine, prelude::*};
use crate::utils::{core_settings::CoreSettings, module::Module};

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Singleton {
    #[export]
    loaded_modules: Dictionary,

    base: Base<Node>
}

#[godot_api]
impl INode for Singleton {
    fn init(base: Base<Node>) -> Self {
        Self {
            loaded_modules: Dictionary::new(),
            base,
        }
    }
}

#[godot_api]
impl Singleton {
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
}
