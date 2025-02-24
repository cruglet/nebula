/**************************************************************************/
/*  nebula_area_pair_3d.h                                                  */
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

#ifndef NEBULA_AREA_PAIR_3D_H
#define NEBULA_AREA_PAIR_3D_H

#include "nebula_area_3d.h"
#include "nebula_body_3d.h"
#include "nebula_constraint_3d.h"
#include "nebula_soft_body_3d.h"

class NebulaAreaPair3D : public NebulaConstraint3D {
	NebulaBody3D *body = nullptr;
	NebulaArea3D *area = nullptr;
	int body_shape;
	int area_shape;
	bool colliding = false;
	bool process_collision = false;
	bool has_space_override = false;
	bool body_has_attached_area = false;

public:
	virtual bool setup(real_t p_step) override;
	virtual bool pre_solve(real_t p_step) override;
	virtual void solve(real_t p_step) override;

	NebulaAreaPair3D(NebulaBody3D *p_body, int p_body_shape, NebulaArea3D *p_area, int p_area_shape);
	~NebulaAreaPair3D();
};

class NebulaArea2Pair3D : public NebulaConstraint3D {
	NebulaArea3D *area_a = nullptr;
	NebulaArea3D *area_b = nullptr;
	int shape_a;
	int shape_b;
	bool colliding_a = false;
	bool colliding_b = false;
	bool process_collision_a = false;
	bool process_collision_b = false;
	bool area_a_monitorable;
	bool area_b_monitorable;

public:
	virtual bool setup(real_t p_step) override;
	virtual bool pre_solve(real_t p_step) override;
	virtual void solve(real_t p_step) override;

	NebulaArea2Pair3D(NebulaArea3D *p_area_a, int p_shape_a, NebulaArea3D *p_area_b, int p_shape_b);
	~NebulaArea2Pair3D();
};

class NebulaAreaSoftBodyPair3D : public NebulaConstraint3D {
	NebulaSoftBody3D *soft_body = nullptr;
	NebulaArea3D *area = nullptr;
	int soft_body_shape;
	int area_shape;
	bool colliding = false;
	bool process_collision = false;
	bool has_space_override = false;
	bool body_has_attached_area = false;

public:
	virtual bool setup(real_t p_step) override;
	virtual bool pre_solve(real_t p_step) override;
	virtual void solve(real_t p_step) override;

	NebulaAreaSoftBodyPair3D(NebulaSoftBody3D *p_sof_body, int p_soft_body_shape, NebulaArea3D *p_area, int p_area_shape);
	~NebulaAreaSoftBodyPair3D();
};

#endif // NEBULA_AREA_PAIR_3D_H
