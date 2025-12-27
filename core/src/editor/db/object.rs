use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct NebulaObject {
    #[base] base: Base<Resource>,
}


#[godot_api]
impl IResource for NebulaObject {
    fn init(base: Base<Resource>) -> Self {
        Self {
            base
        }
    }
}


#[godot_api]
impl NebulaObject {
}