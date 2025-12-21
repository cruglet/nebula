use godot::prelude::*;
use godot::classes::RegEx;

/// Utility class for Git-related operations.
///
/// Currently provides functionality for converting GitHub URLs to raw content URLs.
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
    /// Converts a standard GitHub repository URL pointing to a file into a raw URL.
    ///
    /// Example:
    /// ```
    /// var url: String = "https://github.com/user/repo/blob/main/path/to/file.txt";
    /// var raw_url = Git.convert_github_to_raw_url(url);
    /// # raw_url == "https://raw.githubusercontent.com/user/repo/refs/heads/main/path/to/file.txt"
    /// ```
    ///
    /// This is useful when you want to directly download the contents of a file
    /// from a GitHub repository without cloning it.
    ///
    /// Parameters:
    /// - `url`: The standard GitHub URL to a file (e.g., from a `blob` path).
    ///
    /// Returns:
    /// - The corresponding raw GitHub content URL.
    #[func]
    pub fn convert_github_to_raw_url(url: GString) -> GString {
        let github_to_raw_regex: Gd<RegEx> = RegEx::create_from_string(r"^https://github\.com/([^/]+)/([^/]+)/blob/([^/]+(?:/[^/]+)*)/(.+)$").unwrap(); 
        github_to_raw_regex.sub(&url, "https://raw.githubusercontent.com/$1/$2/refs/heads/$3/$4")
    }
}
