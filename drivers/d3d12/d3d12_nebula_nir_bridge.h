/**************************************************************************/
/*  d3d12_nebula_nir_bridge.h                                              */
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

#ifndef D3D12_NEBULA_NIR_BRIDGE_H
#define D3D12_NEBULA_NIR_BRIDGE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// This one leaves room for potentially extremely copious bindings in a set.
static const uint32_t NEBULA_NIR_DESCRIPTOR_SET_MULTIPLIER = 100000000;
// This one leaves room for potentially big sized arrays.
static const uint32_t NEBULA_NIR_BINDING_MULTIPLIER = 100000;

static const uint64_t NEBULA_NIR_SC_SENTINEL_MAGIC = 0x45678900; // This must be as big as to be VBR-ed as a 32 bits number.
static const uint64_t NEBULA_NIR_SC_SENTINEL_MAGIC_MASK = 0xffffffffffffff00;
static const uint64_t NEBULA_NIR_SC_SENTINEL_ID_MASK = 0x00000000000000ff;

typedef struct NebulaNirCallbacks {
	void *data;
	void (*report_resource)(uint32_t p_register, uint32_t p_space, uint32_t p_dxil_type, void *p_data);
	void (*report_sc_bit_offset_fn)(uint32_t p_sc_id, uint64_t p_bit_offset, void *p_data);
	void (*report_bitcode_bit_offset_fn)(uint64_t p_bit_offset, void *p_data);
} NebulaNirCallbacks;

#ifdef __cplusplus
}
#endif

#endif // D3D12_NEBULA_NIR_BRIDGE_H
