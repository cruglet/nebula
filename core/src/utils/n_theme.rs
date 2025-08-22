use godot::prelude::*;
use godot::classes::Resource;

#[derive(GodotClass)]
#[class(base=Resource, rename=nThemeR)]
pub struct NebulaTheme {
    base: Base<Resource>
}

#[godot_api]
impl IResource for NebulaTheme {
    fn init(base: Base<Resource>) -> Self {
        Self {
            base
        }
    }
}
