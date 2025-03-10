/**************************************************************************/
/*  nav_link.h                                                            */
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

#ifndef NAV_LINK_H
#define NAV_LINK_H

#include "nav_base.h"
#include "nav_utils.h"

class NavLink : public NavBase {
	NavMap *map = nullptr;
	bool bidirectional = true;
	Vector3 start_position;
	Vector3 end_position;
	bool enabled = true;

	bool link_dirty = true;

public:
	NavLink() {
		type = NavigationUtilities::PathSegmentType::PATH_SEGMENT_TYPE_LINK;
	}

	void set_map(NavMap *p_map);
	NavMap *get_map() const {
		return map;
	}

	void set_enabled(bool p_enabled);
	bool get_enabled() const { return enabled; }

	void set_bidirectional(bool p_bidirectional);
	bool is_bidirectional() const {
		return bidirectional;
	}

	void set_start_position(Vector3 p_position);
	Vector3 get_start_position() const {
		return start_position;
	}

	void set_end_position(Vector3 p_position);
	Vector3 get_end_position() const {
		return end_position;
	}

	bool check_dirty();
};

#endif // NAV_LINK_H
