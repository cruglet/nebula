use godot::prelude::*;

/// Abstract global class that provides metadata about the current instance of Nebula.
///
/// This class does not perform any processing on its own. It provides information about Nebula,
/// such as the currently running version, and helper functions for version comparison.  
/// This is useful for module compatibility checks and update/version checking.
///
/// ### Versioning
/// Nebula uses semantic versioning:
/// - **Major version**: Significant changes, engine overhauls, rewrites, backward-incompatible changes, or large feature additions.
/// - **Minor version**: Smaller updates that generally maintain module compatibility.
/// - **Patch number**: Minor fixes or small adjustments that do not affect module compatibility.
#[derive(GodotClass)]
#[class(base=Object)]
struct Nebula {
    base: Base<Object>,
}

#[godot_api]
impl IObject for Nebula {
    fn init(base: Base<Object>) -> Self {
        Self { base }
    }
}

enum Comparison {
    Greater,
    Equal,
    Less,
}

#[godot_api]
impl Nebula {
    /// Current major version number of Nebula.
    const MAJOR_VERSION: u16 = 0;

    /// Current minor version number of Nebula.
    const MINOR_VERSION: u16 = 0;

    /// Current patch number of Nebula.
    const PATCH_NUMBER: u16 = 0;

    /// Returns the current version of Nebula as a string formatted as `"major.minor.patch"`.
    #[func]
    fn get_version_string() -> GString {
        let v = format!("{}.{}.{}", Nebula::MAJOR_VERSION, Nebula::MINOR_VERSION, Nebula::PATCH_NUMBER);
        v.to_godot()
    }

    /// Returns the major version of Nebula as an integer.
    #[func]
    fn get_version_major() -> i32 {
        Nebula::MAJOR_VERSION as i32
    }

    /// Returns the minor version of Nebula as an integer.
    #[func]
    fn get_version_minor() -> i32 {
        Nebula::MINOR_VERSION as i32
    }

    /// Returns the patch number of Nebula as an integer.
    #[func]
    fn get_version_patch() -> i32 {
        Nebula::PATCH_NUMBER as i32
    }

    /// Checks if the current version is newer than the provided version string.
    ///
    /// `version` should be formatted as `"major.minor.patch"`, but missing parts are allowed.
    /// This method is consistent with [method compare_versions].
    #[func]
    fn is_newer_than(version: GString) -> bool {
        Nebula::compare_versions(Nebula::get_version_string(), version) == 1
    }

    /// Checks if the current version is older than the provided version string.
    ///
    /// `version` should be formatted as `"major.minor.patch"`, but missing parts are allowed.
    /// This method is consistent with [method compare_versions].
    #[func]
    fn is_older_than(version: GString) -> bool {
        Nebula::compare_versions(Nebula::get_version_string(), version) == -1
    }

    /// Compares two version strings.
    ///
    /// Returns:
    /// - `1` if [param version_a] is newer than [param version_b]
    /// - `-1` if [param version_a] is older than [param version_b]
    /// - `0` if both versions are equal
    ///
    /// The comparison only checks the parts that are present in the version strings.
    /// If a version string omits the patch number or minor version, those parts are simply ignored.
    /// Examples:
    /// - `"1.2"` compared to `"1.2.1"` will return `0` (since the patch is ignored)
    /// - `"1"` compared to `"1.0.0"` will return `0` (since minor and patch are ignored)
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
                    if vr1 > vr2 {
                        return Some(Comparison::Greater);
                    }
                    if vr1 == vr2 {
                        return Some(Comparison::Equal);
                    }
                    if vr1 < vr2 {
                        return Some(Comparison::Less);
                    }
                }
            }
            None
        };

        for i in 0..min {
            if let Some(comp) = compare(i) {
                match comp {
                    Comparison::Greater => return 1,
                    Comparison::Less => return -1,
                    _ => {}
                }
            }
        }
        0
    }

    #[func]
    fn get_reserved_extensions() -> PackedStringArray {
        PackedStringArray::from(
            vec![
            "nproj".to_godot(),
            "nmod".to_godot(),
            ]
        )
    }
}
