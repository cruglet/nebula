use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct NebulaDB {
    base: Base<Object>,
    classes: Array<Variant>
}


#[godot_api]
impl IObject for NebulaDB {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            classes: Array::default()
        }
    }
}
