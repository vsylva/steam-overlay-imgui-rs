// wrapper.h
// bindgen bindgen.h -o lib.rs

#define IMGUI_API
#define IMGUI_IMPL_API
#define CIMGUI_DEFINE_ENUMS_AND_STRUCTS // 关键宏：让 cimgui 展开所有定义

#include "cimgui.h"