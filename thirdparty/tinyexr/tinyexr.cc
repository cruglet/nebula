#if defined(_WIN32)
#ifndef NOMINMAX
#define NOMINMAX
#endif
#endif

// -- NEBULA start --
#include <zlib.h> // Should come before including tinyexr.
// -- NEBULA end --

#define TINYEXR_IMPLEMENTATION
#include "tinyexr.h"
