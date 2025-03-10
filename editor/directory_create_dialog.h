/**************************************************************************/
/*  directory_create_dialog.h                                             */
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

#ifndef DIRECTORY_CREATE_DIALOG_H
#define DIRECTORY_CREATE_DIALOG_H

#include "scene/gui/dialogs.h"

class EditorValidationPanel;
class Label;
class LineEdit;

class DirectoryCreateDialog : public ConfirmationDialog {
	GDCLASS(DirectoryCreateDialog, ConfirmationDialog);

	String base_dir;

	Label *label = nullptr;
	LineEdit *dir_path = nullptr;
	EditorValidationPanel *validation_panel = nullptr;

	String _validate_path(const String &p_path) const;
	void _on_dir_path_changed();

protected:
	static void _bind_methods();

	virtual void ok_pressed() override;
	virtual void _post_popup() override;

public:
	void config(const String &p_base_dir);

	DirectoryCreateDialog();
};

#endif // DIRECTORY_CREATE_DIALOG_H
