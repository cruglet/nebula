/**************************************************************************/
/*  nav_region.h                                                          */
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

#ifndef NAV_REGION_H
#define NAV_REGION_H

#include "nav_base.h"
#include "nav_utils.h"

#include "core/os/rw_lock.h"
#include "scene/resources/navigation_mesh.h"

class NavRegion : public NavBase {
	NavMap *map = nullptr;
	Transform3D transform;
	Vector<gd::Edge::Connection> connections;
	bool enabled = true;

	bool use_edge_connections = true;

	bool polygons_dirty = true;

	/// Cache
	LocalVector<gd::Polygon> polygons;

	real_t surface_area = 0.0;

	RWLock navmesh_rwlock;
	Vector<Vector3> pending_navmesh_vertices;
	Vector<Vector<int>> pending_navmesh_polygons;

public:
	NavRegion() {
		type = NavigationUtilities::PathSegmentType::PATH_SEGMENT_TYPE_REGION;
	}

	void scratch_polygons() {
		polygons_dirty = true;
	}

	void set_enabled(bool p_enabled);
	bool get_enabled() const { return enabled; }

	void set_map(NavMap *p_map);
	NavMap *get_map() const {
		return map;
	}

	void set_use_edge_connections(bool p_enabled);
	bool get_use_edge_connections() const {
		return use_edge_connections;
	}

	void set_transform(Transform3D transform);
	const Transform3D &get_transform() const {
		return transform;
	}

	void set_navigation_mesh(Ref<NavigationMesh> p_navigation_mesh);

	Vector<gd::Edge::Connection> &get_connections() {
		return connections;
	}
	int get_connections_count() const;
	Vector3 get_connection_pathway_start(int p_connection_id) const;
	Vector3 get_connection_pathway_end(int p_connection_id) const;

	LocalVector<gd::Polygon> const &get_polygons() const {
		return polygons;
	}

	Vector3 get_random_point(uint32_t p_navigation_layers, bool p_uniformly) const;

	real_t get_surface_area() const { return surface_area; };

	bool sync();

private:
	void update_polygons();
};

#endif // NAV_REGION_H
