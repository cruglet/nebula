/**************************************************************************/
/*  dtls_server_mbedtls.cpp                                               */
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

#include "dtls_server_mbedtls.h"

#include "packet_peer_mbed_dtls.h"

Error DTLSServerMbedTLS::setup(Ref<TLSOptions> p_options) {
	ERR_FAIL_COND_V(p_options.is_null() || !p_options->is_server(), ERR_INVALID_PARAMETER);
	ERR_FAIL_COND_V(cookies->setup() != OK, ERR_ALREADY_IN_USE);
	tls_options = p_options;
	return OK;
}

void DTLSServerMbedTLS::stop() {
	cookies->clear();
}

Ref<PacketPeerDTLS> DTLSServerMbedTLS::take_connection(Ref<PacketPeerUDP> p_udp_peer) {
	Ref<PacketPeerMbedDTLS> out;

	ERR_FAIL_COND_V(tls_options.is_null(), out);
	ERR_FAIL_COND_V(!p_udp_peer.is_valid(), out);

	out.instantiate();
	out->accept_peer(p_udp_peer, tls_options, cookies);
	return out;
}

DTLSServer *DTLSServerMbedTLS::_create_func() {
	return memnew(DTLSServerMbedTLS);
}

void DTLSServerMbedTLS::initialize() {
	_create = _create_func;
	available = true;
}

void DTLSServerMbedTLS::finalize() {
	_create = nullptr;
	available = false;
}

DTLSServerMbedTLS::DTLSServerMbedTLS() {
	cookies.instantiate();
}

DTLSServerMbedTLS::~DTLSServerMbedTLS() {
	stop();
}
