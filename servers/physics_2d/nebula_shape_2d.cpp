/**************************************************************************/
/*  nebula_shape_2d.cpp                                                    */
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

#include "nebula_shape_2d.h"

#include "core/math/geometry_2d.h"
#include "core/templates/sort_array.h"

void NebulaShape2D::configure(const Rect2 &p_aabb) {
	aabb = p_aabb;
	configured = true;
	for (const KeyValue<NebulaShapeOwner2D *, int> &E : owners) {
		NebulaShapeOwner2D *co = const_cast<NebulaShapeOwner2D *>(E.key);
		co->_shape_changed();
	}
}

Vector2 NebulaShape2D::get_support(const Vector2 &p_normal) const {
	Vector2 res[2];
	int amnt;
	get_supports(p_normal, res, amnt);
	return res[0];
}

void NebulaShape2D::add_owner(NebulaShapeOwner2D *p_owner) {
	HashMap<NebulaShapeOwner2D *, int>::Iterator E = owners.find(p_owner);
	if (E) {
		E->value++;
	} else {
		owners[p_owner] = 1;
	}
}

void NebulaShape2D::remove_owner(NebulaShapeOwner2D *p_owner) {
	HashMap<NebulaShapeOwner2D *, int>::Iterator E = owners.find(p_owner);
	ERR_FAIL_COND(!E);
	E->value--;
	if (E->value == 0) {
		owners.remove(E);
	}
}

bool NebulaShape2D::is_owner(NebulaShapeOwner2D *p_owner) const {
	return owners.has(p_owner);
}

const HashMap<NebulaShapeOwner2D *, int> &NebulaShape2D::get_owners() const {
	return owners;
}

NebulaShape2D::~NebulaShape2D() {
	ERR_FAIL_COND(owners.size());
}

/*********************************************************/
/*********************************************************/
/*********************************************************/

void NebulaWorldBoundaryShape2D::get_supports(const Vector2 &p_normal, Vector2 *r_supports, int &r_amount) const {
	r_amount = 0;
}

bool NebulaWorldBoundaryShape2D::contains_point(const Vector2 &p_point) const {
	return normal.dot(p_point) < d;
}

bool NebulaWorldBoundaryShape2D::intersect_segment(const Vector2 &p_begin, const Vector2 &p_end, Vector2 &r_point, Vector2 &r_normal) const {
	Vector2 segment = p_begin - p_end;
	real_t den = normal.dot(segment);

	//printf("den is %i\n",den);
	if (Math::abs(den) <= CMP_EPSILON) {
		return false;
	}

	real_t dist = (normal.dot(p_begin) - d) / den;
	//printf("dist is %i\n",dist);

	if (dist < -CMP_EPSILON || dist > (1.0 + CMP_EPSILON)) {
		return false;
	}

	r_point = p_begin + segment * -dist;
	r_normal = normal;

	return true;
}

real_t NebulaWorldBoundaryShape2D::get_moment_of_inertia(real_t p_mass, const Size2 &p_scale) const {
	return 0;
}

void NebulaWorldBoundaryShape2D::set_data(const Variant &p_data) {
	ERR_FAIL_COND(p_data.get_type() != Variant::ARRAY);
	Array arr = p_data;
	ERR_FAIL_COND(arr.size() != 2);
	normal = arr[0];
	d = arr[1];
	configure(Rect2(Vector2(-1e15, -1e15), Vector2(1e15 * 2, 1e15 * 2)));
}

Variant NebulaWorldBoundaryShape2D::get_data() const {
	Array arr;
	arr.resize(2);
	arr[0] = normal;
	arr[1] = d;
	return arr;
}

/*********************************************************/
/*********************************************************/
/*********************************************************/

void NebulaSeparationRayShape2D::get_supports(const Vector2 &p_normal, Vector2 *r_supports, int &r_amount) const {
	r_amount = 1;

	if (p_normal.y > 0) {
		*r_supports = Vector2(0, length);
	} else {
		*r_supports = Vector2();
	}
}

bool NebulaSeparationRayShape2D::contains_point(const Vector2 &p_point) const {
	return false;
}

bool NebulaSeparationRayShape2D::intersect_segment(const Vector2 &p_begin, const Vector2 &p_end, Vector2 &r_point, Vector2 &r_normal) const {
	return false; //rays can't be intersected
}

real_t NebulaSeparationRayShape2D::get_moment_of_inertia(real_t p_mass, const Size2 &p_scale) const {
	return 0; //rays are mass-less
}

void NebulaSeparationRayShape2D::set_data(const Variant &p_data) {
	Dictionary d = p_data;
	length = d["length"];
	slide_on_slope = d["slide_on_slope"];
	configure(Rect2(0, 0, 0.001, length));
}

Variant NebulaSeparationRayShape2D::get_data() const {
	Dictionary d;
	d["length"] = length;
	d["slide_on_slope"] = slide_on_slope;
	return d;
}

/*********************************************************/
/*********************************************************/
/*********************************************************/

void NebulaSegmentShape2D::get_supports(const Vector2 &p_normal, Vector2 *r_supports, int &r_amount) const {
	if (Math::abs(p_normal.dot(n)) > segment_is_valid_support_threshold) {
		r_supports[0] = a;
		r_supports[1] = b;
		r_amount = 2;
		return;
	}

	real_t dp = p_normal.dot(b - a);
	if (dp > 0) {
		*r_supports = b;
	} else {
		*r_supports = a;
	}
	r_amount = 1;
}

bool NebulaSegmentShape2D::contains_point(const Vector2 &p_point) const {
	return false;
}

bool NebulaSegmentShape2D::intersect_segment(const Vector2 &p_begin, const Vector2 &p_end, Vector2 &r_point, Vector2 &r_normal) const {
	if (!Geometry2D::segment_intersects_segment(p_begin, p_end, a, b, &r_point)) {
		return false;
	}

	if (n.dot(p_begin) > n.dot(a)) {
		r_normal = n;
	} else {
		r_normal = -n;
	}

	return true;
}

real_t NebulaSegmentShape2D::get_moment_of_inertia(real_t p_mass, const Size2 &p_scale) const {
	return p_mass * ((a * p_scale).distance_squared_to(b * p_scale)) / 12;
}

void NebulaSegmentShape2D::set_data(const Variant &p_data) {
	ERR_FAIL_COND(p_data.get_type() != Variant::RECT2);

	Rect2 r = p_data;
	a = r.position;
	b = r.size;
	n = (b - a).orthogonal();

	Rect2 aabb_new;
	aabb_new.position = a;
	aabb_new.expand_to(b);
	if (aabb_new.size.x == 0) {
		aabb_new.size.x = 0.001;
	}
	if (aabb_new.size.y == 0) {
		aabb_new.size.y = 0.001;
	}
	configure(aabb_new);
}

Variant NebulaSegmentShape2D::get_data() const {
	Rect2 r;
	r.position = a;
	r.size = b;
	return r;
}

/*********************************************************/
/*********************************************************/
/*********************************************************/

void NebulaCircleShape2D::get_supports(const Vector2 &p_normal, Vector2 *r_supports, int &r_amount) const {
	r_amount = 1;
	*r_supports = p_normal * radius;
}

bool NebulaCircleShape2D::contains_point(const Vector2 &p_point) const {
	return p_point.length_squared() < radius * radius;
}

bool NebulaCircleShape2D::intersect_segment(const Vector2 &p_begin, const Vector2 &p_end, Vector2 &r_point, Vector2 &r_normal) const {
	Vector2 line_vec = p_end - p_begin;

	real_t a, b, c;

	a = line_vec.dot(line_vec);
	b = 2 * p_begin.dot(line_vec);
	c = p_begin.dot(p_begin) - radius * radius;

	real_t sqrtterm = b * b - 4 * a * c;

	if (sqrtterm < 0) {
		return false;
	}
	sqrtterm = Math::sqrt(sqrtterm);
	real_t res = (-b - sqrtterm) / (2 * a);

	if (res < 0 || res > 1 + CMP_EPSILON) {
		return false;
	}

	r_point = p_begin + line_vec * res;
	r_normal = r_point.normalized();
	return true;
}

real_t NebulaCircleShape2D::get_moment_of_inertia(real_t p_mass, const Size2 &p_scale) const {
	real_t a = radius * p_scale.x;
	real_t b = radius * p_scale.y;
	return p_mass * (a * a + b * b) / 4;
}

void NebulaCircleShape2D::set_data(const Variant &p_data) {
	ERR_FAIL_COND(!p_data.is_num());
	radius = p_data;
	configure(Rect2(-radius, -radius, radius * 2, radius * 2));
}

Variant NebulaCircleShape2D::get_data() const {
	return radius;
}

/*********************************************************/
/*********************************************************/
/*********************************************************/

void NebulaRectangleShape2D::get_supports(const Vector2 &p_normal, Vector2 *r_supports, int &r_amount) const {
	for (int i = 0; i < 2; i++) {
		Vector2 ag;
		ag[i] = 1.0;
		real_t dp = ag.dot(p_normal);
		if (Math::abs(dp) <= segment_is_valid_support_threshold) {
			continue;
		}

		real_t sgn = dp > 0 ? 1.0 : -1.0;

		r_amount = 2;

		r_supports[0][i] = half_extents[i] * sgn;
		r_supports[0][i ^ 1] = half_extents[i ^ 1];

		r_supports[1][i] = half_extents[i] * sgn;
		r_supports[1][i ^ 1] = -half_extents[i ^ 1];

		return;
	}

	/* USE POINT */

	r_amount = 1;
	r_supports[0] = Vector2(
			(p_normal.x < 0) ? -half_extents.x : half_extents.x,
			(p_normal.y < 0) ? -half_extents.y : half_extents.y);
}

bool NebulaRectangleShape2D::contains_point(const Vector2 &p_point) const {
	real_t x = p_point.x;
	real_t y = p_point.y;
	real_t edge_x = half_extents.x;
	real_t edge_y = half_extents.y;
	return (x >= -edge_x) && (x < edge_x) && (y >= -edge_y) && (y < edge_y);
}

bool NebulaRectangleShape2D::intersect_segment(const Vector2 &p_begin, const Vector2 &p_end, Vector2 &r_point, Vector2 &r_normal) const {
	return get_aabb().intersects_segment(p_begin, p_end, &r_point, &r_normal);
}

real_t NebulaRectangleShape2D::get_moment_of_inertia(real_t p_mass, const Size2 &p_scale) const {
	Vector2 he2 = half_extents * 2 * p_scale;
	return p_mass * he2.dot(he2) / 12.0;
}

void NebulaRectangleShape2D::set_data(const Variant &p_data) {
	ERR_FAIL_COND(p_data.get_type() != Variant::VECTOR2);

	half_extents = p_data;
	configure(Rect2(-half_extents, half_extents * 2.0));
}

Variant NebulaRectangleShape2D::get_data() const {
	return half_extents;
}

/*********************************************************/
/*********************************************************/
/*********************************************************/

void NebulaCapsuleShape2D::get_supports(const Vector2 &p_normal, Vector2 *r_supports, int &r_amount) const {
	Vector2 n = p_normal;

	real_t h = height * 0.5 - radius; // half-height of the rectangle part

	if (h > 0 && Math::abs(n.x) > segment_is_valid_support_threshold) {
		// make it flat
		n.y = 0.0;
		n.x = SIGN(n.x) * radius;

		r_amount = 2;
		r_supports[0] = n;
		r_supports[0].y += h;
		r_supports[1] = n;
		r_supports[1].y -= h;
	} else {
		n *= radius;
		n.y += (n.y > 0) ? h : -h;
		r_amount = 1;
		*r_supports = n;
	}
}

bool NebulaCapsuleShape2D::contains_point(const Vector2 &p_point) const {
	Vector2 p = p_point;
	p.y = Math::abs(p.y);
	p.y -= height * 0.5 - radius;
	if (p.y < 0) {
		p.y = 0;
	}

	return p.length_squared() < radius * radius;
}

bool NebulaCapsuleShape2D::intersect_segment(const Vector2 &p_begin, const Vector2 &p_end, Vector2 &r_point, Vector2 &r_normal) const {
	real_t d = 1e10;
	Vector2 n = (p_end - p_begin).normalized();
	bool collided = false;

	//try spheres
	for (int i = 0; i < 2; i++) {
		Vector2 begin = p_begin;
		Vector2 end = p_end;
		real_t ofs = (i == 0) ? -height * 0.5 + radius : height * 0.5 - radius;
		begin.y += ofs;
		end.y += ofs;

		Vector2 line_vec = end - begin;

		real_t a, b, c;

		a = line_vec.dot(line_vec);
		b = 2 * begin.dot(line_vec);
		c = begin.dot(begin) - radius * radius;

		real_t sqrtterm = b * b - 4 * a * c;

		if (sqrtterm < 0) {
			continue;
		}

		sqrtterm = Math::sqrt(sqrtterm);
		real_t res = (-b - sqrtterm) / (2 * a);

		if (res < 0 || res > 1 + CMP_EPSILON) {
			continue;
		}

		Vector2 point = begin + line_vec * res;
		Vector2 pointf(point.x, point.y - ofs);
		real_t pd = n.dot(pointf);
		if (pd < d) {
			r_point = pointf;
			r_normal = point.normalized();
			d = pd;
			collided = true;
		}
	}

	Vector2 rpos, rnorm;
	if (Rect2(Point2(-radius, -height * 0.5 + radius), Size2(radius * 2.0, height - radius * 2)).intersects_segment(p_begin, p_end, &rpos, &rnorm)) {
		real_t pd = n.dot(rpos);
		if (pd < d) {
			r_point = rpos;
			r_normal = rnorm;
			d = pd;
			collided = true;
		}
	}

	//return get_aabb().intersects_segment(p_begin,p_end,&r_point,&r_normal);
	return collided; //todo
}

real_t NebulaCapsuleShape2D::get_moment_of_inertia(real_t p_mass, const Size2 &p_scale) const {
	Vector2 he2 = Vector2(radius * 2, height) * p_scale;
	return p_mass * he2.dot(he2) / 12.0;
}

void NebulaCapsuleShape2D::set_data(const Variant &p_data) {
	ERR_FAIL_COND(p_data.get_type() != Variant::ARRAY && p_data.get_type() != Variant::VECTOR2);

	if (p_data.get_type() == Variant::ARRAY) {
		Array arr = p_data;
		ERR_FAIL_COND(arr.size() != 2);
		height = arr[0];
		radius = arr[1];
	} else {
		Point2 p = p_data;
		radius = p.x;
		height = p.y;
	}

	Point2 he(radius, height * 0.5);
	configure(Rect2(-he, he * 2));
}

Variant NebulaCapsuleShape2D::get_data() const {
	return Point2(height, radius);
}

/*********************************************************/
/*********************************************************/
/*********************************************************/

void NebulaConvexPolygonShape2D::get_supports(const Vector2 &p_normal, Vector2 *r_supports, int &r_amount) const {
	int support_idx = -1;
	real_t d = -1e10;
	r_amount = 0;

	for (int i = 0; i < point_count; i++) {
		//test point
		real_t ld = p_normal.dot(points[i].pos);
		if (ld > d) {
			support_idx = i;
			d = ld;
		}

		//test segment
		if (points[i].normal.dot(p_normal) > segment_is_valid_support_threshold) {
			r_amount = 2;
			r_supports[0] = points[i].pos;
			r_supports[1] = points[(i + 1) % point_count].pos;
			return;
		}
	}

	ERR_FAIL_COND_MSG(support_idx == -1, "Convex polygon shape support not found.");

	r_amount = 1;
	r_supports[0] = points[support_idx].pos;
}

bool NebulaConvexPolygonShape2D::contains_point(const Vector2 &p_point) const {
	bool out = false;
	bool in = false;

	for (int i = 0; i < point_count; i++) {
		real_t d = points[i].normal.dot(p_point) - points[i].normal.dot(points[i].pos);
		if (d > 0) {
			out = true;
		} else {
			in = true;
		}
	}

	return in != out;
}

bool NebulaConvexPolygonShape2D::intersect_segment(const Vector2 &p_begin, const Vector2 &p_end, Vector2 &r_point, Vector2 &r_normal) const {
	Vector2 n = (p_end - p_begin).normalized();
	real_t d = 1e10;
	bool inters = false;

	for (int i = 0; i < point_count; i++) {
		Vector2 res;

		if (!Geometry2D::segment_intersects_segment(p_begin, p_end, points[i].pos, points[(i + 1) % point_count].pos, &res)) {
			continue;
		}

		real_t nd = n.dot(res);
		if (nd < d) {
			d = nd;
			r_point = res;
			r_normal = points[i].normal;
			inters = true;
		}
	}

	return inters;
}

real_t NebulaConvexPolygonShape2D::get_moment_of_inertia(real_t p_mass, const Size2 &p_scale) const {
	ERR_FAIL_COND_V_MSG(point_count == 0, 0, "Convex polygon shape has no points.");
	Rect2 aabb_new;
	aabb_new.position = points[0].pos * p_scale;
	for (int i = 0; i < point_count; i++) {
		aabb_new.expand_to(points[i].pos * p_scale);
	}

	return p_mass * aabb_new.size.dot(aabb_new.size) / 12.0;
}

void NebulaConvexPolygonShape2D::set_data(const Variant &p_data) {
#ifdef REAL_T_IS_DOUBLE
	ERR_FAIL_COND(p_data.get_type() != Variant::PACKED_VECTOR2_ARRAY && p_data.get_type() != Variant::PACKED_FLOAT64_ARRAY);
#else
	ERR_FAIL_COND(p_data.get_type() != Variant::PACKED_VECTOR2_ARRAY && p_data.get_type() != Variant::PACKED_FLOAT32_ARRAY);
#endif

	if (points) {
		memdelete_arr(points);
	}
	points = nullptr;
	point_count = 0;

	if (p_data.get_type() == Variant::PACKED_VECTOR2_ARRAY) {
		Vector<Vector2> arr = p_data;
		ERR_FAIL_COND(arr.is_empty());
		point_count = arr.size();
		points = memnew_arr(Point, point_count);
		const Vector2 *r = arr.ptr();

		for (int i = 0; i < point_count; i++) {
			points[i].pos = r[i];
		}

		for (int i = 0; i < point_count; i++) {
			Vector2 p = points[i].pos;
			Vector2 pn = points[(i + 1) % point_count].pos;
			points[i].normal = (pn - p).orthogonal().normalized();
		}
	} else {
		Vector<real_t> dvr = p_data;
		point_count = dvr.size() / 4;
		ERR_FAIL_COND(point_count == 0);

		points = memnew_arr(Point, point_count);
		const real_t *r = dvr.ptr();

		for (int i = 0; i < point_count; i++) {
			int idx = i << 2;
			points[i].pos.x = r[idx + 0];
			points[i].pos.y = r[idx + 1];
			points[i].normal.x = r[idx + 2];
			points[i].normal.y = r[idx + 3];
		}
	}

	ERR_FAIL_COND(point_count == 0);
	Rect2 aabb_new;
	aabb_new.position = points[0].pos;
	for (int i = 1; i < point_count; i++) {
		aabb_new.expand_to(points[i].pos);
	}

	configure(aabb_new);
}

Variant NebulaConvexPolygonShape2D::get_data() const {
	Vector<Vector2> dvr;

	dvr.resize(point_count);

	for (int i = 0; i < point_count; i++) {
		dvr.set(i, points[i].pos);
	}

	return dvr;
}

NebulaConvexPolygonShape2D::~NebulaConvexPolygonShape2D() {
	if (points) {
		memdelete_arr(points);
	}
}

//////////////////////////////////////////////////

void NebulaConcavePolygonShape2D::get_supports(const Vector2 &p_normal, Vector2 *r_supports, int &r_amount) const {
	real_t d = -1e10;
	int idx = -1;
	for (int i = 0; i < points.size(); i++) {
		real_t ld = p_normal.dot(points[i]);
		if (ld > d) {
			d = ld;
			idx = i;
		}
	}

	r_amount = 1;
	ERR_FAIL_COND(idx == -1);
	*r_supports = points[idx];
}

bool NebulaConcavePolygonShape2D::contains_point(const Vector2 &p_point) const {
	return false; //sorry
}

bool NebulaConcavePolygonShape2D::intersect_segment(const Vector2 &p_begin, const Vector2 &p_end, Vector2 &r_point, Vector2 &r_normal) const {
	if (segments.size() == 0 || points.size() == 0) {
		return false;
	}

	uint32_t *stack = (uint32_t *)alloca(sizeof(int) * bvh_depth);

	enum {
		TEST_AABB_BIT = 0,
		VISIT_LEFT_BIT = 1,
		VISIT_RIGHT_BIT = 2,
		VISIT_DONE_BIT = 3,
		VISITED_BIT_SHIFT = 29,
		NODE_IDX_MASK = (1 << VISITED_BIT_SHIFT) - 1,
		VISITED_BIT_MASK = ~NODE_IDX_MASK,

	};

	Vector2 n = (p_end - p_begin).normalized();
	real_t d = 1e10;
	bool inters = false;

	/*
	for(int i=0;i<bvh_depth;i++)
		stack[i]=0;
	*/

	int level = 0;

	const Segment *segmentptr = &segments[0];
	const Vector2 *pointptr = &points[0];
	const BVH *bvhptr = &bvh[0];

	stack[0] = 0;
	while (true) {
		uint32_t node = stack[level] & NODE_IDX_MASK;
		const BVH &bvh2 = bvhptr[node];
		bool done = false;

		switch (stack[level] >> VISITED_BIT_SHIFT) {
			case TEST_AABB_BIT: {
				bool valid = bvh2.aabb.intersects_segment(p_begin, p_end);
				if (!valid) {
					stack[level] = (VISIT_DONE_BIT << VISITED_BIT_SHIFT) | node;

				} else {
					if (bvh2.left < 0) {
						const Segment &s = segmentptr[bvh2.right];
						Vector2 a = pointptr[s.points[0]];
						Vector2 b = pointptr[s.points[1]];

						Vector2 res;

						if (Geometry2D::segment_intersects_segment(p_begin, p_end, a, b, &res)) {
							real_t nd = n.dot(res);
							if (nd < d) {
								d = nd;
								r_point = res;
								r_normal = (b - a).orthogonal().normalized();
								inters = true;
							}
						}

						stack[level] = (VISIT_DONE_BIT << VISITED_BIT_SHIFT) | node;

					} else {
						stack[level] = (VISIT_LEFT_BIT << VISITED_BIT_SHIFT) | node;
					}
				}
			}
				continue;
			case VISIT_LEFT_BIT: {
				stack[level] = (VISIT_RIGHT_BIT << VISITED_BIT_SHIFT) | node;
				stack[level + 1] = bvh2.left | TEST_AABB_BIT;
				level++;
			}
				continue;
			case VISIT_RIGHT_BIT: {
				stack[level] = (VISIT_DONE_BIT << VISITED_BIT_SHIFT) | node;
				stack[level + 1] = bvh2.right | TEST_AABB_BIT;
				level++;
			}
				continue;
			case VISIT_DONE_BIT: {
				if (level == 0) {
					done = true;
					break;
				} else {
					level--;
				}
			}
				continue;
		}

		if (done) {
			break;
		}
	}

	if (inters) {
		if (n.dot(r_normal) > 0) {
			r_normal = -r_normal;
		}
	}

	return inters;
}

int NebulaConcavePolygonShape2D::_generate_bvh(BVH *p_bvh, int p_len, int p_depth) {
	if (p_len == 1) {
		bvh_depth = MAX(p_depth, bvh_depth);
		bvh.push_back(*p_bvh);
		return bvh.size() - 1;
	}

	//else sort best

	Rect2 global_aabb = p_bvh[0].aabb;
	for (int i = 1; i < p_len; i++) {
		global_aabb = global_aabb.merge(p_bvh[i].aabb);
	}

	if (global_aabb.size.x > global_aabb.size.y) {
		SortArray<BVH, BVH_CompareX> sort;
		sort.sort(p_bvh, p_len);

	} else {
		SortArray<BVH, BVH_CompareY> sort;
		sort.sort(p_bvh, p_len);
	}

	int median = p_len / 2;

	BVH node;
	node.aabb = global_aabb;
	int node_idx = bvh.size();
	bvh.push_back(node);

	int l = _generate_bvh(p_bvh, median, p_depth + 1);
	int r = _generate_bvh(&p_bvh[median], p_len - median, p_depth + 1);
	bvh.write[node_idx].left = l;
	bvh.write[node_idx].right = r;

	return node_idx;
}

void NebulaConcavePolygonShape2D::set_data(const Variant &p_data) {
#ifdef REAL_T_IS_DOUBLE
	ERR_FAIL_COND(p_data.get_type() != Variant::PACKED_VECTOR2_ARRAY && p_data.get_type() != Variant::PACKED_FLOAT64_ARRAY);
#else
	ERR_FAIL_COND(p_data.get_type() != Variant::PACKED_VECTOR2_ARRAY && p_data.get_type() != Variant::PACKED_FLOAT32_ARRAY);
#endif

	Rect2 aabb_new;

	if (p_data.get_type() == Variant::PACKED_VECTOR2_ARRAY) {
		Vector<Vector2> p2arr = p_data;
		int len = p2arr.size();
		ERR_FAIL_COND(len % 2);

		segments.clear();
		points.clear();
		bvh.clear();
		bvh_depth = 1;

		if (len == 0) {
			configure(aabb_new);
			return;
		}

		const Vector2 *arr = p2arr.ptr();

		HashMap<Point2, int> pointmap;
		for (int i = 0; i < len; i += 2) {
			Point2 p1 = arr[i];
			Point2 p2 = arr[i + 1];
			int idx_p1, idx_p2;

			if (pointmap.has(p1)) {
				idx_p1 = pointmap[p1];
			} else {
				idx_p1 = pointmap.size();
				pointmap[p1] = idx_p1;
			}

			if (pointmap.has(p2)) {
				idx_p2 = pointmap[p2];
			} else {
				idx_p2 = pointmap.size();
				pointmap[p2] = idx_p2;
			}

			Segment s;
			s.points[0] = idx_p1;
			s.points[1] = idx_p2;
			segments.push_back(s);
		}

		points.resize(pointmap.size());
		aabb_new.position = pointmap.begin()->key;
		for (const KeyValue<Point2, int> &E : pointmap) {
			aabb_new.expand_to(E.key);
			points.write[E.value] = E.key;
		}

		Vector<BVH> main_vbh;
		main_vbh.resize(segments.size());
		for (int i = 0; i < main_vbh.size(); i++) {
			main_vbh.write[i].aabb.position = points[segments[i].points[0]];
			main_vbh.write[i].aabb.expand_to(points[segments[i].points[1]]);
			main_vbh.write[i].left = -1;
			main_vbh.write[i].right = i;
		}

		_generate_bvh(main_vbh.ptrw(), main_vbh.size(), 1);

	} else {
		//dictionary with arrays
	}

	configure(aabb_new);
}

Variant NebulaConcavePolygonShape2D::get_data() const {
	Vector<Vector2> rsegments;
	int len = segments.size();
	rsegments.resize(len * 2);
	Vector2 *w = rsegments.ptrw();
	for (int i = 0; i < len; i++) {
		w[(i << 1) + 0] = points[segments[i].points[0]];
		w[(i << 1) + 1] = points[segments[i].points[1]];
	}

	return rsegments;
}

void NebulaConcavePolygonShape2D::cull(const Rect2 &p_local_aabb, QueryCallback p_callback, void *p_userdata) const {
	uint32_t *stack = (uint32_t *)alloca(sizeof(int) * bvh_depth);

	enum {
		TEST_AABB_BIT = 0,
		VISIT_LEFT_BIT = 1,
		VISIT_RIGHT_BIT = 2,
		VISIT_DONE_BIT = 3,
		VISITED_BIT_SHIFT = 29,
		NODE_IDX_MASK = (1 << VISITED_BIT_SHIFT) - 1,
		VISITED_BIT_MASK = ~NODE_IDX_MASK,

	};

	/*
	for(int i=0;i<bvh_depth;i++)
		stack[i]=0;
	*/

	if (segments.size() == 0 || points.size() == 0 || bvh.size() == 0) {
		return;
	}

	int level = 0;

	const Segment *segmentptr = &segments[0];
	const Vector2 *pointptr = &points[0];
	const BVH *bvhptr = &bvh[0];

	stack[0] = 0;
	while (true) {
		uint32_t node = stack[level] & NODE_IDX_MASK;
		const BVH &bvh2 = bvhptr[node];

		switch (stack[level] >> VISITED_BIT_SHIFT) {
			case TEST_AABB_BIT: {
				bool valid = p_local_aabb.intersects(bvh2.aabb);
				if (!valid) {
					stack[level] = (VISIT_DONE_BIT << VISITED_BIT_SHIFT) | node;

				} else {
					if (bvh2.left < 0) {
						const Segment &s = segmentptr[bvh2.right];
						Vector2 a = pointptr[s.points[0]];
						Vector2 b = pointptr[s.points[1]];

						NebulaSegmentShape2D ss(a, b, (b - a).orthogonal().normalized());

						if (p_callback(p_userdata, &ss)) {
							return;
						}
						stack[level] = (VISIT_DONE_BIT << VISITED_BIT_SHIFT) | node;

					} else {
						stack[level] = (VISIT_LEFT_BIT << VISITED_BIT_SHIFT) | node;
					}
				}
			}
				continue;
			case VISIT_LEFT_BIT: {
				stack[level] = (VISIT_RIGHT_BIT << VISITED_BIT_SHIFT) | node;
				stack[level + 1] = bvh2.left | TEST_AABB_BIT;
				level++;
			}
				continue;
			case VISIT_RIGHT_BIT: {
				stack[level] = (VISIT_DONE_BIT << VISITED_BIT_SHIFT) | node;
				stack[level + 1] = bvh2.right | TEST_AABB_BIT;
				level++;
			}
				continue;
			case VISIT_DONE_BIT: {
				if (level == 0) {
					return;
				} else {
					level--;
				}
			}
				continue;
		}
	}
}
