
use godot::prelude::*;
use godot::classes::{Resource};

#[derive(GodotClass)]
#[class(base=Resource, tool)]
#[allow(non_camel_case_types)]
pub struct nTheme {
    base: Base<Resource>
}


#[godot_api]
impl IResource for nTheme {
    fn init(base: Base<Resource>) -> Self {
        Self {
            base
        }
    }
}
