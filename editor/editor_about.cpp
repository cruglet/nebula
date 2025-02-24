/**************************************************************************/
/*  editor_about.cpp                                                      */
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

#include "editor_about.h"

#include "core/license.gen.h"
#include "core/os/time.h"
#include "core/version.h"
#include "editor/editor_string_names.h"
#include "editor/themes/editor_scale.h"
#include "scene/gui/item_list.h"
#include "scene/resources/style_box.h"

// The metadata key used to store and retrieve the version text to copy to the clipboard.
const String EditorAbout::META_TEXT_TO_COPY = "text_to_copy";

void EditorAbout::_notification(int p_what) {
	switch (p_what) {
		case NOTIFICATION_THEME_CHANGED: {
			const Ref<Font> font = get_theme_font(SNAME("source"), EditorStringName(EditorFonts));
			const int font_size = get_theme_font_size(SNAME("source_size"), EditorStringName(EditorFonts));

			_about_text->begin_bulk_theme_override();
			_about_text->add_theme_font_override("normal_font", font);
			_about_text->add_theme_font_size_override("normal_font_size", font_size);
			_about_text->add_theme_constant_override(SceneStringName(line_separation), 4 * EDSCALE);
			_about_text->end_bulk_theme_override();

			license_text_label->add_theme_font_override("normal_font", font);

			_logo->set_texture(get_editor_theme_icon(SNAME("Logo")));

			for (ItemList *il : name_lists) {
				for (int i = 0; i < il->get_item_count(); i++) {
					if (il->get_item_metadata(i)) {
						il->set_item_icon(i, get_theme_icon(SNAME("ExternalLink"), EditorStringName(EditorIcons)));
						il->set_item_icon_modulate(i, get_theme_color(SNAME("font_disabled_color"), EditorStringName(Editor)));
					}
				}
			}
		} break;
	}
}

void EditorAbout::_license_tree_selected() {
	TreeItem *selected = _about_tree->get_selected();
	_about_text->scroll_to_line(0);
	_about_text->set_text(selected->get_metadata(0));
}


void EditorAbout::_item_with_website_selected(int p_id, ItemList *p_il) {
	const String website = p_il->get_item_metadata(p_id);
	if (!website.is_empty()) {
		OS::get_singleton()->shell_open(website);
	}
}

void EditorAbout::_item_list_resized(ItemList *p_il) {
	p_il->set_fixed_column_width(p_il->get_size().x / 3.0 - 16 * EDSCALE * 2.5); // Weird. Should be 3.0 and that's it?.
}

ScrollContainer *EditorAbout::_populate_list(const String &p_name, const List<String> &p_sections, const char *const *const p_src[], const int p_single_column_flags, const bool p_allow_website) {
	ScrollContainer *sc = memnew(ScrollContainer);
	sc->set_name(p_name);
	sc->set_v_size_flags(Control::SIZE_EXPAND);

	VBoxContainer *vbc = memnew(VBoxContainer);
	vbc->set_h_size_flags(Control::SIZE_EXPAND_FILL);
	sc->add_child(vbc);

	Ref<StyleBoxEmpty> empty_stylebox = memnew(StyleBoxEmpty);

	int i = 0;
	for (List<String>::ConstIterator itr = p_sections.begin(); itr != p_sections.end(); ++itr, ++i) {
		bool single_column = p_single_column_flags & (1 << i);
		const char *const *names_ptr = p_src[i];
		if (*names_ptr) {
			Label *lbl = memnew(Label);
			lbl->set_theme_type_variation("HeaderSmall");
			lbl->set_text(*itr);
			vbc->add_child(lbl);

			ItemList *il = memnew(ItemList);
			il->set_auto_translate_mode(AUTO_TRANSLATE_MODE_DISABLED);
			il->set_h_size_flags(Control::SIZE_EXPAND_FILL);
			il->set_same_column_width(true);
			il->set_auto_height(true);
			il->set_mouse_filter(Control::MOUSE_FILTER_IGNORE);
			il->set_focus_mode(Control::FOCUS_NONE);
			il->add_theme_constant_override("h_separation", 16 * EDSCALE);
			if (p_allow_website) {
				il->set_focus_mode(Control::FOCUS_CLICK);
				il->set_mouse_filter(Control::MOUSE_FILTER_PASS);

				il->connect("item_activated", callable_mp(this, &EditorAbout::_item_with_website_selected).bind(il));
				il->connect(SceneStringName(resized), callable_mp(this, &EditorAbout::_item_list_resized).bind(il));
				il->connect(SceneStringName(focus_exited), callable_mp(il, &ItemList::deselect_all));

				il->add_theme_style_override("focus", empty_stylebox);
				il->add_theme_style_override("selected", empty_stylebox);

				while (*names_ptr) {
					const String name = String::utf8(*names_ptr++);
					const String identifier = name.get_slice("<", 0);
					const String website = name.get_slice_count("<") == 1 ? "" : name.get_slice("<", 1).trim_suffix(">");

					const int name_item_id = il->add_item(identifier, nullptr, false);
					il->set_item_tooltip_enabled(name_item_id, false);

					if (!website.is_empty()) {
						il->set_item_selectable(name_item_id, true);
						il->set_item_metadata(name_item_id, website);
						il->set_item_tooltip(name_item_id, website + "\n\n" + TTR("Double-click to open in browser."));
						il->set_item_tooltip_enabled(name_item_id, true);
					}

					if (!*names_ptr && name.contains(" anonymous ")) {
						il->set_item_disabled(name_item_id, true);
					}
				}
			} else {
				while (*names_ptr) {
					il->add_item(String::utf8(*names_ptr++), nullptr, false);
				}
			}
			il->set_max_columns(single_column ? 1 : 16);

			name_lists.append(il);

			vbc->add_child(il);

			HSeparator *hs = memnew(HSeparator);
			hs->set_modulate(Color(0, 0, 0, 0));
			vbc->add_child(hs);
		}
	}

	return sc;
}

EditorAbout::EditorAbout() {
	set_title(TTR("About Nebula"));
	set_hide_on_ok(true);

	VBoxContainer *vbc = memnew(VBoxContainer);
	add_child(vbc);

	HBoxContainer *hbc = memnew(HBoxContainer);
	hbc->set_h_size_flags(Control::SIZE_EXPAND_FILL);
	hbc->set_alignment(BoxContainer::ALIGNMENT_CENTER);
	hbc->add_theme_constant_override("separation", 30 * EDSCALE);
	vbc->add_child(hbc);

	_logo = memnew(TextureRect);
	_logo->set_stretch_mode(TextureRect::STRETCH_KEEP_ASPECT_CENTERED);
	hbc->add_child(_logo);

	String build_date;
	if (VERSION_TIMESTAMP > 0) {
		build_date = Time::get_singleton()->get_datetime_string_from_unix_time(VERSION_TIMESTAMP, true) + " UTC";
	} else {
		build_date = TTR("(unknown)");
	}

	TabContainer *tc = memnew(TabContainer);
	tc->set_tab_alignment(TabBar::ALIGNMENT_CENTER);
	tc->set_custom_minimum_size(Size2(400, 200) * EDSCALE);
	tc->set_v_size_flags(Control::SIZE_EXPAND_FILL);
	tc->set_theme_type_variation("TabContainerOdd");
	vbc->add_child(tc);

	// About
	VBoxContainer *about = memnew(VBoxContainer);
	about->set_name(TTR("About"));
	about->set_h_size_flags(Control::SIZE_EXPAND_FILL);
	tc->add_child(about);

	Label *about_label = memnew(Label);
	about_label->set_h_size_flags(Control::SIZE_EXPAND_FILL);
	about_label->set_autowrap_mode(TextServer::AUTOWRAP_WORD_SMART);
	about_label->set_text(TTR("Nebula Engine is a fork of the Godot Engine. This means that Nebula relies on a number of its third-party free and open source libraries, below they are listed:\n"));
	about_label->set_size(Size2(630, 1) * EDSCALE);
	about->add_child(about_label);

	HSplitContainer *about_hbc = memnew(HSplitContainer);
	about_hbc->set_h_size_flags(Control::SIZE_EXPAND_FILL);
	about_hbc->set_v_size_flags(Control::SIZE_EXPAND_FILL);
	about_hbc->set_split_offset(240 * EDSCALE);
	about->add_child(about_hbc);

	_about_tree = memnew(Tree);
	_about_tree->set_auto_translate_mode(AUTO_TRANSLATE_MODE_DISABLED);
	_about_tree->set_hide_root(true);
	TreeItem *root = _about_tree->create_item();
	TreeItem *about_ti_lc = _about_tree->create_item(root);
	about_ti_lc->set_text(0, TTR("Godot Dependencies"));
	about_ti_lc->set_selectable(0, false);
	String long_text = "";

	for (int i = 0; i < LICENSE_COUNT; i++) {
		TreeItem *ti = _about_tree->create_item(about_ti_lc);
		String licensename = String::utf8(LICENSE_NAMES[i]);
		ti->set_text(0, licensename);
		long_text += "- " + licensename + "\n\n";
		String licensebody = String::utf8(LICENSE_BODIES[i]);
		ti->set_metadata(0, licensebody);
		long_text += "    " + licensebody.replace("\n", "\n    ") + "\n\n";
	}
	about_hbc->add_child(_about_tree);

	_about_text = memnew(RichTextLabel);
	_about_text->set_threaded(true);
	_about_text->set_h_size_flags(Control::SIZE_EXPAND_FILL);
	_about_text->set_v_size_flags(Control::SIZE_EXPAND_FILL);
	_about_text->set_text("TODO");
	about_hbc->add_child(_about_text);

	// License.
	license_text_label = memnew(RichTextLabel);
	license_text_label->set_threaded(true);
	license_text_label->set_name(TTR("Godot License"));
	license_text_label->set_h_size_flags(Control::SIZE_EXPAND_FILL);
	license_text_label->set_v_size_flags(Control::SIZE_EXPAND_FILL);
	license_text_label->set_text(String::utf8(NEBULA_LICENSE_TEXT));
	tc->add_child(license_text_label);

	_about_tree->connect(SceneStringName(item_selected), callable_mp(this, &EditorAbout::_license_tree_selected));
}

EditorAbout::~EditorAbout() {}
