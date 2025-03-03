/**************************************************************************/
/*  nebula_step_3d.h                                                       */
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

#ifndef NEBULA_STEP_3D_H
#define NEBULA_STEP_3D_H

#include "nebula_space_3d.h"

#include "core/templates/local_vector.h"

class NebulaStep3D {
	uint64_t _step = 1;

	int iterations = 0;
	real_t delta = 0.0;

	LocalVector<LocalVector<NebulaBody3D *>> body_islands;
	LocalVector<LocalVector<NebulaConstraint3D *>> constraint_islands;
	LocalVector<NebulaConstraint3D *> all_constraints;

	void _populate_island(NebulaBody3D *p_body, LocalVector<NebulaBody3D *> &p_body_island, LocalVector<NebulaConstraint3D *> &p_constraint_island);
	void _populate_island_soft_body(NebulaSoftBody3D *p_soft_body, LocalVector<NebulaBody3D *> &p_body_island, LocalVector<NebulaConstraint3D *> &p_constraint_island);
	void _setup_constraint(uint32_t p_constraint_index, void *p_userdata = nullptr);
	void _pre_solve_island(LocalVector<NebulaConstraint3D *> &p_constraint_island) const;
	void _solve_island(uint32_t p_island_index, void *p_userdata = nullptr);
	void _check_suspend(const LocalVector<NebulaBody3D *> &p_body_island) const;

public:
	void step(NebulaSpace3D *p_space, real_t p_delta);
	NebulaStep3D();
	~NebulaStep3D();
};

#endif // NEBULA_STEP_3D_H
