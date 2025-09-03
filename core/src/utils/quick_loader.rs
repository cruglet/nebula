use godot::prelude::*;
use godot::classes::{Object, IObject, Image, ImageTexture, FileAccess, ResourceUid};


#[derive(GodotClass)]
#[class(base=Object)]
struct QuickLoader {
    base: Base<Object>,
}

#[godot_api]
impl IObject for QuickLoader {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
        }
    }
}

#[godot_api]
impl QuickLoader {
    fn _load_image(path: GString) -> Option<Gd<ImageTexture>> {
        let mut img = Image::new_gd();
        let img_path: GString = ResourceUid::singleton().call("ensure_path", &[path.to_variant()]).to_string().into();
        let extension: GString = img_path.get_extension();

        if !FileAccess::file_exists(&img_path) {
            godot_warn!("Image texture file does not exist!");
            return None;
        }

        let img_buffer = &FileAccess::get_file_as_bytes(&img_path);

        if extension.eq(&"svg".to_godot()) {
            img.load_svg_from_buffer(img_buffer);
        } else if extension.eq(&"png".to_godot()) {
            img.load_png_from_buffer(img_buffer);
        } else if extension.eq(&"jpg".to_godot()) || extension.eq(&"jpeg".to_godot()) {
            img.load_jpg_from_buffer(img_buffer);
        }

        ImageTexture::create_from_image(&img)
    }


    #[func]
    fn load_image(path: GString) -> Gd<ImageTexture> {
        if let Some(img) = QuickLoader::_load_image(path) {
            return img;
        }
        ImageTexture::new_gd()
    }

    #[func]
    fn load_image_with_fallback(path: GString, fallback_path: GString) -> Gd<ImageTexture> {
        if let Some(img) = QuickLoader::_load_image(path) {
            return img;
        }
        
        if let Some(fb) = QuickLoader::_load_image(fallback_path) {
            return fb;
        }

        ImageTexture::new_gd()
    }
}
