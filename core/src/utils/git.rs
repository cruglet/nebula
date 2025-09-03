use godot::prelude::*;
use godot::classes::RegEx;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct Git {
    base: Base<Object>
}


#[godot_api]
impl IObject for Git {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
        }
    }
}


#[godot_api]
impl Git {
    #[func]
    pub fn convert_github_to_raw_url(url: GString) -> GString {
        let github_to_raw_regex: Gd<RegEx> = RegEx::create_from_string(r"^https://github\.com/([^/]+)/([^/]+)/blob/([^/]+(?:/[^/]+)*)/(.+)$").unwrap(); 
        github_to_raw_regex.sub(&url, "https://raw.githubusercontent.com/$1/$2/refs/heads/$3/$4")
    }
}
