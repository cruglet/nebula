use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct NebulaDB {
    base: Base<Object>
}


#[godot_api]
impl IObject for NebulaDB {
    fn init(base: Base<Object>) -> Self {
        Self {
            base: base
        }
    }
}