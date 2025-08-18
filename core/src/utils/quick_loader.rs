use godot::prelude::*;
use godot::classes::{Object, IObject, Image, ImageTexture, FileAccess, file_access::ModeFlags};


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
    #[func]
    fn load_image(path: GString) -> Gd<ImageTexture> {
        let mut img = Image::new_gd();
        let extension: GString = path.get_extension();

        if !FileAccess::file_exists(&path) {
            godot_warn!("Image texture file does not exist!");
            return ImageTexture::new_gd();
        }

        let img_buffer = &FileAccess::get_file_as_bytes(&path);

        if extension.eq(&"svg".to_godot()) {
            img.load_svg_from_buffer(img_buffer);
        } else if extension.eq(&"png".to_godot()) {
            img.load_png_from_buffer(img_buffer);
        } else if extension.eq(&"jpg".to_godot()) || extension.eq(&"jpeg".to_godot()) {
            img.load_jpg_from_buffer(img_buffer);
        }

        if let Some(tex) = ImageTexture::create_from_image(&img) {
            return tex;
        }
    
        ImageTexture::new_gd()
    }
}
