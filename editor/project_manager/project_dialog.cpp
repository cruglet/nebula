/**************************************************************************/
/*  project_dialog.cpp                                                    */
/**************************************************************************/
/*                         This file is part of:                          */
/*                             Nebula Engine                              */
/*                    https://github.com/cruglet/nebula                   */
/**************************************************************************/
/* Copyright (c) 2024-present Nebula Engine contributors                  */
/* Copyright (c) 2014-present Godot Engine contributors (see AUTHORS.md). */
/*                                                                        */
/* Permission is hereby granted, free of charge, to any person obtaining  */
/* a copy of this software and associated documentation files (the        */
/* "Software"), to deal in the Software without restriction, including    */
/* without limitation the rights to use, copy, modify, merge, publish,    */
/* distribute, sublicense, and/or sell copies of the Software, and to     */
/* permit persons to whom the Software is furnished to do so, subject to  */
/* the following conditions:                                              */
/*                                                                        */
/* The above copyright notice and this permission notice shall be         */
/* included in all copies or substantial portions of the Software.        */
/*                                                                        */
/* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,        */
/* EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF     */
/* MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. */
/* IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY   */
/* CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,   */
/* TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE      */
/* SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.                 */
/**************************************************************************/

#include "project_dialog.h"

#include "core/config/project_settings.h"
#include "core/io/dir_access.h"
#include "core/io/zip_io.h"
#include "core/math/math_defs.h"
#include "core/util/wii/misc.h"
#include "core/util/wii/wbfs.h"
#include "core/version.h"
#include "editor/editor_settings.h"
#include "editor/editor_string_names.h"
#include "editor/editor_vcs_interface.h"
#include "editor/gui/editor_file_dialog.h"
#include "editor/project_manager/project_list.h"
#include "editor/project_manager/validate_game_file.h"
#include "editor/themes/editor_icons.h"
#include "editor/themes/editor_scale.h"
#include "scene/gui/check_box.h"
#include "scene/gui/check_button.h"
#include "scene/gui/control.h"
#include "scene/gui/line_edit.h"
#include "scene/gui/option_button.h"
#include "scene/gui/separator.h"
#include "scene/gui/texture_rect.h"

void ProjectDialog::_set_message(const String &p_msg, MessageType p_type, InputType p_input_type) {
	msg->set_text(p_msg);
	get_ok_button()->set_disabled(p_type == MESSAGE_ERROR);

	Ref<Texture2D> new_icon;
	switch (p_type) {
		case MESSAGE_ERROR: {
			msg->add_theme_color_override(SceneStringName(font_color), get_theme_color(SNAME("error_color"), EditorStringName(Editor)));
			new_icon = get_editor_theme_icon(SNAME("StatusError"));
		} break;
		case MESSAGE_WARNING: {
			msg->add_theme_color_override(SceneStringName(font_color), get_theme_color(SNAME("warning_color"), EditorStringName(Editor)));
			new_icon = get_editor_theme_icon(SNAME("StatusWarning"));
		} break;
		case MESSAGE_SUCCESS: {
			msg->add_theme_color_override(SceneStringName(font_color), get_theme_color(SNAME("success_color"), EditorStringName(Editor)));
			new_icon = get_editor_theme_icon(SNAME("StatusSuccess"));
		} break;
	}

	if (p_input_type == PROJECT_PATH) {
		project_status_rect->set_texture(new_icon);
	} else if (p_input_type == INSTALL_PATH) {
		install_status_rect->set_texture(new_icon);
	}
}

static bool is_zip_file(Ref<DirAccess> p_d, const String &p_path) {
	return p_path.get_extension() == "zip" && p_d->file_exists(p_path);
}

void ProjectDialog::_validate_path() {
	_set_message("", MESSAGE_SUCCESS, PROJECT_PATH);
	_set_message("", MESSAGE_SUCCESS, INSTALL_PATH);

	project_preview->set_project_title(project_name->get_text());
	project_preview->set_project_path(project_path->get_text());

	if (project_name->get_text().strip_edges().is_empty()) {
		_set_message(TTR("Please give your project a name."), MESSAGE_ERROR);
		return;
	}

	Ref<DirAccess> d = DirAccess::create(DirAccess::ACCESS_FILESYSTEM);
	String path = project_path->get_text().simplify_path();

	String target_path = path;
	InputType target_path_input_type = PROJECT_PATH;

	if (mode == MODE_IMPORT) {
		if (path.get_file().strip_edges() == "project.nebula") {
			path = path.get_base_dir();
			project_path->set_text(path);
		}

		if (is_zip_file(d, path)) {
			zip_path = path;
		} else if (is_zip_file(d, path.strip_edges())) {
			zip_path = path.strip_edges();
		} else {
			zip_path = "";
		}

		if (!zip_path.is_empty()) {
			target_path = install_path->get_text().simplify_path();
			target_path_input_type = INSTALL_PATH;

			create_dir->show();
			install_path_container->show();

			Ref<FileAccess> io_fa;
			zlib_filefunc_def io = zipio_create_io(&io_fa);

			unzFile pkg = unzOpen2(zip_path.utf8().get_data(), &io);
			if (!pkg) {
				_set_message(TTR("Invalid \".zip\" project file; it is not in ZIP format."), MESSAGE_ERROR);
				unzClose(pkg);
				return;
			}

			int ret = unzGoToFirstFile(pkg);
			while (ret == UNZ_OK) {
				unz_file_info info;
				char fname[16384];
				ret = unzGetCurrentFileInfo(pkg, &info, fname, 16384, nullptr, 0, nullptr, 0);
				ERR_FAIL_COND_MSG(ret != UNZ_OK, "Failed to get current file info.");

				String name = String::utf8(fname);
				if (name.get_file() == "project.nebula") {
					break;
				}

				ret = unzGoToNextFile(pkg);
			}

			if (ret == UNZ_END_OF_LIST_OF_FILE) {
				_set_message(TTR("Invalid \".zip\" project file; it doesn't contain a \"project.nebula\" file."), MESSAGE_ERROR);
				unzClose(pkg);
				return;
			}

			unzClose(pkg);
		} else if (d->dir_exists(path) && d->file_exists(path.path_join("project.nebula"))) {
			zip_path = "";

			create_dir->hide();
			install_path_container->hide();

			_set_message(TTR("Valid project found at path."), MESSAGE_SUCCESS);
		} else {
			create_dir->hide();
			install_path_container->hide();

			_set_message(TTR("Please choose a \"project.nebula\", a directory with one, or a \".zip\" file."), MESSAGE_ERROR);
			return;
		}
	}

	if (target_path.is_relative_path()) {
		_set_message(TTR("The path specified is invalid."), MESSAGE_ERROR, target_path_input_type);
		return;
	}

	if (target_path.get_file() != OS::get_singleton()->get_safe_dir_name(target_path.get_file())) {
		_set_message(TTR("The directory name specified contains invalid characters or trailing whitespace."), MESSAGE_ERROR, target_path_input_type);
		return;
	}

	String working_dir = d->get_current_dir();
	String executable_dir = OS::get_singleton()->get_executable_path().get_base_dir();
	if (target_path == working_dir || target_path == executable_dir) {
		_set_message(TTR("Creating a project at the engine's working directory or executable directory is not allowed, as it would prevent the project manager from starting."), MESSAGE_ERROR, target_path_input_type);
		return;
	}

	// TODO: The following 5 lines could be simplified if OS.get_user_home_dir() or SYSTEM_DIR_HOME is implemented. See: https://github.com/nebulaengine/nebula-proposals/issues/4851.
#ifdef WINDOWS_ENABLED
	String home_dir = OS::get_singleton()->get_environment("USERPROFILE");
#else
	String home_dir = OS::get_singleton()->get_environment("HOME");
#endif
	String documents_dir = OS::get_singleton()->get_system_dir(OS::SYSTEM_DIR_DOCUMENTS);
	if (target_path == home_dir || target_path == documents_dir) {
		_set_message(TTR("You cannot save a project at the selected path. Please create a subfolder or choose a new path."), MESSAGE_ERROR, target_path_input_type);
		return;
	}

	is_folder_empty = true;
	if (mode == MODE_NEW || mode == MODE_INSTALL || (mode == MODE_IMPORT && target_path_input_type == InputType::INSTALL_PATH)) {
		if (create_dir->is_pressed()) {
			if (!d->dir_exists(target_path.get_base_dir())) {
				_set_message(TTR("The parent directory of the path specified doesn't exist."), MESSAGE_ERROR, target_path_input_type);
				return;
			}

			if (d->dir_exists(target_path)) {
				// The path is not necessarily empty here, but we will update the message later if it isn't.
				_set_message(TTR("The project folder already exists and is empty."), MESSAGE_SUCCESS, target_path_input_type);
			} else {
				_set_message(TTR("The project folder will be automatically created."), MESSAGE_SUCCESS, target_path_input_type);
			}
		} else {
			if (!d->dir_exists(target_path)) {
				_set_message(TTR("The path specified doesn't exist."), MESSAGE_ERROR, target_path_input_type);
				return;
			}

			// The path is not necessarily empty here, but we will update the message later if it isn't.
			_set_message(TTR("The project folder exists and is empty."), MESSAGE_SUCCESS, target_path_input_type);
		}

		if (d->change_dir(target_path) == OK) {
			d->list_dir_begin();
			String n = d->get_next();
			while (!n.is_empty()) {
				if (n[0] != '.') {
					// Allow `.`, `..` (reserved current/parent folder names)
					// and hidden files/folders to be present.
					// For instance, this lets users initialize a Git repository
					// and still be able to create a project in the directory afterwards.
					is_folder_empty = false;
					break;
				}
				n = d->get_next();
			}
			d->list_dir_end();

			if (!is_folder_empty) {
				_set_message(TTR("The selected path is not empty."), MESSAGE_ERROR, target_path_input_type);
			}
		}
	}
}

String ProjectDialog::_get_target_path() {
	if (mode == MODE_NEW || mode == MODE_INSTALL) {
		return project_path->get_text();
	} else if (mode == MODE_IMPORT) {
		return install_path->get_text();
	} else {
		ERR_FAIL_V("");
	}
}
void ProjectDialog::_set_target_path(const String &p_text) {
	if (mode == MODE_NEW || mode == MODE_INSTALL) {
		project_path->set_text(p_text);
	} else if (mode == MODE_IMPORT) {
		install_path->set_text(p_text);
	} else {
		ERR_FAIL();
	}
}

void ProjectDialog::_update_target_auto_dir() {
	String new_auto_dir;
	if (mode == MODE_NEW || mode == MODE_INSTALL) {
		new_auto_dir = project_name->get_text();
	} else if (mode == MODE_IMPORT) {
		new_auto_dir = project_path->get_text().get_file().get_basename();
	}
	int naming_convention = (int)EDITOR_GET("project_manager/directory_naming_convention");
	switch (naming_convention) {
		case 0: // No convention
			break;
		case 1: // kebab-case
			new_auto_dir = new_auto_dir.to_lower().replace(" ", "-");
			break;
		case 2: // snake_case
			new_auto_dir = new_auto_dir.to_snake_case();
			break;
		case 3: // camelCase
			new_auto_dir = new_auto_dir.to_camel_case();
			break;
		case 4: // PascalCase
			new_auto_dir = new_auto_dir.to_pascal_case();
			break;
		case 5: // Title Case
			new_auto_dir = new_auto_dir.capitalize();
			break;
		default:
			ERR_FAIL_MSG("Invalid directory naming convention.");
			break;
	}
	new_auto_dir = OS::get_singleton()->get_safe_dir_name(new_auto_dir);

	if (create_dir->is_pressed()) {
		String target_path = _get_target_path();

		if (target_path.get_file() == auto_dir) {
			// Update target dir name to new project name / ZIP name.
			target_path = target_path.get_base_dir().path_join(new_auto_dir);
		}
		_set_target_path(target_path);
	}
	auto_dir = new_auto_dir;
}

void ProjectDialog::_create_dir_toggled(bool p_pressed) {
	String target_path = _get_target_path();

	if (create_dir->is_pressed()) {
		// (Re-)append target dir name.
		if (last_custom_target_dir.is_empty()) {
			target_path = target_path.path_join(auto_dir);
		} else {
			target_path = target_path.path_join(last_custom_target_dir);
		}
	} else {
		// Strip any trailing slash.
		target_path = target_path.rstrip("/\\");
		// Save and remove target dir name.
		if (target_path.get_file() == auto_dir) {
			last_custom_target_dir = "";
		} else {
			last_custom_target_dir = target_path.get_file();
		}
		target_path = target_path.get_base_dir();
	}

	_set_target_path(target_path);
	_validate_path();
}

void ProjectDialog::_project_name_changed() {
	if (mode == MODE_NEW || mode == MODE_INSTALL) {
		_update_target_auto_dir();
	}

	_validate_path();
}

void ProjectDialog::_project_path_changed() {
	if (mode == MODE_IMPORT) {
		_update_target_auto_dir();
	}

	_validate_path();
}

void ProjectDialog::_install_path_changed() {
	_validate_path();
}

void ProjectDialog::_browse_project_path() {
	String path = project_path->get_text();
	if (path.is_relative_path()) {
		path = EDITOR_GET("filesystem/directories/default_project_path");
	}
	if (mode == MODE_IMPORT && install_path->is_visible_in_tree()) {
		fdialog_project->set_current_path(path);
	} else if ((mode == MODE_NEW || mode == MODE_INSTALL) && create_dir->is_pressed()) {
		fdialog_project->set_current_dir(path.get_base_dir());
	} else {
		fdialog_project->set_current_dir(path);
	}

	if (mode == MODE_IMPORT) {
		fdialog_project->set_file_mode(EditorFileDialog::FILE_MODE_OPEN_ANY);
		fdialog_project->clear_filters();
		fdialog_project->add_filter("project.nebula", vformat("%s %s", VERSION_NAME, TTR("Project")));
		fdialog_project->add_filter("*.zip", TTR("ZIP File"));
	} else if (mode == MODE_OPEN) {
		fdialog_project->set_file_mode(EditorFileDialog::FILE_MODE_OPEN_FILE);
		fdialog_project->clear_filters();
		fdialog_project->add_filter("*.iso", TTR("ISO File"));
		fdialog_project->add_filter("*.wbfs", TTR("WBFS File"));
	} else {
		fdialog_project->set_file_mode(EditorFileDialog::FILE_MODE_OPEN_DIR);
	}

	hide();
	fdialog_project->popup_file_dialog();
}

void ProjectDialog::_browse_install_path() {
	ERR_FAIL_COND_MSG(mode != MODE_IMPORT, "Install path is only used for MODE_IMPORT.");

	String path = install_path->get_text();
	if (path.is_relative_path() || !DirAccess::dir_exists_absolute(path)) {
		path = EDITOR_GET("filesystem/directories/default_project_path");
	}
	if (create_dir->is_pressed()) {
		// Select parent directory of install path.
		fdialog_install->set_current_dir(path.get_base_dir());
	} else {
		// Select install path.
		fdialog_install->set_current_dir(path);
	}

	fdialog_install->set_file_mode(EditorFileDialog::FILE_MODE_OPEN_DIR);
	fdialog_install->popup_file_dialog();
}

void ProjectDialog::_project_path_selected(const String &p_path) {
	if (mode == MODE_OPEN) {
		ISOHeader game = _validate_game_file(p_path);
		if (game == -1) {
			emit_signal(SNAME("project_error"), "Unrecognized or unsupported game.\nPlease make sure you are selecting a valid WBFS or ISO file.", Size2());
			return;
		}
		mode = MODE_NEW;
	}

	if (mode == MODE_NEW || mode == MODE_INSTALL) {
		//Replace parent directory, but keep target dir name.
		project_path->set_text(p_path.path_join(project_path->get_text().get_file()));
	} else {
	 	project_path->set_text(p_path);
	}

	show_dialog(true);

	_project_path_changed();

	if (install_path->is_visible_in_tree()) {
		// ZIP is selected; focus install path.
		install_path->grab_focus();
	} else {
		get_ok_button()->grab_focus();
	}
}

void ProjectDialog::_install_path_selected(const String &p_path) {
	ERR_FAIL_COND_MSG(mode != MODE_IMPORT, "Install path is only used for MODE_IMPORT.");

	if (create_dir->is_pressed()) {
		// Replace parent directory, but keep target dir name.
		install_path->set_text(p_path.path_join(install_path->get_text().get_file()));
	} else {
		install_path->set_text(p_path);
	}

	_install_path_changed();

	get_ok_button()->grab_focus();
}

ISOHeader ProjectDialog::_validate_game_file(const String &p_path) {
	CharString path_utf8 = p_path.utf8();
	const char* path_cstr = path_utf8.get_data();
	if (p_path.contains(".wbfs")) {
		return (ISOHeader) WBFS::validate_wbfs(path_cstr);
	}
}

void ProjectDialog::_reset_name() {
	project_name->set_text(TTR("My Project"));
}

void ProjectDialog::_nonempty_confirmation_ok_pressed() {
	is_folder_empty = true;
	ok_pressed();
}

void ProjectDialog::ok_pressed() {
	String path = project_path->get_text();

	if (mode == MODE_NEW) {
		if (create_dir->is_pressed()) {
			Ref<DirAccess> d = DirAccess::create(DirAccess::ACCESS_FILESYSTEM);
			if (!d->dir_exists(path) && d->make_dir(path) != OK) {
				_set_message(TTR("Couldn't create project directory, check permissions."), MESSAGE_ERROR);
				return;
			}
		}

		PackedStringArray project_features = ProjectSettings::get_required_features();
		ProjectSettings::CustomMap initial_settings;

		// Be sure to change this code if/when renderers are changed.
		// Default values are "forward_plus" for the main setting, "mobile" for the mobile override,
		// and "gl_compatibility" for the web override.
		String renderer_type = "gl_compatibility";
		initial_settings["rendering/renderer/rendering_method"] = renderer_type;

		EditorSettings::get_singleton()->set("project_manager/default_renderer", renderer_type);
		EditorSettings::get_singleton()->save();

		if (renderer_type == "forward_plus") {
			project_features.push_back("Forward Plus");
		} else if (renderer_type == "mobile") {
			project_features.push_back("Mobile");
		} else if (renderer_type == "gl_compatibility") {
			project_features.push_back("GL Compatibility");
			initial_settings["rendering/renderer/rendering_method.mobile"] = "gl_compatibility";
		} else {
			WARN_PRINT("Unknown renderer type. Please report this as a bug on GitHub.");
		}

		project_features.sort();
		initial_settings["application/config/features"] = project_features;
		initial_settings["application/config/name"] = project_name->get_text().strip_edges();

		Error err = ProjectSettings::get_singleton()->save_custom(path.path_join("project.nebula"), initial_settings, Vector<String>(), false);
		if (err != OK) {
			_set_message(TTR("Couldn't create project.nebula in project path."), MESSAGE_ERROR);
			return;
		}

		EditorVCSInterface::create_vcs_metadata_files(EditorVCSInterface::VCSMetadata(vcs_metadata_selection->get_selected()), path);
	}

	// Two cases for importing a ZIP.
	switch (mode) {
		case MODE_IMPORT: {
			if (zip_path.is_empty()) {
				break;
			}

			path = install_path->get_text().simplify_path();
			[[fallthrough]];
		}
		case MODE_INSTALL: {
			ERR_FAIL_COND(zip_path.is_empty());

			Ref<FileAccess> io_fa;
			zlib_filefunc_def io = zipio_create_io(&io_fa);

			unzFile pkg = unzOpen2(zip_path.utf8().get_data(), &io);
			if (!pkg) {
				dialog_error->set_text(TTR("Error opening package file, not in ZIP format."));
				dialog_error->popup_centered();
				return;
			}

			// Find the first directory with a "project.nebula".
			String zip_root;
			int ret = unzGoToFirstFile(pkg);
			while (ret == UNZ_OK) {
				unz_file_info info;
				char fname[16384];
				unzGetCurrentFileInfo(pkg, &info, fname, 16384, nullptr, 0, nullptr, 0);
				ERR_FAIL_COND_MSG(ret != UNZ_OK, "Failed to get current file info.");

				String name = String::utf8(fname);
				if (name.get_file() == "project.nebula") {
					zip_root = name.get_base_dir();
					break;
				}

				ret = unzGoToNextFile(pkg);
			}

			if (ret == UNZ_END_OF_LIST_OF_FILE) {
				_set_message(TTR("Invalid \".zip\" project file; it doesn't contain a \"project.nebula\" file."), MESSAGE_ERROR);
				unzClose(pkg);
				return;
			}

			if (create_dir->is_pressed()) {
				Ref<DirAccess> d = DirAccess::create(DirAccess::ACCESS_FILESYSTEM);
				if (!d->dir_exists(path) && d->make_dir(path) != OK) {
					_set_message(TTR("Couldn't create project directory, check permissions."), MESSAGE_ERROR);
					return;
				}
			}

			ret = unzGoToFirstFile(pkg);

			Vector<String> failed_files;
			while (ret == UNZ_OK) {
				//get filename
				unz_file_info info;
				char fname[16384];
				ret = unzGetCurrentFileInfo(pkg, &info, fname, 16384, nullptr, 0, nullptr, 0);
				ERR_FAIL_COND_MSG(ret != UNZ_OK, "Failed to get current file info.");

				String rel_path = String::utf8(fname).trim_prefix(zip_root);
				if (rel_path.is_empty()) { // Root.
				} else if (rel_path.ends_with("/")) { // Directory.
					Ref<DirAccess> da = DirAccess::create(DirAccess::ACCESS_FILESYSTEM);
					da->make_dir(path.path_join(rel_path));
				} else { // File.
					Vector<uint8_t> uncomp_data;
					uncomp_data.resize(info.uncompressed_size);

					unzOpenCurrentFile(pkg);
					ret = unzReadCurrentFile(pkg, uncomp_data.ptrw(), uncomp_data.size());
					ERR_BREAK_MSG(ret < 0, vformat("An error occurred while attempting to read from file: %s. This file will not be used.", rel_path));
					unzCloseCurrentFile(pkg);

					Ref<FileAccess> f = FileAccess::open(path.path_join(rel_path), FileAccess::WRITE);
					if (f.is_valid()) {
						f->store_buffer(uncomp_data.ptr(), uncomp_data.size());
					} else {
						failed_files.push_back(rel_path);
					}
				}

				ret = unzGoToNextFile(pkg);
			}

			unzClose(pkg);

			if (failed_files.size()) {
				String err_msg = TTR("The following files failed extraction from package:") + "\n\n";
				for (int i = 0; i < failed_files.size(); i++) {
					if (i > 15) {
						err_msg += "\nAnd " + itos(failed_files.size() - i) + " more files.";
						break;
					}
					err_msg += failed_files[i] + "\n";
				}

				dialog_error->set_text(err_msg);
				dialog_error->popup_centered();
				return;
			}
		} break;
		default: {
		} break;
	}

	if (mode == MODE_RENAME || mode == MODE_INSTALL) {
		// Load project.nebula as ConfigFile to set the new name.
		ConfigFile cfg;
		String project_nebula = path.path_join("project.nebula");
		Error err = cfg.load(project_nebula);
		if (err != OK) {
			dialog_error->set_text(vformat(TTR("Couldn't load project at '%s' (error %d). It may be missing or corrupted."), project_nebula, err));
			dialog_error->popup_centered();
			return;
		}
		cfg.set_value("application", "config/name", project_name->get_text().strip_edges());
		err = cfg.save(project_nebula);
		if (err != OK) {
			dialog_error->set_text(vformat(TTR("Couldn't save project at '%s' (error %d)."), project_nebula, err));
			dialog_error->popup_centered();
			return;
		}
	}

	hide();
	if (mode == MODE_NEW || mode == MODE_IMPORT || mode == MODE_INSTALL) {
		emit_signal(SNAME("project_created"), path);
	} else if (mode == MODE_RENAME) {
		emit_signal(SNAME("projects_updated"));
	}
}

void ProjectDialog::set_zip_path(const String &p_path) {
	zip_path = p_path;
}

void ProjectDialog::set_zip_title(const String &p_title) {
	zip_title = p_title;
}

void ProjectDialog::set_mode(Mode p_mode) {
	mode = p_mode;
}

void ProjectDialog::set_project_name(const String &p_name) {
	project_name->set_text(p_name);
}

void ProjectDialog::set_project_path(const String &p_path) {
	project_path->set_text(p_path);
}

void ProjectDialog::ask_for_path_and_show() {
	_reset_name();
	_browse_project_path();
}

void ProjectDialog::show_dialog(bool p_reset_name) {
	if (mode == MODE_RENAME) {
		// Name and path are set in `ProjectManager::_rename_project`.
		project_path->set_editable(false);

		set_title(TTR("Rename Project"));
		set_ok_button_text(TTR("Rename"));

		create_dir->hide();
		project_status_rect->hide();
		project_browse->hide();

		name_container->show();
		install_path_container->hide();
		default_files_container->hide();

		callable_mp((Control *)project_name, &Control::grab_focus).call_deferred();
		callable_mp(project_name, &LineEdit::select_all).call_deferred();
	} else {
		if (p_reset_name) {
			_reset_name();
		}
		project_path->set_editable(true);

		String fav_dir = EDITOR_GET("filesystem/directories/default_project_path");
		if (!fav_dir.is_empty()) {
			project_path->set_text(fav_dir);
			install_path->set_text(fav_dir);
			fdialog_project->set_current_dir(fav_dir);
		} else {
			Ref<DirAccess> d = DirAccess::create(DirAccess::ACCESS_FILESYSTEM);
			project_path->set_text(d->get_current_dir());
			install_path->set_text(d->get_current_dir());
			fdialog_project->set_current_dir(d->get_current_dir());
		}

		create_dir->show();
		project_status_rect->show();
		project_browse->show();

		if (mode == MODE_IMPORT) {
			set_title(TTR("Import Existing Project"));
			set_ok_button_text(TTR("Import & Edit"));

			name_container->hide();
			install_path_container->hide();
			default_files_container->hide();

			// Project path dialog is also opened; no need to change focus.
		} else if (mode == MODE_NEW) {
			set_title(TTR("Create New Project"));
			set_ok_button_text(TTR("Create & Edit"));

			name_container->show();
			install_path_container->hide();
			default_files_container->show();

			callable_mp((Control *)project_name, &Control::grab_focus).call_deferred();
			callable_mp(project_name, &LineEdit::select_all).call_deferred();
		} else if (mode == MODE_INSTALL) {
			set_title(TTR("Install Project:") + " " + zip_title);
			set_ok_button_text(TTR("Install & Edit"));

			project_name->set_text(zip_title);

			name_container->show();
			install_path_container->hide();
			default_files_container->hide();

			callable_mp((Control *)project_path, &Control::grab_focus).call_deferred();
		} else if (mode == MODE_OPEN) {
			set_title(TTR("Open Game File: "));
			set_ok_button_text(TTR("Open"));
		}

		auto_dir = "";
		last_custom_target_dir = "";
		_update_target_auto_dir();
		if (create_dir->is_pressed()) {
			// Append `auto_dir` to target path.
			_create_dir_toggled(true);
		}
	}

	_validate_path();

	popup_centered(Size2(500, 0) * EDSCALE);
}

void ProjectDialog::_notification(int p_what) {
	switch (p_what) {
		case NOTIFICATION_THEME_CHANGED: {
			create_dir->set_icon(get_editor_theme_icon(SNAME("FolderCreate")));
			project_browse->set_icon(get_editor_theme_icon(SNAME("FolderBrowse")));
			install_browse->set_icon(get_editor_theme_icon(SNAME("FolderBrowse")));
			project_preview->set_project_icon(get_editor_theme_icon(SNAME("NSMBWBanner")));
		} break;
		case NOTIFICATION_READY: {
			fdialog_project = memnew(EditorFileDialog);
			fdialog_project->set_previews_enabled(false); // Crucial, otherwise the engine crashes.
			fdialog_project->set_access(EditorFileDialog::ACCESS_FILESYSTEM);
			fdialog_project->connect("dir_selected", callable_mp(this, &ProjectDialog::_project_path_selected));
			fdialog_project->connect("file_selected", callable_mp(this, &ProjectDialog::_project_path_selected));
			callable_mp((Node *)this, &Node::add_sibling).call_deferred(fdialog_project, false);
		} break;
	}
}

void ProjectDialog::_bind_methods() {
	ADD_SIGNAL(MethodInfo("project_created"));
	ADD_SIGNAL(MethodInfo("projects_updated"));
	ADD_SIGNAL(MethodInfo("project_error"));
}

ProjectDialog::ProjectDialog() {
	VBoxContainer *vb = memnew(VBoxContainer);
	vb->set_custom_minimum_size(Size2(500,0));
	add_child(vb);

	Label *preview_text = memnew(Label);
	preview_text->set_h_size_flags(Control::SIZE_EXPAND_FILL);
	preview_text->set_horizontal_alignment(HORIZONTAL_ALIGNMENT_CENTER);
	preview_text->set_text("Project Preview");
	vb->add_child(preview_text);

	project_preview = memnew(ProjectListItemControl);
	project_preview->is_preview = true;
	vb->add_child(project_preview);

	HSeparator *pvsp = memnew(HSeparator);
	pvsp->set_custom_minimum_size(Size2(0, 20));
	vb->add_child(pvsp);

	name_container = memnew(VBoxContainer);
	vb->add_child(name_container);

	Label *l = memnew(Label);
	l->set_text(TTR("Project Name:"));
	name_container->add_child(l);

	project_name = memnew(LineEdit);
	project_name->set_h_size_flags(Control::SIZE_EXPAND_FILL);
	name_container->add_child(project_name);

	project_path_container = memnew(VBoxContainer);
	vb->add_child(project_path_container);

	HBoxContainer *pphb_label = memnew(HBoxContainer);
	project_path_container->add_child(pphb_label);

	l = memnew(Label);
	l->set_text(TTR("Project Path:"));
	l->set_h_size_flags(Control::SIZE_EXPAND_FILL);
	pphb_label->add_child(l);

	create_dir = memnew(CheckButton);
	create_dir->set_text(TTR("Create Folder"));
	create_dir->set_pressed(true);
	pphb_label->add_child(create_dir);
	create_dir->connect("toggled", callable_mp(this, &ProjectDialog::_create_dir_toggled));

	HBoxContainer *pphb = memnew(HBoxContainer);
	project_path_container->add_child(pphb);

	project_path = memnew(LineEdit);
	project_path->set_h_size_flags(Control::SIZE_EXPAND_FILL);
	project_path->set_structured_text_bidi_override(TextServer::STRUCTURED_TEXT_FILE);
	pphb->add_child(project_path);

	install_path_container = memnew(VBoxContainer);
	vb->add_child(install_path_container);

	l = memnew(Label);
	l->set_text(TTR("Project Installation Path:"));
	install_path_container->add_child(l);

	HBoxContainer *iphb = memnew(HBoxContainer);
	install_path_container->add_child(iphb);

	install_path = memnew(LineEdit);
	install_path->set_h_size_flags(Control::SIZE_EXPAND_FILL);
	install_path->set_structured_text_bidi_override(TextServer::STRUCTURED_TEXT_FILE);
	iphb->add_child(install_path);

	// status icon
	project_status_rect = memnew(TextureRect);
	project_status_rect->set_stretch_mode(TextureRect::STRETCH_KEEP_CENTERED);
	pphb->add_child(project_status_rect);

	project_browse = memnew(Button);
	project_browse->set_text(TTR("Browse"));
	project_browse->connect(SceneStringName(pressed), callable_mp(this, &ProjectDialog::_browse_project_path));
	pphb->add_child(project_browse);

	// install status icon
	install_status_rect = memnew(TextureRect);
	install_status_rect->set_stretch_mode(TextureRect::STRETCH_KEEP_CENTERED);
	iphb->add_child(install_status_rect);

	install_browse = memnew(Button);
	install_browse->set_text(TTR("Browse"));
	install_browse->connect(SceneStringName(pressed), callable_mp(this, &ProjectDialog::_browse_install_path));
	iphb->add_child(install_browse);

	msg = memnew(Label);
	msg->set_horizontal_alignment(HORIZONTAL_ALIGNMENT_CENTER);
	msg->set_custom_minimum_size(Size2(200, 0) * EDSCALE);
	msg->set_autowrap_mode(TextServer::AUTOWRAP_WORD_SMART);
	vb->add_child(msg);

	HSeparator *cvsp = memnew(HSeparator);
	cvsp->set_custom_minimum_size(Size2(0, 20));
	vb->add_child(cvsp);

	default_files_container = memnew(VBoxContainer);
	vb->add_child(default_files_container);
	l = memnew(Label);
	l->set_horizontal_alignment(HORIZONTAL_ALIGNMENT_CENTER);
	l->set_text(TTR("Version Control"));
	default_files_container->add_child(l);
	vcs_metadata_selection = memnew(OptionButton);
	vcs_metadata_selection->set_custom_minimum_size(Size2(100, 20));
	vcs_metadata_selection->add_item(TTR("None"), (int)EditorVCSInterface::VCSMetadata::NONE);
	vcs_metadata_selection->add_item(TTR("Git"), (int)EditorVCSInterface::VCSMetadata::GIT);
	vcs_metadata_selection->select((int)EditorVCSInterface::VCSMetadata::GIT);
	default_files_container->add_child(vcs_metadata_selection);
	Control *spacer = memnew(Control);
	spacer->set_h_size_flags(Control::SIZE_EXPAND_FILL);
	default_files_container->add_child(spacer);

	HSeparator *vvsp = memnew(HSeparator);
	vvsp->set_custom_minimum_size(Size2(0, 20));
	default_files_container->add_child(vvsp);

	fdialog_install = memnew(EditorFileDialog);
	fdialog_install->set_previews_enabled(false); //Crucial, otherwise the engine crashes.
	fdialog_install->set_access(EditorFileDialog::ACCESS_FILESYSTEM);
	add_child(fdialog_install);

	project_name->connect(SceneStringName(text_changed), callable_mp(this, &ProjectDialog::_project_name_changed).unbind(1));
	project_path->connect(SceneStringName(text_changed), callable_mp(this, &ProjectDialog::_project_path_changed).unbind(1));
	install_path->connect(SceneStringName(text_changed), callable_mp(this, &ProjectDialog::_install_path_changed).unbind(1));
	fdialog_install->connect("dir_selected", callable_mp(this, &ProjectDialog::_install_path_selected));
	fdialog_install->connect("file_selected", callable_mp(this, &ProjectDialog::_install_path_selected));

	set_hide_on_ok(false);

	dialog_error = memnew(AcceptDialog);
	add_child(dialog_error);
}
