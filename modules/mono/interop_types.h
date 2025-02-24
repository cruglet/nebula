/**************************************************************************/
/*  interop_types.h                                                       */
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

#ifndef INTEROP_TYPES_H
#define INTEROP_TYPES_H

#include "core/math/math_defs.h"

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stdint.h>

// This is taken from the old GDNative, which was removed.

#define NEBULA_VARIANT_SIZE (sizeof(real_t) * 4 + sizeof(int64_t))

typedef struct {
	uint8_t _dont_touch_that[NEBULA_VARIANT_SIZE];
} nebula_variant;

#define NEBULA_ARRAY_SIZE sizeof(void *)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_ARRAY_SIZE];
} nebula_array;

#define NEBULA_DICTIONARY_SIZE sizeof(void *)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_DICTIONARY_SIZE];
} nebula_dictionary;

#define NEBULA_STRING_SIZE sizeof(void *)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_STRING_SIZE];
} nebula_string;

#define NEBULA_STRING_NAME_SIZE sizeof(void *)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_STRING_NAME_SIZE];
} nebula_string_name;

#define NEBULA_PACKED_ARRAY_SIZE (2 * sizeof(void *))

typedef struct {
	uint8_t _dont_touch_that[NEBULA_PACKED_ARRAY_SIZE];
} nebula_packed_array;

#define NEBULA_VECTOR2_SIZE (sizeof(real_t) * 2)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_VECTOR2_SIZE];
} nebula_vector2;

#define NEBULA_VECTOR2I_SIZE (sizeof(int32_t) * 2)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_VECTOR2I_SIZE];
} nebula_vector2i;

#define NEBULA_RECT2_SIZE (sizeof(real_t) * 4)

typedef struct nebula_rect2 {
	uint8_t _dont_touch_that[NEBULA_RECT2_SIZE];
} nebula_rect2;

#define NEBULA_RECT2I_SIZE (sizeof(int32_t) * 4)

typedef struct nebula_rect2i {
	uint8_t _dont_touch_that[NEBULA_RECT2I_SIZE];
} nebula_rect2i;

#define NEBULA_VECTOR3_SIZE (sizeof(real_t) * 3)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_VECTOR3_SIZE];
} nebula_vector3;

#define NEBULA_VECTOR3I_SIZE (sizeof(int32_t) * 3)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_VECTOR3I_SIZE];
} nebula_vector3i;

#define NEBULA_TRANSFORM2D_SIZE (sizeof(real_t) * 6)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_TRANSFORM2D_SIZE];
} nebula_transform2d;

#define NEBULA_VECTOR4_SIZE (sizeof(real_t) * 4)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_VECTOR4_SIZE];
} nebula_vector4;

#define NEBULA_VECTOR4I_SIZE (sizeof(int32_t) * 4)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_VECTOR4I_SIZE];
} nebula_vector4i;

#define NEBULA_PLANE_SIZE (sizeof(real_t) * 4)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_PLANE_SIZE];
} nebula_plane;

#define NEBULA_QUATERNION_SIZE (sizeof(real_t) * 4)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_QUATERNION_SIZE];
} nebula_quaternion;

#define NEBULA_AABB_SIZE (sizeof(real_t) * 6)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_AABB_SIZE];
} nebula_aabb;

#define NEBULA_BASIS_SIZE (sizeof(real_t) * 9)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_BASIS_SIZE];
} nebula_basis;

#define NEBULA_TRANSFORM3D_SIZE (sizeof(real_t) * 12)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_TRANSFORM3D_SIZE];
} nebula_transform3d;

#define NEBULA_PROJECTION_SIZE (sizeof(real_t) * 4 * 4)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_PROJECTION_SIZE];
} nebula_projection;

// Colors should always use 32-bit floats, so don't use real_t here.
#define NEBULA_COLOR_SIZE (sizeof(float) * 4)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_COLOR_SIZE];
} nebula_color;

#define NEBULA_NODE_PATH_SIZE sizeof(void *)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_NODE_PATH_SIZE];
} nebula_node_path;

#define NEBULA_RID_SIZE sizeof(uint64_t)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_RID_SIZE];
} nebula_rid;

// Alignment hardcoded in `core/variant/callable.h`.
#define NEBULA_CALLABLE_SIZE (16)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_CALLABLE_SIZE];
} nebula_callable;

// Alignment hardcoded in `core/variant/callable.h`.
#define NEBULA_SIGNAL_SIZE (16)

typedef struct {
	uint8_t _dont_touch_that[NEBULA_SIGNAL_SIZE];
} nebula_signal;

#ifdef __cplusplus
}
#endif

#endif // INTEROP_TYPES_H
