/**************************************************************************/
/*  register_types.cpp                                                    */
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

#include "register_types.h"

#include "2d/nebula_navigation_server_2d.h"
#include "3d/nebula_navigation_server_3d.h"

#ifndef DISABLE_DEPRECATED
#ifndef _3D_DISABLED
#include "3d/navigation_mesh_generator.h"
#endif
#endif // DISABLE_DEPRECATED

#ifdef TOOLS_ENABLED
#include "editor/navigation_mesh_editor_plugin.h"
#endif

#include "core/config/engine.h"
#include "servers/navigation_server_2d.h"
#include "servers/navigation_server_3d.h"

#ifndef DISABLE_DEPRECATED
#ifndef _3D_DISABLED
NavigationMeshGenerator *_nav_mesh_generator = nullptr;
#endif
#endif // DISABLE_DEPRECATED

NavigationServer3D *new_navigation_server_3d() {
	return memnew(NebulaNavigationServer3D);
}

NavigationServer2D *new_navigation_server_2d() {
	return memnew(NebulaNavigationServer2D);
}

void initialize_navigation_module(ModuleInitializationLevel p_level) {
	if (p_level == MODULE_INITIALIZATION_LEVEL_SERVERS) {
		NavigationServer3DManager::set_default_server(new_navigation_server_3d);
		NavigationServer2DManager::set_default_server(new_navigation_server_2d);

#ifndef DISABLE_DEPRECATED
#ifndef _3D_DISABLED
		_nav_mesh_generator = memnew(NavigationMeshGenerator);
		GDREGISTER_CLASS(NavigationMeshGenerator);
		Engine::get_singleton()->add_singleton(Engine::Singleton("NavigationMeshGenerator", NavigationMeshGenerator::get_singleton()));
#endif
#endif // DISABLE_DEPRECATED
	}

#ifdef TOOLS_ENABLED
	if (p_level == MODULE_INITIALIZATION_LEVEL_EDITOR) {
		EditorPlugins::add_by_type<NavigationMeshEditorPlugin>();
	}
#endif
}

void uninitialize_navigation_module(ModuleInitializationLevel p_level) {
	if (p_level != MODULE_INITIALIZATION_LEVEL_SERVERS) {
		return;
	}

#ifndef DISABLE_DEPRECATED
#ifndef _3D_DISABLED
	if (_nav_mesh_generator) {
		memdelete(_nav_mesh_generator);
	}
#endif
#endif // DISABLE_DEPRECATED
}
