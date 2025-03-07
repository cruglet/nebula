/**************************************************************************/
/*  navigation_obstacle_2d_editor_plugin.h                                */
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

#ifndef NAVIGATION_OBSTACLE_2D_EDITOR_PLUGIN_H
#define NAVIGATION_OBSTACLE_2D_EDITOR_PLUGIN_H

#include "editor/plugins/abstract_polygon_2d_editor.h"
#include "scene/2d/navigation_obstacle_2d.h"

class NavigationObstacle2DEditor : public AbstractPolygon2DEditor {
	GDCLASS(NavigationObstacle2DEditor, AbstractPolygon2DEditor);

	NavigationObstacle2D *node = nullptr;

protected:
	virtual Node2D *_get_node() const override;
	virtual void _set_node(Node *p_polygon) override;

	virtual Variant _get_polygon(int p_idx) const override;
	virtual void _set_polygon(int p_idx, const Variant &p_polygon) const override;

	virtual void _action_add_polygon(const Variant &p_polygon) override;
	virtual void _action_remove_polygon(int p_idx) override;
	virtual void _action_set_polygon(int p_idx, const Variant &p_previous, const Variant &p_polygon) override;

public:
	NavigationObstacle2DEditor();
};

class NavigationObstacle2DEditorPlugin : public AbstractPolygon2DEditorPlugin {
	GDCLASS(NavigationObstacle2DEditorPlugin, AbstractPolygon2DEditorPlugin);

public:
	NavigationObstacle2DEditorPlugin();
};

#endif // NAVIGATION_OBSTACLE_2D_EDITOR_PLUGIN_H
