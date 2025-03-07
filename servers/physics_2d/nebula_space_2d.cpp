/**************************************************************************/
/*  nebula_space_2d.cpp                                                    */
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

#include "nebula_space_2d.h"

#include "nebula_collision_solver_2d.h"
#include "nebula_physics_server_2d.h"

#include "core/os/os.h"
#include "core/templates/pair.h"

#define TEST_MOTION_MARGIN_MIN_VALUE 0.0001
#define TEST_MOTION_MIN_CONTACT_DEPTH_FACTOR 0.05

_FORCE_INLINE_ static bool _can_collide_with(NebulaCollisionObject2D *p_object, uint32_t p_collision_mask, bool p_collide_with_bodies, bool p_collide_with_areas) {
	if (!(p_object->get_collision_layer() & p_collision_mask)) {
		return false;
	}

	if (p_object->get_type() == NebulaCollisionObject2D::TYPE_AREA && !p_collide_with_areas) {
		return false;
	}

	if (p_object->get_type() == NebulaCollisionObject2D::TYPE_BODY && !p_collide_with_bodies) {
		return false;
	}

	return true;
}

int NebulaPhysicsDirectSpaceState2D::intersect_point(const PointParameters &p_parameters, ShapeResult *r_results, int p_result_max) {
	if (p_result_max <= 0) {
		return 0;
	}

	Rect2 aabb;
	aabb.position = p_parameters.position - Vector2(0.00001, 0.00001);
	aabb.size = Vector2(0.00002, 0.00002);

	int amount = space->broadphase->cull_aabb(aabb, space->intersection_query_results, NebulaSpace2D::INTERSECTION_QUERY_MAX, space->intersection_query_subindex_results);

	int cc = 0;

	for (int i = 0; i < amount; i++) {
		if (!_can_collide_with(space->intersection_query_results[i], p_parameters.collision_mask, p_parameters.collide_with_bodies, p_parameters.collide_with_areas)) {
			continue;
		}

		if (p_parameters.exclude.has(space->intersection_query_results[i]->get_self())) {
			continue;
		}

		const NebulaCollisionObject2D *col_obj = space->intersection_query_results[i];

		if (p_parameters.pick_point && !col_obj->is_pickable()) {
			continue;
		}

		if (col_obj->get_canvas_instance_id() != p_parameters.canvas_instance_id) {
			continue;
		}

		int shape_idx = space->intersection_query_subindex_results[i];

		NebulaShape2D *shape = col_obj->get_shape(shape_idx);

		Vector2 local_point = (col_obj->get_transform() * col_obj->get_shape_transform(shape_idx)).affine_inverse().xform(p_parameters.position);

		if (!shape->contains_point(local_point)) {
			continue;
		}

		if (cc >= p_result_max) {
			continue;
		}

		r_results[cc].collider_id = col_obj->get_instance_id();
		if (r_results[cc].collider_id.is_valid()) {
			r_results[cc].collider = ObjectDB::get_instance(r_results[cc].collider_id);
		}
		r_results[cc].rid = col_obj->get_self();
		r_results[cc].shape = shape_idx;

		cc++;
	}

	return cc;
}

bool NebulaPhysicsDirectSpaceState2D::intersect_ray(const RayParameters &p_parameters, RayResult &r_result) {
	ERR_FAIL_COND_V(space->locked, false);

	Vector2 begin, end;
	Vector2 normal;
	begin = p_parameters.from;
	end = p_parameters.to;
	normal = (end - begin).normalized();

	int amount = space->broadphase->cull_segment(begin, end, space->intersection_query_results, NebulaSpace2D::INTERSECTION_QUERY_MAX, space->intersection_query_subindex_results);

	//todo, create another array that references results, compute AABBs and check closest point to ray origin, sort, and stop evaluating results when beyond first collision

	bool collided = false;
	Vector2 res_point, res_normal;
	int res_shape = -1;
	const NebulaCollisionObject2D *res_obj = nullptr;
	real_t min_d = 1e10;

	for (int i = 0; i < amount; i++) {
		if (!_can_collide_with(space->intersection_query_results[i], p_parameters.collision_mask, p_parameters.collide_with_bodies, p_parameters.collide_with_areas)) {
			continue;
		}

		if (p_parameters.exclude.has(space->intersection_query_results[i]->get_self())) {
			continue;
		}

		const NebulaCollisionObject2D *col_obj = space->intersection_query_results[i];

		int shape_idx = space->intersection_query_subindex_results[i];
		Transform2D inv_xform = col_obj->get_shape_inv_transform(shape_idx) * col_obj->get_inv_transform();

		Vector2 local_from = inv_xform.xform(begin);
		Vector2 local_to = inv_xform.xform(end);

		const NebulaShape2D *shape = col_obj->get_shape(shape_idx);

		Vector2 shape_point, shape_normal;

		if (shape->contains_point(local_from)) {
			if (p_parameters.hit_from_inside) {
				// Hit shape at starting point.
				min_d = 0;
				res_point = begin;
				res_normal = Vector2();
				res_shape = shape_idx;
				res_obj = col_obj;
				collided = true;
				break;
			} else {
				// Ignore shape when starting inside.
				continue;
			}
		}

		if (shape->intersect_segment(local_from, local_to, shape_point, shape_normal)) {
			Transform2D xform = col_obj->get_transform() * col_obj->get_shape_transform(shape_idx);
			shape_point = xform.xform(shape_point);

			real_t ld = normal.dot(shape_point);

			if (ld < min_d) {
				min_d = ld;
				res_point = shape_point;
				res_normal = inv_xform.basis_xform_inv(shape_normal).normalized();
				res_shape = shape_idx;
				res_obj = col_obj;
				collided = true;
			}
		}
	}

	if (!collided) {
		return false;
	}
	ERR_FAIL_NULL_V(res_obj, false); // Shouldn't happen but silences warning.

	r_result.collider_id = res_obj->get_instance_id();
	if (r_result.collider_id.is_valid()) {
		r_result.collider = ObjectDB::get_instance(r_result.collider_id);
	}
	r_result.normal = res_normal;
	r_result.position = res_point;
	r_result.rid = res_obj->get_self();
	r_result.shape = res_shape;

	return true;
}

int NebulaPhysicsDirectSpaceState2D::intersect_shape(const ShapeParameters &p_parameters, ShapeResult *r_results, int p_result_max) {
	if (p_result_max <= 0) {
		return 0;
	}

	NebulaShape2D *shape = NebulaPhysicsServer2D::nebula_singleton->shape_owner.get_or_null(p_parameters.shape_rid);
	ERR_FAIL_NULL_V(shape, 0);

	Rect2 aabb = p_parameters.transform.xform(shape->get_aabb());
	aabb = aabb.merge(Rect2(aabb.position + p_parameters.motion, aabb.size)); //motion
	aabb = aabb.grow(p_parameters.margin);

	int amount = space->broadphase->cull_aabb(aabb, space->intersection_query_results, NebulaSpace2D::INTERSECTION_QUERY_MAX, space->intersection_query_subindex_results);

	int cc = 0;

	for (int i = 0; i < amount; i++) {
		if (cc >= p_result_max) {
			break;
		}

		if (!_can_collide_with(space->intersection_query_results[i], p_parameters.collision_mask, p_parameters.collide_with_bodies, p_parameters.collide_with_areas)) {
			continue;
		}

		if (p_parameters.exclude.has(space->intersection_query_results[i]->get_self())) {
			continue;
		}

		const NebulaCollisionObject2D *col_obj = space->intersection_query_results[i];
		int shape_idx = space->intersection_query_subindex_results[i];

		if (!NebulaCollisionSolver2D::solve(shape, p_parameters.transform, p_parameters.motion, col_obj->get_shape(shape_idx), col_obj->get_transform() * col_obj->get_shape_transform(shape_idx), Vector2(), nullptr, nullptr, nullptr, p_parameters.margin)) {
			continue;
		}

		r_results[cc].collider_id = col_obj->get_instance_id();
		if (r_results[cc].collider_id.is_valid()) {
			r_results[cc].collider = ObjectDB::get_instance(r_results[cc].collider_id);
		}
		r_results[cc].rid = col_obj->get_self();
		r_results[cc].shape = shape_idx;

		cc++;
	}

	return cc;
}

bool NebulaPhysicsDirectSpaceState2D::cast_motion(const ShapeParameters &p_parameters, real_t &p_closest_safe, real_t &p_closest_unsafe) {
	NebulaShape2D *shape = NebulaPhysicsServer2D::nebula_singleton->shape_owner.get_or_null(p_parameters.shape_rid);
	ERR_FAIL_NULL_V(shape, false);

	Rect2 aabb = p_parameters.transform.xform(shape->get_aabb());
	aabb = aabb.merge(Rect2(aabb.position + p_parameters.motion, aabb.size)); //motion
	aabb = aabb.grow(p_parameters.margin);

	int amount = space->broadphase->cull_aabb(aabb, space->intersection_query_results, NebulaSpace2D::INTERSECTION_QUERY_MAX, space->intersection_query_subindex_results);

	real_t best_safe = 1;
	real_t best_unsafe = 1;

	for (int i = 0; i < amount; i++) {
		if (!_can_collide_with(space->intersection_query_results[i], p_parameters.collision_mask, p_parameters.collide_with_bodies, p_parameters.collide_with_areas)) {
			continue;
		}

		if (p_parameters.exclude.has(space->intersection_query_results[i]->get_self())) {
			continue; //ignore excluded
		}

		const NebulaCollisionObject2D *col_obj = space->intersection_query_results[i];
		int shape_idx = space->intersection_query_subindex_results[i];

		Transform2D col_obj_xform = col_obj->get_transform() * col_obj->get_shape_transform(shape_idx);
		//test initial overlap, does it collide if going all the way?
		if (!NebulaCollisionSolver2D::solve(shape, p_parameters.transform, p_parameters.motion, col_obj->get_shape(shape_idx), col_obj_xform, Vector2(), nullptr, nullptr, nullptr, p_parameters.margin)) {
			continue;
		}

		//test initial overlap, ignore objects it's inside of.
		if (NebulaCollisionSolver2D::solve(shape, p_parameters.transform, Vector2(), col_obj->get_shape(shape_idx), col_obj_xform, Vector2(), nullptr, nullptr, nullptr, p_parameters.margin)) {
			continue;
		}

		Vector2 mnormal = p_parameters.motion.normalized();

		//just do kinematic solving
		real_t low = 0.0;
		real_t hi = 1.0;
		real_t fraction_coeff = 0.5;
		for (int j = 0; j < 8; j++) { //steps should be customizable..
			real_t fraction = low + (hi - low) * fraction_coeff;

			Vector2 sep = mnormal; //important optimization for this to work fast enough
			bool collided = NebulaCollisionSolver2D::solve(shape, p_parameters.transform, p_parameters.motion * fraction, col_obj->get_shape(shape_idx), col_obj_xform, Vector2(), nullptr, nullptr, &sep, p_parameters.margin);

			if (collided) {
				hi = fraction;
				if ((j == 0) || (low > 0.0)) { // Did it not collide before?
					// When alternating or first iteration, use dichotomy.
					fraction_coeff = 0.5;
				} else {
					// When colliding again, converge faster towards low fraction
					// for more accurate results with long motions that collide near the start.
					fraction_coeff = 0.25;
				}
			} else {
				low = fraction;
				if ((j == 0) || (hi < 1.0)) { // Did it collide before?
					// When alternating or first iteration, use dichotomy.
					fraction_coeff = 0.5;
				} else {
					// When not colliding again, converge faster towards high fraction
					// for more accurate results with long motions that collide near the end.
					fraction_coeff = 0.75;
				}
			}
		}

		if (low < best_safe) {
			best_safe = low;
			best_unsafe = hi;
		}
	}

	p_closest_safe = best_safe;
	p_closest_unsafe = best_unsafe;

	return true;
}

bool NebulaPhysicsDirectSpaceState2D::collide_shape(const ShapeParameters &p_parameters, Vector2 *r_results, int p_result_max, int &r_result_count) {
	if (p_result_max <= 0) {
		return false;
	}

	NebulaShape2D *shape = NebulaPhysicsServer2D::nebula_singleton->shape_owner.get_or_null(p_parameters.shape_rid);
	ERR_FAIL_NULL_V(shape, 0);

	Rect2 aabb = p_parameters.transform.xform(shape->get_aabb());
	aabb = aabb.merge(Rect2(aabb.position + p_parameters.motion, aabb.size)); //motion
	aabb = aabb.grow(p_parameters.margin);

	int amount = space->broadphase->cull_aabb(aabb, space->intersection_query_results, NebulaSpace2D::INTERSECTION_QUERY_MAX, space->intersection_query_subindex_results);

	bool collided = false;
	r_result_count = 0;

	NebulaPhysicsServer2D::CollCbkData cbk;
	cbk.max = p_result_max;
	cbk.amount = 0;
	cbk.passed = 0;
	cbk.ptr = r_results;
	NebulaCollisionSolver2D::CallbackResult cbkres = NebulaPhysicsServer2D::_shape_col_cbk;

	NebulaPhysicsServer2D::CollCbkData *cbkptr = &cbk;

	for (int i = 0; i < amount; i++) {
		if (!_can_collide_with(space->intersection_query_results[i], p_parameters.collision_mask, p_parameters.collide_with_bodies, p_parameters.collide_with_areas)) {
			continue;
		}

		const NebulaCollisionObject2D *col_obj = space->intersection_query_results[i];

		if (p_parameters.exclude.has(col_obj->get_self())) {
			continue;
		}

		int shape_idx = space->intersection_query_subindex_results[i];

		cbk.valid_dir = Vector2();
		cbk.valid_depth = 0;

		if (NebulaCollisionSolver2D::solve(shape, p_parameters.transform, p_parameters.motion, col_obj->get_shape(shape_idx), col_obj->get_transform() * col_obj->get_shape_transform(shape_idx), Vector2(), cbkres, cbkptr, nullptr, p_parameters.margin)) {
			collided = cbk.amount > 0;
		}
	}

	r_result_count = cbk.amount;

	return collided;
}

struct _RestCallbackData2D {
	const NebulaCollisionObject2D *object = nullptr;
	const NebulaCollisionObject2D *best_object = nullptr;
	int local_shape = 0;
	int best_local_shape = 0;
	int shape = 0;
	int best_shape = 0;
	Vector2 best_contact;
	Vector2 best_normal;
	real_t best_len = 0.0;
	Vector2 valid_dir;
	real_t valid_depth = 0.0;
	real_t min_allowed_depth = 0.0;
};

static void _rest_cbk_result(const Vector2 &p_point_A, const Vector2 &p_point_B, void *p_userdata) {
	_RestCallbackData2D *rd = static_cast<_RestCallbackData2D *>(p_userdata);

	Vector2 contact_rel = p_point_B - p_point_A;
	real_t len = contact_rel.length();

	if (len < rd->min_allowed_depth) {
		return;
	}

	if (len <= rd->best_len) {
		return;
	}

	Vector2 normal = contact_rel / len;

	if (rd->valid_dir != Vector2()) {
		if (len > rd->valid_depth) {
			return;
		}

		if (rd->valid_dir.dot(normal) > -CMP_EPSILON) {
			return;
		}
	}

	rd->best_len = len;
	rd->best_contact = p_point_B;
	rd->best_normal = normal;
	rd->best_object = rd->object;
	rd->best_shape = rd->shape;
	rd->best_local_shape = rd->local_shape;
}

bool NebulaPhysicsDirectSpaceState2D::rest_info(const ShapeParameters &p_parameters, ShapeRestInfo *r_info) {
	NebulaShape2D *shape = NebulaPhysicsServer2D::nebula_singleton->shape_owner.get_or_null(p_parameters.shape_rid);
	ERR_FAIL_NULL_V(shape, 0);

	real_t margin = MAX(p_parameters.margin, TEST_MOTION_MARGIN_MIN_VALUE);

	Rect2 aabb = p_parameters.transform.xform(shape->get_aabb());
	aabb = aabb.merge(Rect2(aabb.position + p_parameters.motion, aabb.size)); //motion
	aabb = aabb.grow(margin);

	int amount = space->broadphase->cull_aabb(aabb, space->intersection_query_results, NebulaSpace2D::INTERSECTION_QUERY_MAX, space->intersection_query_subindex_results);

	_RestCallbackData2D rcd;

	// Allowed depth can't be lower than motion length, in order to handle contacts at low speed.
	real_t motion_length = p_parameters.motion.length();
	real_t min_contact_depth = margin * TEST_MOTION_MIN_CONTACT_DEPTH_FACTOR;
	rcd.min_allowed_depth = MIN(motion_length, min_contact_depth);

	for (int i = 0; i < amount; i++) {
		if (!_can_collide_with(space->intersection_query_results[i], p_parameters.collision_mask, p_parameters.collide_with_bodies, p_parameters.collide_with_areas)) {
			continue;
		}

		const NebulaCollisionObject2D *col_obj = space->intersection_query_results[i];

		if (p_parameters.exclude.has(col_obj->get_self())) {
			continue;
		}

		int shape_idx = space->intersection_query_subindex_results[i];

		rcd.valid_dir = Vector2();
		rcd.object = col_obj;
		rcd.shape = shape_idx;
		rcd.local_shape = 0;
		bool sc = NebulaCollisionSolver2D::solve(shape, p_parameters.transform, p_parameters.motion, col_obj->get_shape(shape_idx), col_obj->get_transform() * col_obj->get_shape_transform(shape_idx), Vector2(), _rest_cbk_result, &rcd, nullptr, margin);
		if (!sc) {
			continue;
		}
	}

	if (rcd.best_len == 0 || !rcd.best_object) {
		return false;
	}

	r_info->collider_id = rcd.best_object->get_instance_id();
	r_info->shape = rcd.best_shape;
	r_info->normal = rcd.best_normal;
	r_info->point = rcd.best_contact;
	r_info->rid = rcd.best_object->get_self();
	if (rcd.best_object->get_type() == NebulaCollisionObject2D::TYPE_BODY) {
		const NebulaBody2D *body = static_cast<const NebulaBody2D *>(rcd.best_object);
		Vector2 rel_vec = r_info->point - (body->get_transform().get_origin() + body->get_center_of_mass());
		r_info->linear_velocity = Vector2(-body->get_angular_velocity() * rel_vec.y, body->get_angular_velocity() * rel_vec.x) + body->get_linear_velocity();

	} else {
		r_info->linear_velocity = Vector2();
	}

	return true;
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////

int NebulaSpace2D::_cull_aabb_for_body(NebulaBody2D *p_body, const Rect2 &p_aabb) {
	int amount = broadphase->cull_aabb(p_aabb, intersection_query_results, INTERSECTION_QUERY_MAX, intersection_query_subindex_results);

	for (int i = 0; i < amount; i++) {
		bool keep = true;

		if (intersection_query_results[i] == p_body) {
			keep = false;
		} else if (intersection_query_results[i]->get_type() == NebulaCollisionObject2D::TYPE_AREA) {
			keep = false;
		} else if (!p_body->collides_with(static_cast<NebulaBody2D *>(intersection_query_results[i]))) {
			keep = false;
		} else if (static_cast<NebulaBody2D *>(intersection_query_results[i])->has_exception(p_body->get_self()) || p_body->has_exception(intersection_query_results[i]->get_self())) {
			keep = false;
		}

		if (!keep) {
			if (i < amount - 1) {
				SWAP(intersection_query_results[i], intersection_query_results[amount - 1]);
				SWAP(intersection_query_subindex_results[i], intersection_query_subindex_results[amount - 1]);
			}

			amount--;
			i--;
		}
	}

	return amount;
}

bool NebulaSpace2D::test_body_motion(NebulaBody2D *p_body, const PhysicsServer2D::MotionParameters &p_parameters, PhysicsServer2D::MotionResult *r_result) {
	//give me back regular physics engine logic
	//this is madness
	//and most people using this function will think
	//what it does is simpler than using physics
	//this took about a week to get right..
	//but is it right? who knows at this point..

	if (r_result) {
		r_result->collider_id = ObjectID();
		r_result->collider_shape = 0;
	}

	Rect2 body_aabb;

	bool shapes_found = false;

	for (int i = 0; i < p_body->get_shape_count(); i++) {
		if (p_body->is_shape_disabled(i)) {
			continue;
		}

		if (!shapes_found) {
			body_aabb = p_body->get_shape_aabb(i);
			shapes_found = true;
		} else {
			body_aabb = body_aabb.merge(p_body->get_shape_aabb(i));
		}
	}

	if (!shapes_found) {
		if (r_result) {
			*r_result = PhysicsServer2D::MotionResult();
			r_result->travel = p_parameters.motion;
		}
		return false;
	}

	real_t margin = MAX(p_parameters.margin, TEST_MOTION_MARGIN_MIN_VALUE);

	// Undo the currently transform the physics server is aware of and apply the provided one
	body_aabb = p_parameters.from.xform(p_body->get_inv_transform().xform(body_aabb));
	body_aabb = body_aabb.grow(margin);

	static const int max_excluded_shape_pairs = 32;
	ExcludedShapeSW excluded_shape_pairs[max_excluded_shape_pairs];
	int excluded_shape_pair_count = 0;

	real_t min_contact_depth = margin * TEST_MOTION_MIN_CONTACT_DEPTH_FACTOR;

	real_t motion_length = p_parameters.motion.length();
	Vector2 motion_normal = p_parameters.motion / motion_length;

	Transform2D body_transform = p_parameters.from;

	bool recovered = false;

	{
		//STEP 1, FREE BODY IF STUCK

		const int max_results = 32;
		int recover_attempts = 4;
		Vector2 sr[max_results * 2];
		real_t priorities[max_results];

		do {
			NebulaPhysicsServer2D::CollCbkData cbk;
			cbk.max = max_results;
			cbk.amount = 0;
			cbk.passed = 0;
			cbk.ptr = sr;
			cbk.invalid_by_dir = 0;
			excluded_shape_pair_count = 0; //last step is the one valid

			NebulaPhysicsServer2D::CollCbkData *cbkptr = &cbk;
			NebulaCollisionSolver2D::CallbackResult cbkres = NebulaPhysicsServer2D::_shape_col_cbk;
			int priority_amount = 0;

			bool collided = false;

			int amount = _cull_aabb_for_body(p_body, body_aabb);

			for (int j = 0; j < p_body->get_shape_count(); j++) {
				if (p_body->is_shape_disabled(j)) {
					continue;
				}

				NebulaShape2D *body_shape = p_body->get_shape(j);
				Transform2D body_shape_xform = body_transform * p_body->get_shape_transform(j);

				for (int i = 0; i < amount; i++) {
					const NebulaCollisionObject2D *col_obj = intersection_query_results[i];
					if (p_parameters.exclude_bodies.has(col_obj->get_self())) {
						continue;
					}
					if (p_parameters.exclude_objects.has(col_obj->get_instance_id())) {
						continue;
					}

					int shape_idx = intersection_query_subindex_results[i];

					Transform2D col_obj_shape_xform = col_obj->get_transform() * col_obj->get_shape_transform(shape_idx);

					if (body_shape->allows_one_way_collision() && col_obj->is_shape_set_as_one_way_collision(shape_idx)) {
						cbk.valid_dir = col_obj_shape_xform.columns[1].normalized();

						real_t owc_margin = col_obj->get_shape_one_way_collision_margin(shape_idx);
						cbk.valid_depth = MAX(owc_margin, margin); //user specified, but never less than actual margin or it won't work
						cbk.invalid_by_dir = 0;

						if (col_obj->get_type() == NebulaCollisionObject2D::TYPE_BODY) {
							const NebulaBody2D *b = static_cast<const NebulaBody2D *>(col_obj);
							if (b->get_mode() == PhysicsServer2D::BODY_MODE_KINEMATIC || b->get_mode() == PhysicsServer2D::BODY_MODE_RIGID) {
								//fix for moving platforms (kinematic and dynamic), margin is increased by how much it moved in the given direction
								Vector2 lv = b->get_linear_velocity();
								//compute displacement from linear velocity
								Vector2 motion = lv * last_step;
								real_t motion_len = motion.length();
								motion.normalize();
								cbk.valid_depth += motion_len * MAX(motion.dot(-cbk.valid_dir), 0.0);
							}
						}
					} else {
						cbk.valid_dir = Vector2();
						cbk.valid_depth = 0;
						cbk.invalid_by_dir = 0;
					}

					int current_passed = cbk.passed; //save how many points passed collision
					bool did_collide = false;

					NebulaShape2D *against_shape = col_obj->get_shape(shape_idx);
					if (NebulaCollisionSolver2D::solve(body_shape, body_shape_xform, Vector2(), against_shape, col_obj_shape_xform, Vector2(), cbkres, cbkptr, nullptr, margin)) {
						did_collide = cbk.passed > current_passed; //more passed, so collision actually existed
					}
					while (cbk.amount > priority_amount) {
						priorities[priority_amount] = col_obj->get_collision_priority();
						priority_amount++;
					}

					if (!did_collide && cbk.invalid_by_dir > 0) {
						//this shape must be excluded
						if (excluded_shape_pair_count < max_excluded_shape_pairs) {
							ExcludedShapeSW esp;
							esp.local_shape = body_shape;
							esp.against_object = col_obj;
							esp.against_shape_index = shape_idx;
							excluded_shape_pairs[excluded_shape_pair_count++] = esp;
						}
					}

					if (did_collide) {
						collided = true;
					}
				}
			}

			if (!collided) {
				break;
			}

			real_t inv_total_weight = 0.0;
			for (int i = 0; i < cbk.amount; i++) {
				inv_total_weight += priorities[i];
			}
			inv_total_weight = Math::is_zero_approx(inv_total_weight) ? 1.0 : (real_t)cbk.amount / inv_total_weight;

			recovered = true;

			Vector2 recover_motion;
			for (int i = 0; i < cbk.amount; i++) {
				Vector2 a = sr[i * 2 + 0];
				Vector2 b = sr[i * 2 + 1];

				// Compute plane on b towards a.
				Vector2 n = (a - b).normalized();
				real_t d = n.dot(b);

				// Compute depth on recovered motion.
				real_t depth = n.dot(a + recover_motion) - d;
				if (depth > min_contact_depth + CMP_EPSILON) {
					// Only recover if there is penetration.
					recover_motion -= n * (depth - min_contact_depth) * 0.4 * priorities[i] * inv_total_weight;
				}
			}

			if (recover_motion == Vector2()) {
				collided = false;
				break;
			}

			body_transform.columns[2] += recover_motion;
			body_aabb.position += recover_motion;

			recover_attempts--;

		} while (recover_attempts);
	}

	real_t safe = 1.0;
	real_t unsafe = 1.0;
	int best_shape = -1;

	{
		// STEP 2 ATTEMPT MOTION

		Rect2 motion_aabb = body_aabb;
		motion_aabb.position += p_parameters.motion;
		motion_aabb = motion_aabb.merge(body_aabb);

		int amount = _cull_aabb_for_body(p_body, motion_aabb);

		for (int body_shape_idx = 0; body_shape_idx < p_body->get_shape_count(); body_shape_idx++) {
			if (p_body->is_shape_disabled(body_shape_idx)) {
				continue;
			}

			NebulaShape2D *body_shape = p_body->get_shape(body_shape_idx);

			// Colliding separation rays allows to properly snap to the ground,
			// otherwise it's not needed in regular motion.
			if (!p_parameters.collide_separation_ray && (body_shape->get_type() == PhysicsServer2D::SHAPE_SEPARATION_RAY)) {
				// When slide on slope is on, separation ray shape acts like a regular shape.
				if (!static_cast<NebulaSeparationRayShape2D *>(body_shape)->get_slide_on_slope()) {
					continue;
				}
			}

			Transform2D body_shape_xform = body_transform * p_body->get_shape_transform(body_shape_idx);

			bool stuck = false;

			real_t best_safe = 1;
			real_t best_unsafe = 1;

			for (int i = 0; i < amount; i++) {
				const NebulaCollisionObject2D *col_obj = intersection_query_results[i];
				if (p_parameters.exclude_bodies.has(col_obj->get_self())) {
					continue;
				}
				if (p_parameters.exclude_objects.has(col_obj->get_instance_id())) {
					continue;
				}

				int col_shape_idx = intersection_query_subindex_results[i];
				NebulaShape2D *against_shape = col_obj->get_shape(col_shape_idx);

				bool excluded = false;

				for (int k = 0; k < excluded_shape_pair_count; k++) {
					if (excluded_shape_pairs[k].local_shape == body_shape && excluded_shape_pairs[k].against_object == col_obj && excluded_shape_pairs[k].against_shape_index == col_shape_idx) {
						excluded = true;
						break;
					}
				}

				if (excluded) {
					continue;
				}

				Transform2D col_obj_shape_xform = col_obj->get_transform() * col_obj->get_shape_transform(col_shape_idx);
				//test initial overlap, does it collide if going all the way?
				if (!NebulaCollisionSolver2D::solve(body_shape, body_shape_xform, p_parameters.motion, against_shape, col_obj_shape_xform, Vector2(), nullptr, nullptr, nullptr, 0)) {
					continue;
				}

				//test initial overlap
				if (NebulaCollisionSolver2D::solve(body_shape, body_shape_xform, Vector2(), against_shape, col_obj_shape_xform, Vector2(), nullptr, nullptr, nullptr, 0)) {
					if (body_shape->allows_one_way_collision() && col_obj->is_shape_set_as_one_way_collision(col_shape_idx)) {
						Vector2 direction = col_obj_shape_xform.columns[1].normalized();
						if (motion_normal.dot(direction) < 0) {
							continue;
						}
					}

					stuck = true;
					break;
				}

				//just do kinematic solving
				real_t low = 0.0;
				real_t hi = 1.0;
				real_t fraction_coeff = 0.5;
				for (int k = 0; k < 8; k++) { //steps should be customizable..
					real_t fraction = low + (hi - low) * fraction_coeff;

					Vector2 sep = motion_normal; //important optimization for this to work fast enough
					bool collided = NebulaCollisionSolver2D::solve(body_shape, body_shape_xform, p_parameters.motion * fraction, against_shape, col_obj_shape_xform, Vector2(), nullptr, nullptr, &sep, 0);

					if (collided) {
						hi = fraction;
						if ((k == 0) || (low > 0.0)) { // Did it not collide before?
							// When alternating or first iteration, use dichotomy.
							fraction_coeff = 0.5;
						} else {
							// When colliding again, converge faster towards low fraction
							// for more accurate results with long motions that collide near the start.
							fraction_coeff = 0.25;
						}
					} else {
						low = fraction;
						if ((k == 0) || (hi < 1.0)) { // Did it collide before?
							// When alternating or first iteration, use dichotomy.
							fraction_coeff = 0.5;
						} else {
							// When not colliding again, converge faster towards high fraction
							// for more accurate results with long motions that collide near the end.
							fraction_coeff = 0.75;
						}
					}
				}

				if (body_shape->allows_one_way_collision() && col_obj->is_shape_set_as_one_way_collision(col_shape_idx)) {
					Vector2 cd[2];
					NebulaPhysicsServer2D::CollCbkData cbk;
					cbk.max = 1;
					cbk.amount = 0;
					cbk.passed = 0;
					cbk.ptr = cd;
					cbk.valid_dir = col_obj_shape_xform.columns[1].normalized();

					cbk.valid_depth = 10e20;

					Vector2 sep = motion_normal; //important optimization for this to work fast enough
					bool collided = NebulaCollisionSolver2D::solve(body_shape, body_shape_xform, p_parameters.motion * (hi + contact_max_allowed_penetration), col_obj->get_shape(col_shape_idx), col_obj_shape_xform, Vector2(), NebulaPhysicsServer2D::_shape_col_cbk, &cbk, &sep, 0);
					if (!collided || cbk.amount == 0) {
						continue;
					}
				}

				if (low < best_safe) {
					best_safe = low;
					best_unsafe = hi;
				}
			}

			if (stuck) {
				safe = 0;
				unsafe = 0;
				best_shape = body_shape_idx; //sadly it's the best
				break;
			}
			if (best_safe == 1.0) {
				continue;
			}
			if (best_safe < safe) {
				safe = best_safe;
				unsafe = best_unsafe;
				best_shape = body_shape_idx;
			}
		}
	}

	bool collided = false;

	if ((p_parameters.recovery_as_collision && recovered) || (safe < 1)) {
		if (safe >= 1) {
			best_shape = -1; //no best shape with cast, reset to -1
		}

		//it collided, let's get the rest info in unsafe advance
		Transform2D ugt = body_transform;
		ugt.columns[2] += p_parameters.motion * unsafe;

		_RestCallbackData2D rcd;

		// Allowed depth can't be lower than motion length, in order to handle contacts at low speed.
		rcd.min_allowed_depth = MIN(motion_length, min_contact_depth);

		body_aabb.position += p_parameters.motion * unsafe;
		int amount = _cull_aabb_for_body(p_body, body_aabb);

		int from_shape = best_shape != -1 ? best_shape : 0;
		int to_shape = best_shape != -1 ? best_shape + 1 : p_body->get_shape_count();

		for (int j = from_shape; j < to_shape; j++) {
			if (p_body->is_shape_disabled(j)) {
				continue;
			}

			Transform2D body_shape_xform = ugt * p_body->get_shape_transform(j);
			NebulaShape2D *body_shape = p_body->get_shape(j);

			for (int i = 0; i < amount; i++) {
				const NebulaCollisionObject2D *col_obj = intersection_query_results[i];
				if (p_parameters.exclude_bodies.has(col_obj->get_self())) {
					continue;
				}
				if (p_parameters.exclude_objects.has(col_obj->get_instance_id())) {
					continue;
				}

				int shape_idx = intersection_query_subindex_results[i];

				NebulaShape2D *against_shape = col_obj->get_shape(shape_idx);

				bool excluded = false;
				for (int k = 0; k < excluded_shape_pair_count; k++) {
					if (excluded_shape_pairs[k].local_shape == body_shape && excluded_shape_pairs[k].against_object == col_obj && excluded_shape_pairs[k].against_shape_index == shape_idx) {
						excluded = true;
						break;
					}
				}
				if (excluded) {
					continue;
				}

				Transform2D col_obj_shape_xform = col_obj->get_transform() * col_obj->get_shape_transform(shape_idx);

				if (body_shape->allows_one_way_collision() && col_obj->is_shape_set_as_one_way_collision(shape_idx)) {
					rcd.valid_dir = col_obj_shape_xform.columns[1].normalized();

					real_t owc_margin = col_obj->get_shape_one_way_collision_margin(shape_idx);
					rcd.valid_depth = MAX(owc_margin, margin); //user specified, but never less than actual margin or it won't work

					if (col_obj->get_type() == NebulaCollisionObject2D::TYPE_BODY) {
						const NebulaBody2D *b = static_cast<const NebulaBody2D *>(col_obj);
						if (b->get_mode() == PhysicsServer2D::BODY_MODE_KINEMATIC || b->get_mode() == PhysicsServer2D::BODY_MODE_RIGID) {
							//fix for moving platforms (kinematic and dynamic), margin is increased by how much it moved in the given direction
							Vector2 lv = b->get_linear_velocity();
							//compute displacement from linear velocity
							Vector2 motion = lv * last_step;
							real_t motion_len = motion.length();
							motion.normalize();
							rcd.valid_depth += motion_len * MAX(motion.dot(-rcd.valid_dir), 0.0);
						}
					}
				} else {
					rcd.valid_dir = Vector2();
					rcd.valid_depth = 0;
				}

				rcd.object = col_obj;
				rcd.shape = shape_idx;
				rcd.local_shape = j;
				bool sc = NebulaCollisionSolver2D::solve(body_shape, body_shape_xform, Vector2(), against_shape, col_obj_shape_xform, Vector2(), _rest_cbk_result, &rcd, nullptr, margin);
				if (!sc) {
					continue;
				}
			}
		}

		if (rcd.best_len != 0) {
			if (r_result) {
				r_result->collider = rcd.best_object->get_self();
				r_result->collider_id = rcd.best_object->get_instance_id();
				r_result->collider_shape = rcd.best_shape;
				r_result->collision_local_shape = rcd.best_local_shape;
				r_result->collision_normal = rcd.best_normal;
				r_result->collision_point = rcd.best_contact;
				r_result->collision_depth = rcd.best_len;
				r_result->collision_safe_fraction = safe;
				r_result->collision_unsafe_fraction = unsafe;

				const NebulaBody2D *body = static_cast<const NebulaBody2D *>(rcd.best_object);
				Vector2 rel_vec = r_result->collision_point - (body->get_transform().get_origin() + body->get_center_of_mass());
				r_result->collider_velocity = Vector2(-body->get_angular_velocity() * rel_vec.y, body->get_angular_velocity() * rel_vec.x) + body->get_linear_velocity();

				r_result->travel = safe * p_parameters.motion;
				r_result->remainder = p_parameters.motion - safe * p_parameters.motion;
				r_result->travel += (body_transform.get_origin() - p_parameters.from.get_origin());
			}

			collided = true;
		}
	}

	if (!collided && r_result) {
		r_result->travel = p_parameters.motion;
		r_result->remainder = Vector2();
		r_result->travel += (body_transform.get_origin() - p_parameters.from.get_origin());
	}

	return collided;
}

// Assumes a valid collision pair, this should have been checked beforehand in the BVH or octree.
void *NebulaSpace2D::_broadphase_pair(NebulaCollisionObject2D *A, int p_subindex_A, NebulaCollisionObject2D *B, int p_subindex_B, void *p_self) {
	NebulaCollisionObject2D::Type type_A = A->get_type();
	NebulaCollisionObject2D::Type type_B = B->get_type();
	if (type_A > type_B) {
		SWAP(A, B);
		SWAP(p_subindex_A, p_subindex_B);
		SWAP(type_A, type_B);
	}

	NebulaSpace2D *self = static_cast<NebulaSpace2D *>(p_self);
	self->collision_pairs++;

	if (type_A == NebulaCollisionObject2D::TYPE_AREA) {
		NebulaArea2D *area = static_cast<NebulaArea2D *>(A);
		if (type_B == NebulaCollisionObject2D::TYPE_AREA) {
			NebulaArea2D *area_b = static_cast<NebulaArea2D *>(B);
			NebulaArea2Pair2D *area2_pair = memnew(NebulaArea2Pair2D(area_b, p_subindex_B, area, p_subindex_A));
			return area2_pair;
		} else {
			NebulaBody2D *body = static_cast<NebulaBody2D *>(B);
			NebulaAreaPair2D *area_pair = memnew(NebulaAreaPair2D(body, p_subindex_B, area, p_subindex_A));
			return area_pair;
		}

	} else {
		NebulaBodyPair2D *b = memnew(NebulaBodyPair2D(static_cast<NebulaBody2D *>(A), p_subindex_A, static_cast<NebulaBody2D *>(B), p_subindex_B));
		return b;
	}
}

void NebulaSpace2D::_broadphase_unpair(NebulaCollisionObject2D *A, int p_subindex_A, NebulaCollisionObject2D *B, int p_subindex_B, void *p_data, void *p_self) {
	if (!p_data) {
		return;
	}

	NebulaSpace2D *self = static_cast<NebulaSpace2D *>(p_self);
	self->collision_pairs--;
	NebulaConstraint2D *c = static_cast<NebulaConstraint2D *>(p_data);
	memdelete(c);
}

const SelfList<NebulaBody2D>::List &NebulaSpace2D::get_active_body_list() const {
	return active_list;
}

void NebulaSpace2D::body_add_to_active_list(SelfList<NebulaBody2D> *p_body) {
	active_list.add(p_body);
}

void NebulaSpace2D::body_remove_from_active_list(SelfList<NebulaBody2D> *p_body) {
	active_list.remove(p_body);
}

void NebulaSpace2D::body_add_to_mass_properties_update_list(SelfList<NebulaBody2D> *p_body) {
	mass_properties_update_list.add(p_body);
}

void NebulaSpace2D::body_remove_from_mass_properties_update_list(SelfList<NebulaBody2D> *p_body) {
	mass_properties_update_list.remove(p_body);
}

NebulaBroadPhase2D *NebulaSpace2D::get_broadphase() {
	return broadphase;
}

void NebulaSpace2D::add_object(NebulaCollisionObject2D *p_object) {
	ERR_FAIL_COND(objects.has(p_object));
	objects.insert(p_object);
}

void NebulaSpace2D::remove_object(NebulaCollisionObject2D *p_object) {
	ERR_FAIL_COND(!objects.has(p_object));
	objects.erase(p_object);
}

const HashSet<NebulaCollisionObject2D *> &NebulaSpace2D::get_objects() const {
	return objects;
}

void NebulaSpace2D::body_add_to_state_query_list(SelfList<NebulaBody2D> *p_body) {
	state_query_list.add(p_body);
}

void NebulaSpace2D::body_remove_from_state_query_list(SelfList<NebulaBody2D> *p_body) {
	state_query_list.remove(p_body);
}

void NebulaSpace2D::area_add_to_monitor_query_list(SelfList<NebulaArea2D> *p_area) {
	monitor_query_list.add(p_area);
}

void NebulaSpace2D::area_remove_from_monitor_query_list(SelfList<NebulaArea2D> *p_area) {
	monitor_query_list.remove(p_area);
}

void NebulaSpace2D::area_add_to_moved_list(SelfList<NebulaArea2D> *p_area) {
	area_moved_list.add(p_area);
}

void NebulaSpace2D::area_remove_from_moved_list(SelfList<NebulaArea2D> *p_area) {
	area_moved_list.remove(p_area);
}

const SelfList<NebulaArea2D>::List &NebulaSpace2D::get_moved_area_list() const {
	return area_moved_list;
}

void NebulaSpace2D::call_queries() {
	while (state_query_list.first()) {
		NebulaBody2D *b = state_query_list.first()->self();
		state_query_list.remove(state_query_list.first());
		b->call_queries();
	}

	while (monitor_query_list.first()) {
		NebulaArea2D *a = monitor_query_list.first()->self();
		monitor_query_list.remove(monitor_query_list.first());
		a->call_queries();
	}
}

void NebulaSpace2D::setup() {
	contact_debug_count = 0;

	while (mass_properties_update_list.first()) {
		mass_properties_update_list.first()->self()->update_mass_properties();
		mass_properties_update_list.remove(mass_properties_update_list.first());
	}
}

void NebulaSpace2D::update() {
	broadphase->update();
}

void NebulaSpace2D::set_param(PhysicsServer2D::SpaceParameter p_param, real_t p_value) {
	switch (p_param) {
		case PhysicsServer2D::SPACE_PARAM_CONTACT_RECYCLE_RADIUS:
			contact_recycle_radius = p_value;
			break;
		case PhysicsServer2D::SPACE_PARAM_CONTACT_MAX_SEPARATION:
			contact_max_separation = p_value;
			break;
		case PhysicsServer2D::SPACE_PARAM_CONTACT_MAX_ALLOWED_PENETRATION:
			contact_max_allowed_penetration = p_value;
			break;
		case PhysicsServer2D::SPACE_PARAM_CONTACT_DEFAULT_BIAS:
			contact_bias = p_value;
			break;
		case PhysicsServer2D::SPACE_PARAM_BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD:
			body_linear_velocity_sleep_threshold = p_value;
			break;
		case PhysicsServer2D::SPACE_PARAM_BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD:
			body_angular_velocity_sleep_threshold = p_value;
			break;
		case PhysicsServer2D::SPACE_PARAM_BODY_TIME_TO_SLEEP:
			body_time_to_sleep = p_value;
			break;
		case PhysicsServer2D::SPACE_PARAM_CONSTRAINT_DEFAULT_BIAS:
			constraint_bias = p_value;
			break;
		case PhysicsServer2D::SPACE_PARAM_SOLVER_ITERATIONS:
			solver_iterations = p_value;
			break;
	}
}

real_t NebulaSpace2D::get_param(PhysicsServer2D::SpaceParameter p_param) const {
	switch (p_param) {
		case PhysicsServer2D::SPACE_PARAM_CONTACT_RECYCLE_RADIUS:
			return contact_recycle_radius;
		case PhysicsServer2D::SPACE_PARAM_CONTACT_MAX_SEPARATION:
			return contact_max_separation;
		case PhysicsServer2D::SPACE_PARAM_CONTACT_MAX_ALLOWED_PENETRATION:
			return contact_max_allowed_penetration;
		case PhysicsServer2D::SPACE_PARAM_CONTACT_DEFAULT_BIAS:
			return contact_bias;
		case PhysicsServer2D::SPACE_PARAM_BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD:
			return body_linear_velocity_sleep_threshold;
		case PhysicsServer2D::SPACE_PARAM_BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD:
			return body_angular_velocity_sleep_threshold;
		case PhysicsServer2D::SPACE_PARAM_BODY_TIME_TO_SLEEP:
			return body_time_to_sleep;
		case PhysicsServer2D::SPACE_PARAM_CONSTRAINT_DEFAULT_BIAS:
			return constraint_bias;
		case PhysicsServer2D::SPACE_PARAM_SOLVER_ITERATIONS:
			return solver_iterations;
	}
	return 0;
}

void NebulaSpace2D::lock() {
	locked = true;
}

void NebulaSpace2D::unlock() {
	locked = false;
}

bool NebulaSpace2D::is_locked() const {
	return locked;
}

NebulaPhysicsDirectSpaceState2D *NebulaSpace2D::get_direct_state() {
	return direct_access;
}

NebulaSpace2D::NebulaSpace2D() {
	body_linear_velocity_sleep_threshold = GLOBAL_GET("physics/2d/sleep_threshold_linear");
	body_angular_velocity_sleep_threshold = GLOBAL_GET("physics/2d/sleep_threshold_angular");
	body_time_to_sleep = GLOBAL_GET("physics/2d/time_before_sleep");
	solver_iterations = GLOBAL_GET("physics/2d/solver/solver_iterations");
	contact_recycle_radius = GLOBAL_GET("physics/2d/solver/contact_recycle_radius");
	contact_max_separation = GLOBAL_GET("physics/2d/solver/contact_max_separation");
	contact_max_allowed_penetration = GLOBAL_GET("physics/2d/solver/contact_max_allowed_penetration");
	contact_bias = GLOBAL_GET("physics/2d/solver/default_contact_bias");
	constraint_bias = GLOBAL_GET("physics/2d/solver/default_constraint_bias");

	broadphase = NebulaBroadPhase2D::create_func();
	broadphase->set_pair_callback(_broadphase_pair, this);
	broadphase->set_unpair_callback(_broadphase_unpair, this);

	direct_access = memnew(NebulaPhysicsDirectSpaceState2D);
	direct_access->space = this;
}

NebulaSpace2D::~NebulaSpace2D() {
	memdelete(broadphase);
	memdelete(direct_access);
}
