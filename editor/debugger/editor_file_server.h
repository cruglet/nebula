/**************************************************************************/
/*  editor_file_server.h                                                  */
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

#ifndef EDITOR_FILE_SERVER_H
#define EDITOR_FILE_SERVER_H

#include "core/io/packet_peer.h"
#include "core/io/tcp_server.h"
#include "core/object/class_db.h"
#include "core/os/thread.h"
#include "editor/editor_file_system.h"

class EditorFileServer : public Object {
	GDCLASS(EditorFileServer, Object);

	Ref<TCPServer> server;
	String password;
	int port = 0;
	bool active = false;
	void _scan_files_changed(EditorFileSystemDirectory *efd, const Vector<String> &p_tags, HashMap<String, uint64_t> &files_to_send, HashMap<String, uint64_t> &cached_files);

public:
	void poll();

	void start();
	void stop();

	bool is_active() const;

	EditorFileServer();
	~EditorFileServer();
};

#endif // EDITOR_FILE_SERVER_H
