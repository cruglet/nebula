use godot::prelude::*;
use godot::classes::RegEx;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(GodotClass)]
#[class(base=Object)]
pub struct Git {
    base: Base<Object>,
}

#[godot_api]
impl IObject for Git {
    fn init(base: Base<Object>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl Git {
    #[func]
    pub fn convert_github_to_raw_url(url: GString) -> GString {
        let github_to_raw_regex: Gd<RegEx> =
            RegEx::create_from_string(
                r"^https://github\.com/([^/]+)/([^/]+)/blob/([^/]+(?:/[^/]+)*)/(.+)$",
            )
            .unwrap();

        github_to_raw_regex.sub(
            &url,
            "https://raw.githubusercontent.com/$1/$2/refs/heads/$3/$4",
        )
    }

    /// Returns the full current Git commit hash (HEAD).
    #[func]
    pub fn get_current_hash() -> GString {
        let git_root = match std::env::current_dir()
            .ok()
            .and_then(Self::find_git_root)
        {
            Some(p) => p,
            None => return GString::new(),
        };

        let git_dir = git_root.join(".git");
        let head_path = git_dir.join("HEAD");

        let head_contents = match fs::read_to_string(&head_path) {
            Ok(s) => s,
            Err(_) => return GString::new(),
        };

        let head_contents = head_contents.trim();

        // Detached HEAD
        if !head_contents.starts_with("ref:") {
            return head_contents.into();
        }

        // ref: refs/heads/main
        let ref_path = match head_contents.strip_prefix("ref: ") {
            Some(p) => p,
            None => return GString::new(),
        };

        let ref_file = git_dir.join(ref_path);

        match fs::read_to_string(ref_file) {
            Ok(s) => s.trim().into(),
            Err(_) => GString::new(),
        }
    }

    #[func]
    pub fn get_current_short_hash() -> GString {
        let full = Self::get_current_hash();
    
        if full.is_empty() {
            return GString::new();
        }
    
        let short_len = 7;
    
        let s = full.to_string();
        let end = short_len.min(s.len());
    
        s[..end].into()
    }

    /// Returns the current branch name (empty if detached).
    #[func]
    pub fn get_current_branch() -> GString {
        let git_root = match std::env::current_dir()
            .ok()
            .and_then(Self::find_git_root)
        {
            Some(p) => p,
            None => return GString::new(),
        };

        let head_path = git_root.join(".git").join("HEAD");

        let head = match fs::read_to_string(head_path) {
            Ok(s) => s,
            Err(_) => return GString::new(),
        };

        let head = head.trim();

        match head.strip_prefix("ref: refs/heads/") {
            Some(branch) => branch.into(),
            None => GString::new(), // detached HEAD
        }
    }

    #[func]
    pub fn is_git_repo() -> bool {
        std::env::current_dir()
            .ok()
            .and_then(Self::find_git_root)
            .is_some()
    }

    #[func]
    pub fn get_repo_root() -> GString {
        match std::env::current_dir()
            .ok()
            .and_then(Self::find_git_root)
        {
            Some(p) => p.to_string_lossy().to_string().to_godot(),
            None => GString::new(),
        }
    }

    fn find_git_root(start_dir: PathBuf) -> Option<PathBuf> {
        let mut dir: &Path = &start_dir;

        loop {
            if dir.join(".git").exists() {
                return Some(dir.to_path_buf());
            }

            dir = dir.parent()?;
        }
    }
}

