#ifndef NEB_LIB_WBFS
#define NEB_LIB_WBFS
#include <cstdint>
extern "C" {

namespace WBFS {
	int8_t validate_wbfs(const char* path);
}}

#endif
