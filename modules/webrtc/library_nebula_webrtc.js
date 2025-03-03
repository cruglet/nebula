/**************************************************************************/
/*  library_nebula_webrtc.js                                               */
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

const NebulaRTCDataChannel = {
	// Our socket implementation that forwards events to C++.
	$NebulaRTCDataChannel__deps: ['$IDHandler', '$NebulaRuntime'],
	$NebulaRTCDataChannel: {
		connect: function (p_id, p_on_open, p_on_message, p_on_error, p_on_close) {
			const ref = IDHandler.get(p_id);
			if (!ref) {
				return;
			}

			ref.binaryType = 'arraybuffer';
			ref.onopen = function (event) {
				p_on_open();
			};
			ref.onclose = function (event) {
				p_on_close();
			};
			ref.onerror = function (event) {
				p_on_error();
			};
			ref.onmessage = function (event) {
				let buffer;
				let is_string = 0;
				if (event.data instanceof ArrayBuffer) {
					buffer = new Uint8Array(event.data);
				} else if (event.data instanceof Blob) {
					NebulaRuntime.error('Blob type not supported');
					return;
				} else if (typeof event.data === 'string') {
					is_string = 1;
					const enc = new TextEncoder('utf-8');
					buffer = new Uint8Array(enc.encode(event.data));
				} else {
					NebulaRuntime.error('Unknown message type');
					return;
				}
				const len = buffer.length * buffer.BYTES_PER_ELEMENT;
				const out = NebulaRuntime.malloc(len);
				HEAPU8.set(buffer, out);
				p_on_message(out, len, is_string);
				NebulaRuntime.free(out);
			};
		},

		close: function (p_id) {
			const ref = IDHandler.get(p_id);
			if (!ref) {
				return;
			}
			ref.onopen = null;
			ref.onmessage = null;
			ref.onerror = null;
			ref.onclose = null;
			ref.close();
		},

		get_prop: function (p_id, p_prop, p_def) {
			const ref = IDHandler.get(p_id);
			return (ref && ref[p_prop] !== undefined) ? ref[p_prop] : p_def;
		},
	},

	nebula_js_rtc_datachannel_ready_state_get__proxy: 'sync',
	nebula_js_rtc_datachannel_ready_state_get__sig: 'ii',
	nebula_js_rtc_datachannel_ready_state_get: function (p_id) {
		const ref = IDHandler.get(p_id);
		if (!ref) {
			return 3; // CLOSED
		}

		switch (ref.readyState) {
		case 'connecting':
			return 0;
		case 'open':
			return 1;
		case 'closing':
			return 2;
		case 'closed':
		default:
			return 3;
		}
	},

	nebula_js_rtc_datachannel_send__proxy: 'sync',
	nebula_js_rtc_datachannel_send__sig: 'iiiii',
	nebula_js_rtc_datachannel_send: function (p_id, p_buffer, p_length, p_raw) {
		const ref = IDHandler.get(p_id);
		if (!ref) {
			return 1;
		}

		const bytes_array = new Uint8Array(p_length);
		for (let i = 0; i < p_length; i++) {
			bytes_array[i] = NebulaRuntime.getHeapValue(p_buffer + i, 'i8');
		}

		if (p_raw) {
			ref.send(bytes_array.buffer);
		} else {
			const string = new TextDecoder('utf-8').decode(bytes_array);
			ref.send(string);
		}
		return 0;
	},

	nebula_js_rtc_datachannel_is_ordered__proxy: 'sync',
	nebula_js_rtc_datachannel_is_ordered__sig: 'ii',
	nebula_js_rtc_datachannel_is_ordered: function (p_id) {
		return NebulaRTCDataChannel.get_prop(p_id, 'ordered', true);
	},

	nebula_js_rtc_datachannel_id_get__proxy: 'sync',
	nebula_js_rtc_datachannel_id_get__sig: 'ii',
	nebula_js_rtc_datachannel_id_get: function (p_id) {
		return NebulaRTCDataChannel.get_prop(p_id, 'id', 65535);
	},

	nebula_js_rtc_datachannel_max_packet_lifetime_get__proxy: 'sync',
	nebula_js_rtc_datachannel_max_packet_lifetime_get__sig: 'ii',
	nebula_js_rtc_datachannel_max_packet_lifetime_get: function (p_id) {
		const ref = IDHandler.get(p_id);
		if (!ref) {
			return 65535;
		}
		if (ref['maxPacketLifeTime'] !== undefined) {
			return ref['maxPacketLifeTime'];
		} else if (ref['maxRetransmitTime'] !== undefined) {
			// Guess someone didn't appreciate the standardization process.
			return ref['maxRetransmitTime'];
		}
		return 65535;
	},

	nebula_js_rtc_datachannel_max_retransmits_get__proxy: 'sync',
	nebula_js_rtc_datachannel_max_retransmits_get__sig: 'ii',
	nebula_js_rtc_datachannel_max_retransmits_get: function (p_id) {
		return NebulaRTCDataChannel.get_prop(p_id, 'maxRetransmits', 65535);
	},

	nebula_js_rtc_datachannel_is_negotiated__proxy: 'sync',
	nebula_js_rtc_datachannel_is_negotiated__sig: 'ii',
	nebula_js_rtc_datachannel_is_negotiated: function (p_id) {
		return NebulaRTCDataChannel.get_prop(p_id, 'negotiated', 65535);
	},

	nebula_js_rtc_datachannel_get_buffered_amount__proxy: 'sync',
	nebula_js_rtc_datachannel_get_buffered_amount__sig: 'ii',
	nebula_js_rtc_datachannel_get_buffered_amount: function (p_id) {
		return NebulaRTCDataChannel.get_prop(p_id, 'bufferedAmount', 0);
	},

	nebula_js_rtc_datachannel_label_get__proxy: 'sync',
	nebula_js_rtc_datachannel_label_get__sig: 'ii',
	nebula_js_rtc_datachannel_label_get: function (p_id) {
		const ref = IDHandler.get(p_id);
		if (!ref || !ref.label) {
			return 0;
		}
		return NebulaRuntime.allocString(ref.label);
	},

	nebula_js_rtc_datachannel_protocol_get__sig: 'ii',
	nebula_js_rtc_datachannel_protocol_get: function (p_id) {
		const ref = IDHandler.get(p_id);
		if (!ref || !ref.protocol) {
			return 0;
		}
		return NebulaRuntime.allocString(ref.protocol);
	},

	nebula_js_rtc_datachannel_destroy__proxy: 'sync',
	nebula_js_rtc_datachannel_destroy__sig: 'vi',
	nebula_js_rtc_datachannel_destroy: function (p_id) {
		NebulaRTCDataChannel.close(p_id);
		IDHandler.remove(p_id);
	},

	nebula_js_rtc_datachannel_connect__proxy: 'sync',
	nebula_js_rtc_datachannel_connect__sig: 'viiiiii',
	nebula_js_rtc_datachannel_connect: function (p_id, p_ref, p_on_open, p_on_message, p_on_error, p_on_close) {
		const onopen = NebulaRuntime.get_func(p_on_open).bind(null, p_ref);
		const onmessage = NebulaRuntime.get_func(p_on_message).bind(null, p_ref);
		const onerror = NebulaRuntime.get_func(p_on_error).bind(null, p_ref);
		const onclose = NebulaRuntime.get_func(p_on_close).bind(null, p_ref);
		NebulaRTCDataChannel.connect(p_id, onopen, onmessage, onerror, onclose);
	},

	nebula_js_rtc_datachannel_close__proxy: 'sync',
	nebula_js_rtc_datachannel_close__sig: 'vi',
	nebula_js_rtc_datachannel_close: function (p_id) {
		const ref = IDHandler.get(p_id);
		if (!ref) {
			return;
		}
		NebulaRTCDataChannel.close(p_id);
	},
};

autoAddDeps(NebulaRTCDataChannel, '$NebulaRTCDataChannel');
mergeInto(LibraryManager.library, NebulaRTCDataChannel);

const NebulaRTCPeerConnection = {
	$NebulaRTCPeerConnection__deps: ['$IDHandler', '$NebulaRuntime', '$NebulaRTCDataChannel'],
	$NebulaRTCPeerConnection: {
		// Enums
		ConnectionState: {
			'new': 0,
			'connecting': 1,
			'connected': 2,
			'disconnected': 3,
			'failed': 4,
			'closed': 5,
		},

		ConnectionStateCompat: {
			// Using values from IceConnectionState for browsers that do not support ConnectionState (notably Firefox).
			'new': 0,
			'checking': 1,
			'connected': 2,
			'completed': 2,
			'disconnected': 3,
			'failed': 4,
			'closed': 5,
		},

		IceGatheringState: {
			'new': 0,
			'gathering': 1,
			'complete': 2,
		},

		SignalingState: {
			'stable': 0,
			'have-local-offer': 1,
			'have-remote-offer': 2,
			'have-local-pranswer': 3,
			'have-remote-pranswer': 4,
			'closed': 5,
		},

		// Callbacks
		create: function (config, onConnectionChange, onSignalingChange, onIceGatheringChange, onIceCandidate, onDataChannel) {
			let conn = null;
			try {
				conn = new RTCPeerConnection(config);
			} catch (e) {
				NebulaRuntime.error(e);
				return 0;
			}

			const id = IDHandler.add(conn);

			if ('connectionState' in conn && conn['connectionState'] !== undefined) {
				// Use "connectionState" if supported
				conn.onconnectionstatechange = function (event) {
					if (!IDHandler.get(id)) {
						return;
					}
					onConnectionChange(NebulaRTCPeerConnection.ConnectionState[conn.connectionState] || 0);
				};
			} else {
				// Fall back to using "iceConnectionState" when "connectionState" is not supported (notably Firefox).
				conn.oniceconnectionstatechange = function (event) {
					if (!IDHandler.get(id)) {
						return;
					}
					onConnectionChange(NebulaRTCPeerConnection.ConnectionStateCompat[conn.iceConnectionState] || 0);
				};
			}
			conn.onicegatheringstatechange = function (event) {
				if (!IDHandler.get(id)) {
					return;
				}
				onIceGatheringChange(NebulaRTCPeerConnection.IceGatheringState[conn.iceGatheringState] || 0);
			};
			conn.onsignalingstatechange = function (event) {
				if (!IDHandler.get(id)) {
					return;
				}
				onSignalingChange(NebulaRTCPeerConnection.SignalingState[conn.signalingState] || 0);
			};
			conn.onicecandidate = function (event) {
				if (!IDHandler.get(id)) {
					return;
				}
				const c = event.candidate;
				if (!c || !c.candidate) {
					return;
				}
				const candidate_str = NebulaRuntime.allocString(c.candidate);
				const mid_str = NebulaRuntime.allocString(c.sdpMid);
				onIceCandidate(mid_str, c.sdpMLineIndex, candidate_str);
				NebulaRuntime.free(candidate_str);
				NebulaRuntime.free(mid_str);
			};
			conn.ondatachannel = function (event) {
				if (!IDHandler.get(id)) {
					return;
				}
				const cid = IDHandler.add(event.channel);
				onDataChannel(cid);
			};
			return id;
		},

		destroy: function (p_id) {
			const conn = IDHandler.get(p_id);
			if (!conn) {
				return;
			}
			conn.onconnectionstatechange = null;
			conn.oniceconnectionstatechange = null;
			conn.onicegatheringstatechange = null;
			conn.onsignalingstatechange = null;
			conn.onicecandidate = null;
			conn.ondatachannel = null;
			IDHandler.remove(p_id);
		},

		onsession: function (p_id, callback, session) {
			if (!IDHandler.get(p_id)) {
				return;
			}
			const type_str = NebulaRuntime.allocString(session.type);
			const sdp_str = NebulaRuntime.allocString(session.sdp);
			callback(type_str, sdp_str);
			NebulaRuntime.free(type_str);
			NebulaRuntime.free(sdp_str);
		},

		onerror: function (p_id, callback, error) {
			const ref = IDHandler.get(p_id);
			if (!ref) {
				return;
			}
			NebulaRuntime.error(error);
			callback();
		},
	},

	nebula_js_rtc_pc_create__proxy: 'sync',
	nebula_js_rtc_pc_create__sig: 'iiiiiiii',
	nebula_js_rtc_pc_create: function (p_config, p_ref, p_on_connection_state_change, p_on_ice_gathering_state_change, p_on_signaling_state_change, p_on_ice_candidate, p_on_datachannel) {
		const wrap = function (p_func) {
			return NebulaRuntime.get_func(p_func).bind(null, p_ref);
		};
		return NebulaRTCPeerConnection.create(
			JSON.parse(NebulaRuntime.parseString(p_config)),
			wrap(p_on_connection_state_change),
			wrap(p_on_signaling_state_change),
			wrap(p_on_ice_gathering_state_change),
			wrap(p_on_ice_candidate),
			wrap(p_on_datachannel)
		);
	},

	nebula_js_rtc_pc_close__proxy: 'sync',
	nebula_js_rtc_pc_close__sig: 'vi',
	nebula_js_rtc_pc_close: function (p_id) {
		const ref = IDHandler.get(p_id);
		if (!ref) {
			return;
		}
		ref.close();
	},

	nebula_js_rtc_pc_destroy__proxy: 'sync',
	nebula_js_rtc_pc_destroy__sig: 'vi',
	nebula_js_rtc_pc_destroy: function (p_id) {
		NebulaRTCPeerConnection.destroy(p_id);
	},

	nebula_js_rtc_pc_offer_create__proxy: 'sync',
	nebula_js_rtc_pc_offer_create__sig: 'viiii',
	nebula_js_rtc_pc_offer_create: function (p_id, p_obj, p_on_session, p_on_error) {
		const ref = IDHandler.get(p_id);
		if (!ref) {
			return;
		}
		const onsession = NebulaRuntime.get_func(p_on_session).bind(null, p_obj);
		const onerror = NebulaRuntime.get_func(p_on_error).bind(null, p_obj);
		ref.createOffer().then(function (session) {
			NebulaRTCPeerConnection.onsession(p_id, onsession, session);
		}).catch(function (error) {
			NebulaRTCPeerConnection.onerror(p_id, onerror, error);
		});
	},

	nebula_js_rtc_pc_local_description_set__proxy: 'sync',
	nebula_js_rtc_pc_local_description_set__sig: 'viiiii',
	nebula_js_rtc_pc_local_description_set: function (p_id, p_type, p_sdp, p_obj, p_on_error) {
		const ref = IDHandler.get(p_id);
		if (!ref) {
			return;
		}
		const type = NebulaRuntime.parseString(p_type);
		const sdp = NebulaRuntime.parseString(p_sdp);
		const onerror = NebulaRuntime.get_func(p_on_error).bind(null, p_obj);
		ref.setLocalDescription({
			'sdp': sdp,
			'type': type,
		}).catch(function (error) {
			NebulaRTCPeerConnection.onerror(p_id, onerror, error);
		});
	},

	nebula_js_rtc_pc_remote_description_set__proxy: 'sync',
	nebula_js_rtc_pc_remote_description_set__sig: 'viiiiii',
	nebula_js_rtc_pc_remote_description_set: function (p_id, p_type, p_sdp, p_obj, p_session_created, p_on_error) {
		const ref = IDHandler.get(p_id);
		if (!ref) {
			return;
		}
		const type = NebulaRuntime.parseString(p_type);
		const sdp = NebulaRuntime.parseString(p_sdp);
		const onerror = NebulaRuntime.get_func(p_on_error).bind(null, p_obj);
		const onsession = NebulaRuntime.get_func(p_session_created).bind(null, p_obj);
		ref.setRemoteDescription({
			'sdp': sdp,
			'type': type,
		}).then(function () {
			if (type !== 'offer') {
				return Promise.resolve();
			}
			return ref.createAnswer().then(function (session) {
				NebulaRTCPeerConnection.onsession(p_id, onsession, session);
			});
		}).catch(function (error) {
			NebulaRTCPeerConnection.onerror(p_id, onerror, error);
		});
	},

	nebula_js_rtc_pc_ice_candidate_add__proxy: 'sync',
	nebula_js_rtc_pc_ice_candidate_add__sig: 'viiii',
	nebula_js_rtc_pc_ice_candidate_add: function (p_id, p_mid_name, p_mline_idx, p_sdp) {
		const ref = IDHandler.get(p_id);
		if (!ref) {
			return;
		}
		const sdpMidName = NebulaRuntime.parseString(p_mid_name);
		const sdpName = NebulaRuntime.parseString(p_sdp);
		ref.addIceCandidate(new RTCIceCandidate({
			'candidate': sdpName,
			'sdpMid': sdpMidName,
			'sdpMlineIndex': p_mline_idx,
		}));
	},

	nebula_js_rtc_pc_datachannel_create__deps: ['$NebulaRTCDataChannel'],
	nebula_js_rtc_pc_datachannel_create__proxy: 'sync',
	nebula_js_rtc_pc_datachannel_create__sig: 'iiii',
	nebula_js_rtc_pc_datachannel_create: function (p_id, p_label, p_config) {
		try {
			const ref = IDHandler.get(p_id);
			if (!ref) {
				return 0;
			}

			const label = NebulaRuntime.parseString(p_label);
			const config = JSON.parse(NebulaRuntime.parseString(p_config));

			const channel = ref.createDataChannel(label, config);
			return IDHandler.add(channel);
		} catch (e) {
			NebulaRuntime.error(e);
			return 0;
		}
	},
};

autoAddDeps(NebulaRTCPeerConnection, '$NebulaRTCPeerConnection');
mergeInto(LibraryManager.library, NebulaRTCPeerConnection);
