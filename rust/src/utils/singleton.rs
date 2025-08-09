use godot::{classes::Engine, prelude::*};

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Singleton {
    base: Base<Node>
}

#[godot_api]
impl INode for Singleton {
    fn init(base: Base<Node>) -> Self {
        Self {
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
        tree.try_cast::<SceneTree>().unwrap()
    }
}
