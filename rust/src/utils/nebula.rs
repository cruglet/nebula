use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Object)]
struct Nebula {
    base: Base<Object>
}

#[godot_api]
impl IObject for Nebula {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
        }
    }
}

enum Comparison {
    Greater,
    Equal,
    Less
}

#[godot_api]
impl Nebula {
    const MAJOR_VERSION: u16 = 0;
    const MINOR_VERSION: u16 = 0;
    const PATCH_NUMBER: u16 = 0;

    #[func]
    fn get_version_string() -> GString {
        let v = format!("{}.{}.{}", Nebula::MAJOR_VERSION, Nebula::MINOR_VERSION, Nebula::PATCH_NUMBER);
        v.to_godot()
    }

    #[func]
    fn get_version_major() -> i32 {
        Nebula::MAJOR_VERSION as i32
    }

    #[func]
    fn get_version_minor() -> i32 {
        Nebula::MINOR_VERSION as i32
    }

    #[func]
    fn get_version_patch() -> i32 {
        Nebula::PATCH_NUMBER as i32
    }

    #[func]
    fn is_newer_than(version: GString) -> bool {
        Nebula::compare_versions(Nebula::get_version_string(), version) == 1
    }

    #[func]
    fn is_older_than(version: GString) -> bool {
        Nebula::compare_versions(Nebula::get_version_string(), version) == -1
    }

    #[func]
    fn compare_versions(version_a: GString, version_b: GString) -> i32 {
        let nums_a = version_a.split(".");
        let nums_b = version_b.split(".");

        let min = nums_a.len().min(nums_b.len());

        let compare = |i: usize| -> Option<Comparison> {
            if let (Some(v1), Some(v2)) = (nums_a.get(i), nums_b.get(i)) {
                let s1: &str = &v1.to_string();
                let s2: &str = &v2.to_string();
                if let (Ok(vr1), Ok(vr2)) = (s1.parse::<u16>(), s2.parse::<u16>()) {
                    if vr1 > vr2 {return Some(Comparison::Greater)}
                    if vr1 == vr2 {return Some(Comparison::Equal)}
                    if vr1 < vr2 {return Some(Comparison::Less)}
                }
            }
        None
        };

        for i in 0..min {
            if let Some(comp) = compare(i) {
                match comp {
                    Comparison::Greater => {return 1}
                    Comparison::Less => {return -1}
                    _ => {}
                }
            }
        }
        0
    }
}
