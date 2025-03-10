/**************************************************************************/
/*  translation.compat.inc                                                */
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

#ifndef DISABLE_DEPRECATED

void Translation::_bind_compatibility_methods() {
	ClassDB::bind_compatibility_method(D_METHOD("add_message", "src_message", "xlated_message", "context"), &Translation::add_message, DEFVAL(""));
	ClassDB::bind_compatibility_method(D_METHOD("add_plural_message", "src_message", "xlated_messages", "context"), &Translation::add_plural_message, DEFVAL(""));
	ClassDB::bind_compatibility_method(D_METHOD("get_message", "src_message", "context"), &Translation::get_message, DEFVAL(""));
	ClassDB::bind_compatibility_method(D_METHOD("get_plural_message", "src_message", "src_plural_message", "n", "context"), &Translation::get_plural_message, DEFVAL(""));
	ClassDB::bind_compatibility_method(D_METHOD("erase_message", "src_message", "context"), &Translation::erase_message, DEFVAL(""));
}

void TranslationServer::_bind_compatibility_methods() {
	ClassDB::bind_compatibility_method(D_METHOD("translate", "message", "context"), &TranslationServer::translate, DEFVAL(""));
	ClassDB::bind_compatibility_method(D_METHOD("translate_plural", "message", "plural_message", "n", "context"), &TranslationServer::translate_plural, DEFVAL(""));
}

#endif
