/**************************************************************************/
/*  nebula_collision_solver_2d_sat.h                                       */
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

#ifndef NEBULA_COLLISION_SOLVER_2D_SAT_H
#define NEBULA_COLLISION_SOLVER_2D_SAT_H

#include "nebula_collision_solver_2d.h"

bool sat_2d_calculate_penetration(const NebulaShape2D *p_shape_A, const Transform2D &p_transform_A, const Vector2 &p_motion_A, const NebulaShape2D *p_shape_B, const Transform2D &p_transform_B, const Vector2 &p_motion_B, NebulaCollisionSolver2D::CallbackResult p_result_callback, void *p_userdata, bool p_swap = false, Vector2 *sep_axis = nullptr, real_t p_margin_A = 0, real_t p_margin_B = 0);

#endif // NEBULA_COLLISION_SOLVER_2D_SAT_H
