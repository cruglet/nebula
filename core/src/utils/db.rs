use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct DocParser {
    base: Base<Object>,
}


#[godot_api]
impl IObject for DocParser {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
        }
    }
}
