use std::fs;
use godot::prelude::*;
use godot::classes::{Object, IObject, Image, ImageTexture, FileAccess, ResourceUid, Texture2D};


#[derive(GodotClass)]
#[class(base=Object)]
struct QuickActions {
    base: Base<Object>,
}

#[godot_api]
impl IObject for QuickActions {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
        }
    }
}

#[godot_api]
impl QuickActions {
    #[func]
    fn load_image(path: GString) -> Gd<ImageTexture> {
        if let Some(img) = QuickActions::_load_image(path) {
            return img;
        }
        ImageTexture::new_gd()
    }

    #[func]
    fn load_image_with_fallback(path: GString, fallback_image: Gd<Texture2D>) -> Gd<Texture2D> {
        if let Some(img) = QuickActions::_load_image(path) {
            return img.upcast();
        }

        fallback_image
    }

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
    fn delete_folder_recursively(path: GString) {
        let path_str = path.get_base_dir().to_string();
        match fs::remove_dir_all(path_str) {
            Ok(_) => {}
            Err(err) => {
                godot_print!("Error removing directory contents!\nError: {}", err);
                godot_print!("{}", path);
            }
        }
    }

    #[func]
    fn get_visible_children(node: Gd<Node>) -> Array<Gd<Node>> {
        let children: Array<Gd<Node>> = node.get_children();

        children
            .iter_shared()
            .filter_map(|n| {
                if let Ok(v) = n.get("visible").try_to::<bool>() && v {
                    return Some(n.clone());
                }
                None
            })
            .collect()
    }
}
