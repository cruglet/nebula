/**************************************************************************/
/*  validate_game_file.cpp                                                */
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
#include "validate_game_file.h"
#include <iostream>
#include <fstream>
#include <vector>

// Define the constructor
ValidateGameFile::ValidateGameFile() {
    // Constructor logic (if needed)
}

// Define the destructor
ValidateGameFile::~ValidateGameFile() {
    // Destructor logic (if needed)
}


static std::string hexVectorToString(const std::vector<char>& hexVec) {
    return std::string(hexVec.begin(), hexVec.end());  // Convert vector to string
}

std::vector<char> readBinaryData(const std::string& filepath, std::streampos offset, size_t size) {
    std::ifstream file(filepath, std::ios::binary);  // Open file in binary mode
    if (!file) {
        std::runtime_error("Error: Unable to open file!");  // Fixed: actually throwing an error
    }

    file.seekg(offset);  // Move to the given offset
    if (!file) {
        std::runtime_error("Error: Seeking failed!");
    }

    std::vector<char> buffer(size);  // Allocate buffer
    file.read(buffer.data(), size);  // Read binary data

    if (!file) {
        std::runtime_error("Error: Reading failed!");
    }

    return buffer;  // Return the binary data
}

char* ValidateGameFile::getFileHeader(char* f_path) {
    std::string path = f_path;
    std::streampos offset = 0x200;
    size_t size = 6;

    std::vector<char> x = readBinaryData(path, offset, size);

    for (unsigned char byte : x) {
        printf("%02X ", byte);
    }

	 std::string disc_id = hexVectorToString(x);

	 std::cout << disc_id << std::endl;

    printf("\n");

    return f_path;  // Returning input for now (modify as needed)
}


