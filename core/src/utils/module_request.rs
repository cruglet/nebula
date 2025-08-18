use godot::prelude::*;
use godot::classes::{HttpRequest, RegEx};
use godot::classes::{http_client::Method, object::ConnectFlags};
use godot::global::bytes_to_var_with_objects;

use crate::utils::git::Git;
use crate::utils::singleton::Singleton;

#[derive(GodotClass)]
#[class(base=Object)]
struct ModuleRequest {
    base: Base<Object>
}

#[godot_api]
impl IObject for ModuleRequest {
    fn init(base: Base<Object>) -> Self {
        Self {base}
    }
}

#[godot_api]
impl ModuleRequest {
    #[signal] fn metadata_fetched(metadata: Dictionary, source_url: GString, module_size: i64);
    #[signal] fn preview_image_fetched(image_data: PackedByteArray, module_id: GString, image_type: GString);
    #[signal] fn could_not_connect();

    #[func]
    fn fetch_parallel(&self, repositories: Array<GString>) -> Gd<ModuleRequest> {
        for i in 0..repositories.len() {
            if let Some(full_repo_url) = repositories.get(i) {
                let repo_url = full_repo_url.get_basename(); // this removes the .git at the end if the clone link is used
                let modules_file_url = repo_url.path_join("blob/main/MODULES");
                let raw_module_url = Git::convert_github_to_raw_url(modules_file_url);
                let mut module_request: Gd<HttpRequest> = HttpRequest::new_alloc();
                Singleton::singleton().add_child(&module_request);

                module_request.set_meta("request_instance", &self.base().to_variant());

                module_request.signals().request_completed().builder().flags(ConnectFlags::ONE_SHOT).connect_other_gd(&module_request, ModuleRequest::on_module_list_request_completed);
                module_request.request(&raw_module_url);
            } else {
                godot_warn!("Invalid repository string at index {}, skipping...", i);
                continue;
            }
        }

        self.to_gd()
    }

    fn on_module_list_request_completed(&mut http_ref: Gd<HttpRequest>, result: i64, response_code: i64, _headers: PackedStringArray, body: PackedByteArray) {
       let request_instance: Gd<ModuleRequest> = http_ref.get_meta("request_instance").try_to().expect("This should not hit");

       if result != 0 || response_code != 200 {
           godot_error!("Failed to fetch module list data! {} {}", result, response_code);
           request_instance.signals().could_not_connect().emit();
           return
       };

       let text: GString = body.get_string_from_utf8();
       ModuleRequest::parse_modules_file(text, request_instance);
       http_ref.queue_free();
    }


    fn parse_modules_file(text: GString, request_instance: Gd<ModuleRequest>) {
        let module_file_text: PackedStringArray = text.split("\n");
        let mut root: GString = GString::from("");
        for i in 0..module_file_text.len() {
            let line: GString = module_file_text.get(i).unwrap();
            if line.is_empty() || line.begins_with("#") {
                continue;
            }
            // root change line
            if line.begins_with("root=") {
                root = line.split("root=").get(1).unwrap();
                continue;
            }
            // module
            ModuleRequest::request_module(line, &root, request_instance.to_godot());
        }
    }

    fn request_module(module_url: GString, module_root: &GString, request_instance: Gd<ModuleRequest>) {
        let mut singleton: Gd<Singleton> = Singleton::singleton();
        let mut http: Gd<HttpRequest> = HttpRequest::new_alloc();
        singleton.add_child(&http);

        let raw_module_url = &Git::convert_github_to_raw_url(module_url); 
        http.set_meta("raw_module_url", &raw_module_url.to_variant());
        http.set_meta("module_root", &module_root.to_variant());
        http.set_meta("request_instance", &request_instance.to_variant());
        
        http.signals().request_completed().builder().flags(ConnectFlags::ONE_SHOT).connect_other_gd(&http, ModuleRequest::on_module_header_request_completed);
        http.request_ex(raw_module_url).custom_headers(&PackedStringArray::from(["Range: bytes=0-3".to_godot()])).method(Method::GET).done();
    }


    fn on_module_header_request_completed(&mut http_ref: Gd<HttpRequest>, result: i64, response_code: i64, _headers: PackedStringArray, body: PackedByteArray) {
       if result != 0 || response_code != 206 {
           godot_error!("Failed to fetch module header data! {} {}", result, response_code);
           return
       };
       let metadata_size = body.decode_u32(0).expect("wtf");
       let start = 4;
       let end = start + metadata_size - 1;
       let raw_module_url: GString = http_ref.get_meta("raw_module_url").to_string().to_godot();

       http_ref.signals().request_completed().builder().flags(ConnectFlags::ONE_SHOT).connect_other_gd(&http_ref, ModuleRequest::on_module_metadata_request_completed);
       http_ref.request_ex(&raw_module_url).custom_headers(&PackedStringArray::from([format!("Range: bytes={}-{}", start, end).to_godot()])).method(Method::GET).done();
    }


    fn on_module_metadata_request_completed(&mut http_ref: Gd<HttpRequest>, result: i64, response_code: i64, headers: PackedStringArray, body: PackedByteArray) {
        if result != 0 || response_code != 206 {
            godot_error!("Failed to fetch module metadata!");
            return
        };

        let request_instance: Gd<ModuleRequest> = http_ref.get_meta("request_instance").try_to().expect("This should not hit");
        let raw_url: GString = http_ref.get_meta("raw_module_url").to_string().to_godot();
        let root: GString = http_ref.get_meta("module_root").to_string().to_godot();
        let mut data: Dictionary = Dictionary::new();

        if let Ok(metadata) = bytes_to_var_with_objects(&body).try_to::<Dictionary>() {
            if metadata.contains_key("name") && metadata.contains_key("description") {
                let module_size: i64 = ModuleRequest::get_file_size_from_headers(headers).unwrap();
                request_instance.signals().metadata_fetched().emit(&metadata.to_godot(), &raw_url, module_size);
                data = metadata;
            } else {return};
        };

        let req_url = ModuleRequest::get_project_file_from_raw_url(raw_url, root, data.get("module_image").unwrap().to_string().to_godot());
        http_ref.set_meta("image_url", &data.get("module_image").unwrap().to_string().to_godot().to_variant());
        http_ref.set_meta("module_id", &data.get("id").unwrap().to_string().to_godot().to_variant());
        http_ref.signals().request_completed().builder().flags(ConnectFlags::ONE_SHOT).connect_other_gd(&http_ref, ModuleRequest::on_module_preview_request_completed);
        http_ref.request(&req_url);
    }
    

    fn on_module_preview_request_completed(&mut http_ref: Gd<HttpRequest>, result: i64, response_code: i64, _headers: PackedStringArray, body: PackedByteArray) {
       if result != 0 || response_code != 200 {
            godot_error!("Error fetching the preview image! {} {}", result, response_code);
            return
       }

       let request_instance: Gd<ModuleRequest> = http_ref.get_meta("request_instance").try_to().expect("This should not hit");
       let image_url: GString = http_ref.get_meta("image_url").to_string().to_godot();
       let module_id: GString = http_ref.get_meta("module_id").to_string().to_godot();
       request_instance.signals().preview_image_fetched().emit(&body, &module_id, &image_url.get_extension());
       http_ref.queue_free();
    }


    fn get_project_file_from_raw_url(raw_url: GString, root: GString, project_file_path: GString) -> GString {
        let re = RegEx::create_from_string(&format!(r"^(https://raw\.githubusercontent\.com/[^/]+/[^/]+/refs/heads/[^/]+/)(?:{}/)?(.*)$", root).to_godot()).unwrap();
        re.sub(&raw_url, &format!("$1{}", project_file_path.replace("res:/", &root.rstrip("/").lstrip("/"))).to_godot())
    }


    fn get_file_size_from_headers(headers: PackedStringArray) -> Option<i64> {
        let re = RegEx::create_from_string(r"Content-Range:\s*bytes\s*\d+-\d+/(\d+)").unwrap();
        
        for i in 0..headers.len() {
            let header: GString = headers.get(i).unwrap();
            if re.search(&header).is_some() {
                let size_str = re.sub(&header, "$1");
                if let Ok(size) = size_str.to_string().parse::<i64>() {
                    return Some(size);
                }
            }
        }
        None
    }
}
