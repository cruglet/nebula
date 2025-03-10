/**************************************************************************/
/*  navigation_path_query_result_2d.h                                     */
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

#ifndef NAVIGATION_PATH_QUERY_RESULT_2D_H
#define NAVIGATION_PATH_QUERY_RESULT_2D_H

#include "core/object/ref_counted.h"
#include "servers/navigation/navigation_utilities.h"

class NavigationPathQueryResult2D : public RefCounted {
	GDCLASS(NavigationPathQueryResult2D, RefCounted);

	Vector<Vector2> path;
	Vector<int32_t> path_types;
	TypedArray<RID> path_rids;
	Vector<int64_t> path_owner_ids;

protected:
	static void _bind_methods();

public:
	enum PathSegmentType {
		PATH_SEGMENT_TYPE_REGION = 0,
		PATH_SEGMENT_TYPE_LINK = 1,
	};

	void set_path(const Vector<Vector2> &p_path);
	const Vector<Vector2> &get_path() const;

	void set_path_types(const Vector<int32_t> &p_path_types);
	const Vector<int32_t> &get_path_types() const;

	void set_path_rids(const TypedArray<RID> &p_path_rids);
	TypedArray<RID> get_path_rids() const;

	void set_path_owner_ids(const Vector<int64_t> &p_path_owner_ids);
	const Vector<int64_t> &get_path_owner_ids() const;

	void reset();
};

VARIANT_ENUM_CAST(NavigationPathQueryResult2D::PathSegmentType);

#endif // NAVIGATION_PATH_QUERY_RESULT_2D_H
