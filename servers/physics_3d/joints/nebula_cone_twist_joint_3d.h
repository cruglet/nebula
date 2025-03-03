/**************************************************************************/
/*  nebula_cone_twist_joint_3d.h                                           */
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

#ifndef NEBULA_CONE_TWIST_JOINT_3D_H
#define NEBULA_CONE_TWIST_JOINT_3D_H

/*
Adapted to Nebula from the Bullet library.
*/

/*
Bullet Continuous Collision Detection and Physics Library
NebulaConeTwistJoint3D is Copyright (c) 2007 Starbreeze Studios

This software is provided 'as-is', without any express or implied warranty.
In no event will the authors be held liable for any damages arising from the use of this software.
Permission is granted to anyone to use this software for any purpose,
including commercial applications, and to alter it and redistribute it freely,
subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.
2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.
3. This notice may not be removed or altered from any source distribution.

Written by: Marcus Hennix
*/

#include "servers/physics_3d/nebula_joint_3d.h"
#include "servers/physics_3d/joints/nebula_jacobian_entry_3d.h"

// NebulaConeTwistJoint3D can be used to simulate ragdoll joints (upper arm, leg etc).
class NebulaConeTwistJoint3D : public NebulaJoint3D {
#ifdef IN_PARALLELL_SOLVER
public:
#endif

	union {
		struct {
			NebulaBody3D *A;
			NebulaBody3D *B;
		};

		NebulaBody3D *_arr[2] = { nullptr, nullptr };
	};

	NebulaJacobianEntry3D m_jac[3] = {}; //3 orthogonal linear constraints

	real_t m_appliedImpulse = 0.0;
	Transform3D m_rbAFrame;
	Transform3D m_rbBFrame;

	real_t m_limitSoftness = 0.0;
	real_t m_biasFactor = 0.3;
	real_t m_relaxationFactor = 1.0;

	real_t m_swingSpan1 = Math_TAU / 8.0;
	real_t m_swingSpan2 = 0.0;
	real_t m_twistSpan = 0.0;

	Vector3 m_swingAxis;
	Vector3 m_twistAxis;

	real_t m_kSwing = 0.0;
	real_t m_kTwist = 0.0;

	real_t m_twistLimitSign = 0.0;
	real_t m_swingCorrection = 0.0;
	real_t m_twistCorrection = 0.0;

	real_t m_accSwingLimitImpulse = 0.0;
	real_t m_accTwistLimitImpulse = 0.0;

	bool m_angularOnly = false;
	bool m_solveTwistLimit = false;
	bool m_solveSwingLimit = false;

public:
	virtual PhysicsServer3D::JointType get_type() const override { return PhysicsServer3D::JOINT_TYPE_CONE_TWIST; }

	virtual bool setup(real_t p_step) override;
	virtual void solve(real_t p_step) override;

	NebulaConeTwistJoint3D(NebulaBody3D *rbA, NebulaBody3D *rbB, const Transform3D &rbAFrame, const Transform3D &rbBFrame);

	void setAngularOnly(bool angularOnly) {
		m_angularOnly = angularOnly;
	}

	void setLimit(real_t _swingSpan1, real_t _swingSpan2, real_t _twistSpan, real_t _softness = 0.8f, real_t _biasFactor = 0.3f, real_t _relaxationFactor = 1.0f) {
		m_swingSpan1 = _swingSpan1;
		m_swingSpan2 = _swingSpan2;
		m_twistSpan = _twistSpan;

		m_limitSoftness = _softness;
		m_biasFactor = _biasFactor;
		m_relaxationFactor = _relaxationFactor;
	}

	inline int getSolveTwistLimit() {
		return m_solveTwistLimit;
	}

	inline int getSolveSwingLimit() {
		return m_solveTwistLimit;
	}

	inline real_t getTwistLimitSign() {
		return m_twistLimitSign;
	}

	void set_param(PhysicsServer3D::ConeTwistJointParam p_param, real_t p_value);
	real_t get_param(PhysicsServer3D::ConeTwistJointParam p_param) const;
};

#endif // NEBULA_CONE_TWIST_JOINT_3D_H
