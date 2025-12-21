use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct NebulaFile {
    #[base] base: Base<RefCounted>,
}


#[godot_api]
impl IRefCounted for NebulaFile {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base
        }
    }
}


#[godot_api]
impl NebulaFile {
    #[func] fn get_buffer(&self) -> Array<i32> {
        

        Array::new()
    }
}