use godot::prelude::*;
use godot::classes::file_access::ModeFlags;
use godot::classes::{DirAccess, FileAccess, PckPacker, ProjectSettings, Resource, ResourceUid};
use godot::global::{bytes_to_var_with_objects, var_to_bytes_with_objects};

#[derive(GodotClass)]
#[class(base=Resource, tool)]
pub struct Module {
    #[export] name: GString,
    #[export(multiline)] description: GString,
    #[export] authors: Array<GString>,
    #[export] group: GString,
    #[export] id: GString,
    #[export] major_version: u8,
    #[export] minor_version: u8,
    #[export] patch_number: u8,
    #[export(file="*.tscn,*.scn*,*.res")] entry_scene: GString,
    #[export(dir)] module_folder: GString,
    #[export(file="*.png,*.jpg,*.jpeg,*.svg")] module_image: GString,
    #[export(file="*.png,*.jpg,*.jpeg,*.svg")] project_image: GString,
    #[export] compatible_files: Array<GString>,

    export_folder: GString,
    
    #[var(
        usage_flags = [EDITOR],
        hint = TOOL_BUTTON,
        hint_string = "Export Module"
    )]
    _generate_mod_fn: Callable,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for Module {
    fn init(base: Base<Resource>) -> Self {
        Self {
            name: GString::new(),
            description: GString::new(),
            group: GString::new(),
            authors: Array::new(),
            id: GString::new(),
            major_version: 0,
            minor_version: 0,
            patch_number: 0,
            entry_scene: GString::new(),
            module_folder: GString::new(),
            module_image: GString::new(),
            project_image: GString::new(),
            compatible_files: Array::new(),
            export_folder: GString::from("res://build/modules"),
            _generate_mod_fn: Callable::invalid(),
            base,
        }
    }

    fn set_property(&mut self, property: StringName, value: Variant) -> bool {
        if !self._generate_mod_fn.is_valid() {
            self._generate_mod_fn = self.base().callable("_generate_module");
        }

        if property == StringName::from("id") {
            self.id = value.to_string().to_lowercase().replace(" ", "_").into();
            return true;
        }

        if property == StringName::from("group") {
            self.group = value.to_string().to_lowercase().replace(" ", "_").into();
            return true;
        }

        if property == StringName::from("entry_scene") && FileAccess::file_exists(&value.to_string()) {
            self.entry_scene = ResourceUid::singleton().call("ensure_path", &[value]).to_string().into();
            return true;
        }

        if property == StringName::from("project_image") && FileAccess::file_exists(&value.to_string()) {
            self.project_image = ResourceUid::singleton().call("ensure_path", &[value]).to_string().into();
            return true;
        }

        if property == StringName::from("module_image") && FileAccess::file_exists(&value.to_string()) {
            self.module_image = ResourceUid::singleton().call("ensure_path", &[value]).to_string().into();
            return true;
        }
        
        false
    }
}

#[godot_api]
impl Module {
    #[func]
    fn new() -> Gd<Module> {
        Gd::from_init_fn(|base| {
            Module {
                name: GString::new(),
                description: GString::new(),
                group: GString::new(),
                id: GString::new(),
                authors: Array::new(),
                major_version: 0,
                minor_version: 0,
                patch_number: 0,
                entry_scene: GString::new(),
                module_image: GString::new(),
                module_folder: GString::new(),
                project_image: GString::new(),
                compatible_files: Array::new(),
                export_folder: GString::new(),
                _generate_mod_fn: Callable::invalid(),
                base,
            }
        })
    }


    #[func]
    fn load(path: GString) -> Gd<Module> {
        let mut m: Gd<Module> = Module::new();
        
        let file: Option<Gd<FileAccess>> = FileAccess::open(&path.to_string(), ModeFlags::READ);
        let meta_size: u32;
        let meta_bytes: PackedByteArray;
        let pck_meta: Dictionary;

        if let Some(mut f) = file {
            meta_size = f.get_32();
            meta_bytes = f.get_buffer(meta_size.into());
            pck_meta = bytes_to_var_with_objects(&meta_bytes).try_to().unwrap();
            f.close();
        } else {
            godot_error!("Something went wrong opening the module file!");
            return m;
        }

        if !pck_meta.contains_key("entry_scene") {
            godot_error!("Missing 'entry_scene' in metadata.");
            return m;
        }

        let offset: u64 = (4 + meta_size).into();
        let success: bool = ProjectSettings::singleton().load_resource_pack_ex(&path.to_string()).replace_files(true).offset(offset.try_into().unwrap()).done();

        if success {
            let mut module_data = m.bind_mut();
            let mut mod_base = module_data.base_mut();

            for item in pck_meta.keys_shared() {
                mod_base.set(&item.to_string(), &pck_meta.get(item).unwrap_or(Variant::nil()));
            }
        }

        m
    }

    #[func]
    fn get_version_string(&self) -> GString {
        format!("{}.{}.{}", self.major_version, self.minor_version, self.patch_number).to_godot()
    }

    #[func]
    fn _generate_module(&self) {
        if self.name.is_empty() {
            godot_error!("[Module Packer] No ID provided!");
            return;
        }
        if self.description.is_empty() {
            godot_error!("[Module Packer] No Description provided!");
            return;
        }
        if self.group.is_empty() {
            godot_error!("[Module Packer] No Group provided!");
            return;
        }
        if self.id.is_empty() {
            godot_error!("[Module Packer] No ID provided!");
            return;
        }
        if self.authors.is_empty() {
            godot_error!("[Module Packer] No Authors provided!");
            return;
        }
        if self.entry_scene.is_empty() {
            godot_error!("[Module Packer] No Entry Scene provided!");
            return;
        }
        if self.module_folder.is_empty() {
            godot_error!("[Module Packer] No Module Folder provided!");
            return;
        }
        if self.export_folder.is_empty() {
            godot_error!("[Module Packer] No Export Folder provided!");
            return;
        }

        godot_print_rich!("[color=white][Module Packer] Packing files...");

        let all_files = self.get_all_files_in_module_folder();
        if all_files.is_empty() {
            godot_error!("[Module Packer] No files found in module folder");
            return;
        }

        let base = if self.module_folder.ends_with("/") {
            self.module_folder.to_string()
        } else {
            format!("{}/", self.module_folder)
        };

        let export_folder = if self.export_folder.ends_with("/") {
            self.export_folder.to_string()
        } else {
            format!("{}/", self.export_folder)
        };

        if DirAccess::make_dir_recursive_absolute(&export_folder) != godot::global::Error::OK {
            godot_error!("[Module Packer] Failed to create export folder: {}", export_folder);
            return;
        }

        let raw_mod_path = format!("{}{}.raw.mod", export_folder, self.id);
        let final_nmod_path = format!("{}{}.nmod", export_folder, self.id);

        let mut packer = PckPacker::new_gd();
        if packer.pck_start(&raw_mod_path) != godot::global::Error::OK {
            godot_error!("[Module Packer] Failed to start .mod file at: {}", raw_mod_path);
            return;
        }

        for file_path in all_files.iter() {
            let full_path = format!("{}{}", base, file_path);
            
            godot_print!("[Module Packer] Packing {}...", full_path);

            if packer.add_file(&full_path, &full_path) != godot::global::Error::OK {
                godot_error!("[Module Packer] Failed to add file to pack: {}", file_path.to_string());
                return;
            }
        }

        if packer.flush_ex().verbose(true).done() != godot::global::Error::OK {
            godot_error!("[Module Packer] Failed to generate .mod");
            return;
        }

        let mut metadata = Dictionary::new();
        metadata.set("name", Variant::from(self.name.to_string()));
        metadata.set("description", Variant::from(self.description.to_string()));
        metadata.set("authors", Variant::from(self.authors.clone()));
        metadata.set("group", Variant::from(self.group.to_string()));
        metadata.set("id", Variant::from(self.id.to_string()));
        metadata.set("major_version", Variant::from(self.major_version));
        metadata.set("minor_version", Variant::from(self.minor_version));
        metadata.set("patch_number", Variant::from(self.patch_number));
        metadata.set("entry_scene", Variant::from(self.entry_scene.to_string()));
        metadata.set("module_folder", Variant::from(self.module_folder.to_string()));
        metadata.set("module_image", Variant::from(self.module_image.to_string()));
        metadata.set("project_image", Variant::from(self.project_image.to_string()));
        metadata.set("compatible_files", Variant::from(self.compatible_files.clone()));
        metadata.set("file_count", Variant::from(all_files.len() as i64));

        let meta_bytes =  var_to_bytes_with_objects(&Variant::from(metadata));
        let meta_size = meta_bytes.len();
        if meta_size > u32::MAX as usize {
            godot_error!("[Module Packer] Metadata too large");
            return;
        }

        let mut raw_file = match FileAccess::open(&raw_mod_path, ModeFlags::READ) {
            Some(f) => f,
            None => {
                godot_error!("[Module Packer] Failed to open raw .mod file");
                return;
            }
        };
        let raw_length = raw_file.get_length();
        let raw_mod_data = raw_file.get_buffer(raw_length.try_into().unwrap());
        raw_file.close();

        let mut final_file = match FileAccess::open(&final_nmod_path, ModeFlags::WRITE) {
            Some(f) => f,
            None => {
                godot_error!("[Module Packer] Failed to create final .nmod file");
                return;
            }
        };

        final_file.store_32(meta_size as u32);
        final_file.store_buffer(&meta_bytes);
        final_file.store_buffer(&raw_mod_data);
        final_file.close();

        let _ = DirAccess::remove_absolute(&raw_mod_path);

        godot_print_rich!("[color=green][Module Packer] Module (.nmod) created at: {}", final_nmod_path);
    }

    fn get_all_files_in_module_folder(&self) -> Vec<GString> {
        let mut files = Vec::new();
        let base = if self.module_folder.ends_with("/") {
            self.module_folder.to_string()
        } else {
            format!("{}/", self.module_folder)
        };
        self.add_files_recursive(&base, &base, &mut files);
        files
    }

    fn add_files_recursive(&self, root: &str, current_path: &str, files: &mut Vec<GString>) {
        let mut dir = match DirAccess::open(current_path) {
            Some(d) => d,
            None => {
                godot_error!("[Module Packer] Failed to open directory: {}", current_path);
                return;
            }
        };

        dir.list_dir_begin();

        loop {
            let file_name = dir.get_next();
            if file_name.is_empty() {
                break;
            }
            if file_name.begins_with(".") {
                continue;
            }

            let full_path = format!("{}/{}", current_path, file_name);
            if dir.current_is_dir() {
                self.add_files_recursive(root, &full_path, files);
            } else {
                let rel_path = full_path.trim_start_matches(root).trim_start_matches('/').to_string();
                files.push(rel_path.into());
            }
        }

        dir.list_dir_end();
    }
}
