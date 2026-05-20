use std::{ffi::CStr, ptr::null};

use ffi::{
    ImDrawList, ImDrawList_AddCircle, ImDrawList_AddCircleFilled, ImDrawList_AddLine, ImDrawList_AddRect,
    ImDrawList_AddRectFilled, ImDrawList_AddRectFilledMultiColor, ImDrawList_AddText_Vec2, ImDrawList_PopClipRect,
    ImDrawList_PushClipRect, ImGuiChildFlags__ImGuiChildFlags_Borders, ImGuiCol__ImGuiCol_Border,
    ImGuiCol__ImGuiCol_Button, ImGuiCol__ImGuiCol_ButtonActive, ImGuiCol__ImGuiCol_ButtonHovered,
    ImGuiCol__ImGuiCol_ChildBg, ImGuiCol__ImGuiCol_FrameBg, ImGuiCol__ImGuiCol_FrameBgActive,
    ImGuiCol__ImGuiCol_FrameBgHovered, ImGuiCol__ImGuiCol_PopupBg, ImGuiCol__ImGuiCol_SliderGrab,
    ImGuiCol__ImGuiCol_TabHovered, ImGuiCol__ImGuiCol_TabSelected, ImGuiCol__ImGuiCol_Text,
    ImGuiCol__ImGuiCol_WindowBg, ImGuiInputTextFlags, ImGuiKey, ImGuiKey_ImGuiKey_0, ImGuiKey_ImGuiKey_1,
    ImGuiKey_ImGuiKey_2, ImGuiKey_ImGuiKey_3, ImGuiKey_ImGuiKey_4, ImGuiKey_ImGuiKey_5, ImGuiKey_ImGuiKey_6,
    ImGuiKey_ImGuiKey_7, ImGuiKey_ImGuiKey_8, ImGuiKey_ImGuiKey_9, ImGuiKey_ImGuiKey_A, ImGuiKey_ImGuiKey_Apostrophe,
    ImGuiKey_ImGuiKey_B, ImGuiKey_ImGuiKey_Backslash, ImGuiKey_ImGuiKey_Backspace, ImGuiKey_ImGuiKey_C,
    ImGuiKey_ImGuiKey_CapsLock, ImGuiKey_ImGuiKey_Comma, ImGuiKey_ImGuiKey_D, ImGuiKey_ImGuiKey_Delete,
    ImGuiKey_ImGuiKey_DownArrow, ImGuiKey_ImGuiKey_E, ImGuiKey_ImGuiKey_End, ImGuiKey_ImGuiKey_Enter,
    ImGuiKey_ImGuiKey_Equal, ImGuiKey_ImGuiKey_Escape, ImGuiKey_ImGuiKey_F, ImGuiKey_ImGuiKey_F1, ImGuiKey_ImGuiKey_F2,
    ImGuiKey_ImGuiKey_F3, ImGuiKey_ImGuiKey_F4, ImGuiKey_ImGuiKey_F5, ImGuiKey_ImGuiKey_F6, ImGuiKey_ImGuiKey_F7,
    ImGuiKey_ImGuiKey_F8, ImGuiKey_ImGuiKey_F9, ImGuiKey_ImGuiKey_F10, ImGuiKey_ImGuiKey_F11, ImGuiKey_ImGuiKey_F12,
    ImGuiKey_ImGuiKey_G, ImGuiKey_ImGuiKey_GraveAccent, ImGuiKey_ImGuiKey_H, ImGuiKey_ImGuiKey_Home,
    ImGuiKey_ImGuiKey_I, ImGuiKey_ImGuiKey_Insert, ImGuiKey_ImGuiKey_J, ImGuiKey_ImGuiKey_K, ImGuiKey_ImGuiKey_Keypad0,
    ImGuiKey_ImGuiKey_Keypad1, ImGuiKey_ImGuiKey_Keypad2, ImGuiKey_ImGuiKey_Keypad3, ImGuiKey_ImGuiKey_Keypad4,
    ImGuiKey_ImGuiKey_Keypad5, ImGuiKey_ImGuiKey_Keypad6, ImGuiKey_ImGuiKey_Keypad7, ImGuiKey_ImGuiKey_Keypad8,
    ImGuiKey_ImGuiKey_Keypad9, ImGuiKey_ImGuiKey_KeypadAdd, ImGuiKey_ImGuiKey_KeypadDecimal,
    ImGuiKey_ImGuiKey_KeypadDivide, ImGuiKey_ImGuiKey_KeypadEnter, ImGuiKey_ImGuiKey_KeypadMultiply,
    ImGuiKey_ImGuiKey_KeypadSubtract, ImGuiKey_ImGuiKey_L, ImGuiKey_ImGuiKey_LeftArrow, ImGuiKey_ImGuiKey_LeftBracket,
    ImGuiKey_ImGuiKey_M, ImGuiKey_ImGuiKey_Minus, ImGuiKey_ImGuiKey_N, ImGuiKey_ImGuiKey_None,
    ImGuiKey_ImGuiKey_NumLock, ImGuiKey_ImGuiKey_O, ImGuiKey_ImGuiKey_P, ImGuiKey_ImGuiKey_PageDown,
    ImGuiKey_ImGuiKey_PageUp, ImGuiKey_ImGuiKey_Pause, ImGuiKey_ImGuiKey_Period, ImGuiKey_ImGuiKey_PrintScreen,
    ImGuiKey_ImGuiKey_Q, ImGuiKey_ImGuiKey_R, ImGuiKey_ImGuiKey_RightArrow, ImGuiKey_ImGuiKey_RightBracket,
    ImGuiKey_ImGuiKey_S, ImGuiKey_ImGuiKey_ScrollLock, ImGuiKey_ImGuiKey_Semicolon, ImGuiKey_ImGuiKey_Slash,
    ImGuiKey_ImGuiKey_Space, ImGuiKey_ImGuiKey_T, ImGuiKey_ImGuiKey_Tab, ImGuiKey_ImGuiKey_U,
    ImGuiKey_ImGuiKey_UpArrow, ImGuiKey_ImGuiKey_V, ImGuiKey_ImGuiKey_W, ImGuiKey_ImGuiKey_X, ImGuiKey_ImGuiKey_Y,
    ImGuiKey_ImGuiKey_Z, ImGuiStyleVar__ImGuiStyleVar_ChildBorderSize, ImGuiStyleVar__ImGuiStyleVar_ChildRounding,
    ImGuiStyleVar__ImGuiStyleVar_FramePadding, ImGuiStyleVar__ImGuiStyleVar_FrameRounding,
    ImGuiStyleVar__ImGuiStyleVar_PopupBorderSize, ImGuiStyleVar__ImGuiStyleVar_WindowPadding,
    ImGuiStyleVar__ImGuiStyleVar_WindowRounding, ImU32, ImVec2_c, ImVec4_c, igBegin, igBeginChildEx, igBeginCombo,
    igBeginGroup, igBeginTooltip, igCalcTextSize, igColumns, igDummy, igEnd, igEndChild, igEndCombo, igEndGroup,
    igEndTooltip, igGetContentRegionAvail, igGetCursorScreenPos, igGetForegroundDrawList_Nil, igGetID_Str, igGetIO_Nil,
    igGetKeyName, igGetStyle, igGetTextLineHeight, igGetTime, igGetWindowDrawList, igGetWindowPos, igGetWindowSize,
    igInputText, igInvisibleButton, igIsItemActive, igIsItemClicked, igIsItemHovered, igIsKeyPressed_Bool, igNewLine,
    igNextColumn, igPopItemWidth, igPopStyleColor, igPopStyleVar, igPushItemWidth, igPushStyleColor_U32,
    igPushStyleVar_Float, igPushStyleVar_Vec2, igSameLine, igSelectable_Bool, igSetColumnWidth, igSetCursorScreenPos,
    igSetItemDefaultFocus, igSetNextWindowPos, igSetNextWindowSize, igSetNextWindowSizeConstraints, igSpacing, igText,
    igTextDisabled, igTextUnformatted,
};

use crate::ffi;

fn im_col32(r: u8, g: u8, b: u8, a: u8,) -> ImU32 {
    ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32)
}

unsafe fn cstr(s: &str,) -> std::ffi::CString {
    std::ffi::CString::new(s.trim_end_matches('\0',),).unwrap_unchecked()
}

pub mod color {
    use crate::ffi::ImVec4_c;

    pub const BG_DEEP: ImVec4_c = ImVec4_c { x: 0.055, y: 0.055, z: 0.075, w: 1.000, };
    pub const BG_BASE: ImVec4_c = ImVec4_c { x: 0.090, y: 0.090, z: 0.115, w: 1.000, };
    pub const BG_ELEVATE: ImVec4_c = ImVec4_c { x: 0.130, y: 0.130, z: 0.165, w: 1.000, };
    pub const BG_WIDGET: ImVec4_c = ImVec4_c { x: 0.160, y: 0.160, z: 0.200, w: 1.000, };

    pub const ACCENT: ImVec4_c = ImVec4_c { x: 0.000, y: 0.820, z: 0.880, w: 1.000, };
    pub const ACCENT_DIM: ImVec4_c = ImVec4_c { x: 0.000, y: 0.480, z: 0.530, w: 1.000, };
    pub const ACCENT_BRIGHT: ImVec4_c = ImVec4_c { x: 0.450, y: 1.000, z: 1.000, w: 1.000, };
    pub const ACCENT_GLOW: ImVec4_c = ImVec4_c { x: 0.000, y: 0.820, z: 0.880, w: 0.120, };

    pub const DANGER: ImVec4_c = ImVec4_c { x: 0.960, y: 0.220, z: 0.340, w: 1.000, };
    pub const SUCCESS: ImVec4_c = ImVec4_c { x: 0.150, y: 0.870, z: 0.420, w: 1.000, };
    pub const WARNING: ImVec4_c = ImVec4_c { x: 1.000, y: 0.730, z: 0.070, w: 1.000, };

    pub const TEXT_HIGH: ImVec4_c = ImVec4_c { x: 0.940, y: 0.940, z: 0.960, w: 1.000, };
    pub const TEXT_MID: ImVec4_c = ImVec4_c { x: 0.580, y: 0.580, z: 0.630, w: 1.000, };
    pub const TEXT_LOW: ImVec4_c = ImVec4_c { x: 0.330, y: 0.330, z: 0.370, w: 1.000, };
}

mod _impl {
    use std::{collections::HashMap, sync::LazyLock};

    use crate::ffi::{
        ImDrawList, ImDrawList_AddRectFilled, ImGuiID, ImU32, ImVec2_c, ImVec4_c, igColorConvertFloat4ToU32,
        igGetIO_Nil,
    };

    static mut S_ANIM: LazyLock<HashMap<ImGuiID, f32,>,> = LazyLock::new(|| HashMap::new(),);

    pub unsafe fn anim(id: ImGuiID, target: f32, speed: f32,) -> f32 {
        let v = S_ANIM.entry(id,).or_insert(target,);
        let dt = unsafe { (*igGetIO_Nil()).DeltaTime };
        *v += (target - *v) * (1.0 - (-speed * dt).exp());
        *v
    }

    pub fn lerp4(a: ImVec4_c, b: ImVec4_c, t: f32,) -> ImVec4_c {
        ImVec4_c {
            x: a.x + (b.x - a.x) * t,
            y: a.y + (b.y - a.y) * t,
            z: a.z + (b.z - a.z) * t,
            w: a.w + (b.w - a.w) * t,
        }
    }

    pub fn u32(c: ImVec4_c,) -> ImU32 {
        unsafe { igColorConvertFloat4ToU32(c,) }
    }

    pub fn alpha(c: ImVec4_c, a: f32,) -> ImVec4_c {
        ImVec4_c { x: c.x, y: c.y, z: c.z, w: a, }
    }

    pub fn glow_rect(dl: *mut ImDrawList, mn: ImVec2_c, mx: ImVec2_c, col: ImVec4_c, radius: f32, alpha_val: f32,) {
        for i in (1..=3).rev()
        {
            let s = i as f32;
            let alf = alpha_val * (s / 3.0);
            unsafe {
                ImDrawList_AddRectFilled(
                    dl,
                    ImVec2_c { x: mn.x - s * radius * 0.25, y: mn.y - s * radius * 0.25, },
                    ImVec2_c { x: mx.x + s * radius * 0.25, y: mx.y + s * radius * 0.25, },
                    u32(alpha(col, alf,),),
                    radius + s * 2.0,
                    0,
                );
            }
        }
    }

    pub fn im_clamp_f32(v: f32, lo: f32, hi: f32,) -> f32 {
        if v < lo
        {
            lo
        }
        else if v > hi
        {
            hi
        }
        else
        {
            v
        }
    }

    pub fn im_max_f32(a: f32, b: f32,) -> f32 {
        if a > b { a } else { b }
    }

    pub fn im_min_f32(a: f32, b: f32,) -> f32 {
        if a < b { a } else { b }
    }
}

use _impl::*;

pub fn apply_theme() {
    unsafe {
        let s: &mut ffi::ImGuiStyle = &mut *igGetStyle();
        let c: &mut [ImVec4_c; 60] = &mut s.Colors;

        s.WindowRounding = 12.0;
        s.ChildRounding = 8.0;
        s.FrameRounding = 7.0;
        s.PopupRounding = 10.0;
        s.ScrollbarRounding = 8.0;
        s.GrabRounding = 6.0;
        s.TabRounding = 7.0;
        s.WindowBorderSize = 1.0;
        s.FrameBorderSize = 0.0;
        s.PopupBorderSize = 1.0;
        s.TabBorderSize = 0.0;

        s.WindowPadding = ImVec2_c { x: 16.0, y: 16.0, };
        s.FramePadding = ImVec2_c { x: 11.0, y: 7.0, };
        s.ItemSpacing = ImVec2_c { x: 10.0, y: 9.0, };
        s.ItemInnerSpacing = ImVec2_c { x: 7.0, y: 5.0, };
        s.ScrollbarSize = 10.0;
        s.GrabMinSize = 14.0;
        s.IndentSpacing = 18.0;

        c[ImGuiCol__ImGuiCol_WindowBg as usize] = color::BG_DEEP;
        c[ImGuiCol__ImGuiCol_ChildBg as usize] = color::BG_BASE;
        c[ImGuiCol__ImGuiCol_PopupBg as usize] = ImVec4_c { x: 0.08, y: 0.08, z: 0.11, w: 0.97, };

        c[ImGuiCol__ImGuiCol_Border as usize] = ImVec4_c { x: 0.20, y: 0.20, z: 0.27, w: 0.75, };
        c[6] = ImVec4_c { x: 0.0, y: 0.0, z: 0.0, w: 0.0, };

        c[ImGuiCol__ImGuiCol_FrameBg as usize] = color::BG_WIDGET;
        c[ImGuiCol__ImGuiCol_FrameBgHovered as usize] = ImVec4_c { x: 0.20, y: 0.20, z: 0.27, w: 1.0, };
        c[ImGuiCol__ImGuiCol_FrameBgActive as usize] = ImVec4_c { x: 0.24, y: 0.24, z: 0.32, w: 1.0, };

        c[10] = ImVec4_c { x: 0.06, y: 0.06, z: 0.08, w: 1.0, };
        c[11] = ImVec4_c { x: 0.07, y: 0.07, z: 0.10, w: 1.0, };
        c[12] = ImVec4_c { x: 0.06, y: 0.06, z: 0.08, w: 0.80, };

        c[13] = ImVec4_c { x: 0.07, y: 0.07, z: 0.09, w: 1.0, };

        c[14] = alpha(color::BG_DEEP, 0.60,);
        c[15] = alpha(color::ACCENT_DIM, 0.70,);
        c[16] = alpha(color::ACCENT, 0.80,);
        c[17] = color::ACCENT;

        c[18] = color::ACCENT;
        c[ImGuiCol__ImGuiCol_SliderGrab as usize] = color::ACCENT;
        c[20] = color::ACCENT_BRIGHT;

        c[ImGuiCol__ImGuiCol_Button as usize] = color::BG_WIDGET;
        c[ImGuiCol__ImGuiCol_ButtonHovered as usize] = ImVec4_c { x: 0.06, y: 0.42, z: 0.44, w: 0.65, };
        c[ImGuiCol__ImGuiCol_ButtonActive as usize] = ImVec4_c { x: 0.08, y: 0.55, z: 0.58, w: 0.80, };

        c[24] = alpha(color::ACCENT, 0.22,);
        c[25] = alpha(color::ACCENT, 0.38,);
        c[26] = alpha(color::ACCENT, 0.55,);

        c[27] = ImVec4_c { x: 0.20, y: 0.20, z: 0.27, w: 0.75, };
        c[28] = alpha(color::ACCENT, 0.70,);
        c[29] = color::ACCENT;

        c[30] = alpha(color::ACCENT, 0.18,);
        c[31] = alpha(color::ACCENT, 0.45,);
        c[32] = alpha(color::ACCENT, 0.85,);

        c[35] = ImVec4_c { x: 0.09, y: 0.09, z: 0.12, w: 0.95, };
        c[ImGuiCol__ImGuiCol_TabHovered as usize] = alpha(color::ACCENT, 0.38,);
        c[ImGuiCol__ImGuiCol_TabSelected as usize] = ImVec4_c { x: 0.10, y: 0.42, z: 0.46, w: 0.85, };
        c[38] = ImVec4_c { x: 0.07, y: 0.07, z: 0.09, w: 0.95, };
        c[39] = ImVec4_c { x: 0.09, y: 0.33, z: 0.36, w: 0.75, };

        c[41] = color::ACCENT;
        c[42] = color::ACCENT_BRIGHT;
        c[43] = alpha(color::ACCENT, 0.85,);
        c[44] = color::ACCENT_BRIGHT;

        c[45] = ImVec4_c { x: 0.09, y: 0.09, z: 0.12, w: 1.0, };
        c[46] = ImVec4_c { x: 0.22, y: 0.22, z: 0.30, w: 1.0, };
        c[47] = ImVec4_c { x: 0.16, y: 0.16, z: 0.22, w: 1.0, };

        c[ImGuiCol__ImGuiCol_Text as usize] = color::TEXT_HIGH;
        c[1] = color::TEXT_LOW;
        c[51] = alpha(color::ACCENT, 0.28,);

        c[56] = color::ACCENT;
        c[57] = alpha(color::ACCENT, 0.70,);
        c[53] = alpha(color::ACCENT_BRIGHT, 0.90,);
        c[59] = ImVec4_c { x: 0.04, y: 0.04, z: 0.06, w: 0.60, };
    }
}

pub enum ButtonVariant {
    Primary,
    Ghost,
    Danger,
}

pub fn button(label: &str, size: ImVec2_c, variant: ButtonVariant,) -> bool {
    unsafe {
        let id = igGetID_Str(label.as_ptr() as *const i8,);
        let pos = igGetCursorScreenPos();
        let label_cstr = cstr(label,);

        let text_sz = igCalcTextSize(label_cstr.as_ptr(), null(), false, -1.0,);
        let btn_w = if size.x > 0.0 { size.x } else { text_sz.x + 28.0 };
        let btn_h = if size.y > 0.0 { size.y } else { 34.0 };
        let btn_sz = ImVec2_c { x: btn_w, y: btn_h, };

        let pressed = igInvisibleButton(label_cstr.as_ptr(), btn_sz, 0,);
        let hovered = igIsItemHovered(0,);
        let active = igIsItemActive();

        let h_t = anim(id, if hovered { 1.0 } else { 0.0 }, 11.0,);
        let a_t = anim(id + 0xA000, if active { 1.0 } else { 0.0 }, 22.0,);

        let mn = pos;
        let mx = ImVec2_c { x: pos.x + btn_w, y: pos.y + btn_h, };
        let rr = 7.0;

        let dl = igGetWindowDrawList();

        let accent_col = match variant
        {
            ButtonVariant::Danger => color::DANGER,
            _ => color::ACCENT,
        };

        if h_t > 0.02
        {
            glow_rect(dl, mn, mx, accent_col, 10.0, h_t * 0.22,);
        }

        let bg = match variant
        {
            ButtonVariant::Ghost => alpha(accent_col, h_t * 0.25 + a_t * 0.15,),
            _ =>
            {
                let base = lerp4(
                    color::BG_WIDGET,
                    lerp4(
                        ImVec4_c { x: 0.04, y: 0.38, z: 0.42, w: 1.0, },
                        ImVec4_c { x: 0.08, y: 0.52, z: 0.56, w: 1.0, },
                        a_t,
                    ),
                    h_t * 0.65 + a_t * 0.35,
                );
                match variant
                {
                    ButtonVariant::Danger => lerp4(
                        color::BG_WIDGET,
                        ImVec4_c { x: 0.45, y: 0.06, z: 0.10, w: 1.0, },
                        h_t * 0.65 + a_t * 0.35,
                    ),
                    _ => base,
                }
            }
        };
        ImDrawList_AddRectFilled(dl, mn, mx, u32(bg,), rr, 0,);

        let border_a = match variant
        {
            ButtonVariant::Ghost => 0.55 + h_t * 0.35,
            _ => 0.25 + h_t * 0.55,
        };
        ImDrawList_AddRect(dl, mn, mx, u32(alpha(accent_col, border_a,),), rr, 0, 1.1 + h_t * 0.7,);

        if h_t > 0.01
        {
            ImDrawList_AddRectFilled(
                dl,
                ImVec2_c { x: mn.x + rr, y: mn.y, },
                ImVec2_c { x: mx.x - rr, y: mn.y + 1.5, },
                u32(alpha(color::ACCENT_BRIGHT, h_t * 0.60,),),
                1.0,
                0,
            );
        }

        let nudge = a_t * 1.2;
        let tc = lerp4(color::TEXT_HIGH, color::ACCENT_BRIGHT, h_t * 0.55,);
        ImDrawList_AddText_Vec2(
            dl,
            ImVec2_c { x: mn.x + (btn_sz.x - text_sz.x) * 0.5, y: mn.y + (btn_sz.y - text_sz.y) * 0.5 + nudge, },
            u32(tc,),
            label_cstr.as_ptr(),
            null(),
        );

        pressed
    }
}

pub fn checkbox(label: &str, v: &mut bool,) -> bool {
    unsafe {
        let label_cstr = cstr(label,);
        let id = igGetID_Str(label_cstr.as_ptr(),);
        let pos = igGetCursorScreenPos();
        let sz = 18.0;
        let rr = 4.5;

        let row_h = im_max_f32(sz, igGetTextLineHeight(),);
        let hit_w = sz + 10.0 + igCalcTextSize(label_cstr.as_ptr(), null(), false, -1.0,).x;
        let hit_sz = ImVec2_c { x: hit_w, y: row_h, };

        igInvisibleButton(label_cstr.as_ptr(), hit_sz, 0,);
        let clicked = igIsItemClicked(0,);
        if clicked
        {
            *v = !*v;
        }

        let hovered = igIsItemHovered(0,);
        let chk_t = anim(id, if *v { 1.0 } else { 0.0 }, 14.0,);
        let hov_t = anim(id + 0xB000, if hovered { 1.0 } else { 0.0 }, 10.0,);

        let dl = igGetWindowDrawList();
        let cy = pos.y + (row_h - sz) * 0.5;
        let mn = ImVec2_c { x: pos.x, y: cy, };
        let mx = ImVec2_c { x: pos.x + sz, y: cy + sz, };

        if chk_t > 0.02
        {
            glow_rect(dl, mn, mx, color::ACCENT, 8.0, chk_t * 0.18,);
        }

        let fill = lerp4(
            lerp4(color::BG_WIDGET, ImVec4_c { x: 0.12, y: 0.12, z: 0.16, w: 1.0, }, hov_t * 0.4,),
            ImVec4_c { x: 0.02, y: 0.42, z: 0.46, w: 1.0, },
            chk_t,
        );
        ImDrawList_AddRectFilled(dl, mn, mx, u32(fill,), rr, 0,);

        let ba = 0.28 + chk_t * 0.55 + hov_t * 0.18;
        ImDrawList_AddRect(dl, mn, mx, u32(alpha(color::ACCENT, ba,),), rr, 0, 1.2,);

        if chk_t > 0.005
        {
            let p0 = ImVec2_c { x: pos.x + sz * 0.20, y: cy + sz * 0.52, };
            let p1 = ImVec2_c { x: pos.x + sz * 0.42, y: cy + sz * 0.73, };
            let p2 = ImVec2_c { x: pos.x + sz * 0.80, y: cy + sz * 0.24, };

            let t1 = im_min_f32(chk_t * 2.0, 1.0,);
            let t2 = im_max_f32((chk_t - 0.5) * 2.0, 0.0,);

            if t1 > 0.0
            {
                let ep = ImVec2_c { x: p0.x + (p1.x - p0.x) * t1, y: p0.y + (p1.y - p0.y) * t1, };
                ImDrawList_AddLine(dl, p0, ep, u32(color::ACCENT_BRIGHT,), 2.1,);
            }
            if t2 > 0.0
            {
                let ep = ImVec2_c { x: p1.x + (p2.x - p1.x) * t2, y: p1.y + (p2.y - p1.y) * t2, };
                ImDrawList_AddLine(dl, p1, ep, u32(color::ACCENT_BRIGHT,), 2.1,);
            }
        }

        let ty = pos.y + (row_h - igGetTextLineHeight()) * 0.5;
        let tc = lerp4(color::TEXT_MID, color::TEXT_HIGH, chk_t * 0.5 + hov_t * 0.5,);
        ImDrawList_AddText_Vec2(dl, ImVec2_c { x: pos.x + sz + 9.0, y: ty, }, u32(tc,), label_cstr.as_ptr(), null(),);

        clicked
    }
}

pub fn toggle(label: &str, v: &mut bool,) -> bool {
    unsafe {
        let label_cstr = cstr(label,);
        let id = igGetID_Str(label_cstr.as_ptr(),);
        let pos = igGetCursorScreenPos();
        let tw = 42.0;
        let th = 22.0;
        let tr = th * 0.5;

        let row_h = im_max_f32(th, igGetTextLineHeight(),);
        let label_w = igCalcTextSize(label_cstr.as_ptr(), null(), false, -1.0,).x;
        let hit_sz = ImVec2_c { x: tw + 10.0 + label_w, y: row_h, };

        igInvisibleButton(label_cstr.as_ptr(), hit_sz, 0,);
        let clicked = igIsItemClicked(0,);
        if clicked
        {
            *v = !*v;
        }

        let hovered = igIsItemHovered(0,);
        let t_t = anim(id, if *v { 1.0 } else { 0.0 }, 14.0,);
        let hov_t = anim(id + 0xC000, if hovered { 1.0 } else { 0.0 }, 9.0,);

        let dl = igGetWindowDrawList();
        let cy = pos.y + (row_h - th) * 0.5;
        let mn = ImVec2_c { x: pos.x, y: cy, };
        let mx = ImVec2_c { x: pos.x + tw, y: cy + th, };

        if t_t > 0.02
        {
            glow_rect(dl, mn, mx, color::ACCENT, 10.0, t_t * 0.22,);
        }

        let track_off = color::BG_WIDGET;
        let track_on = ImVec4_c { x: 0.02, y: 0.40, z: 0.44, w: 1.0, };
        let track = lerp4(track_off, track_on, t_t,);
        let track = lerp4(
            track,
            lerp4(
                ImVec4_c { x: 0.17, y: 0.17, z: 0.22, w: 1.0, },
                ImVec4_c { x: 0.04, y: 0.50, z: 0.54, w: 1.0, },
                t_t,
            ),
            hov_t * 0.28,
        );
        ImDrawList_AddRectFilled(dl, mn, mx, u32(track,), tr, 0,);

        ImDrawList_AddRect(dl, mn, mx, u32(alpha(color::ACCENT, 0.22 + t_t * 0.50 + hov_t * 0.14,),), tr, 0, 1.1,);

        let pad = 3.2;
        let kr = tr - pad;
        let kx = pos.x + tr + (tw - th) * t_t;
        let ky = cy + tr;

        ImDrawList_AddCircleFilled(dl, ImVec2_c { x: kx + 1.0, y: ky + 1.0, }, kr, im_col32(0, 0, 0, 80,), 0,);

        let knob_off = ImVec4_c { x: 0.50, y: 0.50, z: 0.56, w: 1.0, };
        ImDrawList_AddCircleFilled(
            dl,
            ImVec2_c { x: kx, y: ky, },
            kr,
            u32(lerp4(knob_off, color::ACCENT_BRIGHT, t_t,),),
            0,
        );

        if t_t > 0.02
        {
            ImDrawList_AddCircle(
                dl,
                ImVec2_c { x: kx, y: ky, },
                kr + 3.0,
                u32(alpha(color::ACCENT, t_t * 0.38,),),
                0,
                1.2,
            );
        }

        let ty = pos.y + (row_h - igGetTextLineHeight()) * 0.5;
        let tc = lerp4(color::TEXT_MID, color::TEXT_HIGH, t_t * 0.45 + hov_t * 0.55,);
        ImDrawList_AddText_Vec2(dl, ImVec2_c { x: pos.x + tw + 9.0, y: ty, }, u32(tc,), label_cstr.as_ptr(), null(),);

        clicked
    }
}

pub fn slider_float(label: &str, v: &mut f32, v_min: f32, v_max: f32, fmt: &str, height: f32,) -> bool {
    unsafe {
        let label_cstr = cstr(label,);
        let _fmt_cstr = cstr(fmt,);
        let id = igGetID_Str(label_cstr.as_ptr(),);
        let pos = igGetCursorScreenPos();
        let w = igGetContentRegionAvail().x;

        igInvisibleButton(label_cstr.as_ptr(), ImVec2_c { x: w, y: height, }, 0,);
        let hovered = igIsItemHovered(0,);
        let active = igIsItemActive();

        if active
        {
            let mx = (*igGetIO_Nil()).MousePos.x;
            let t = (mx - pos.x) / w;
            *v = v_min + (v_max - v_min) * im_clamp_f32(t, 0.0, 1.0,);
        }

        let hov_t = anim(id, if hovered { 1.0 } else { 0.0 }, 10.0,);
        let act_t = anim(id + 0xD000, if active { 1.0 } else { 0.0 }, 18.0,);

        let frac = im_clamp_f32((*v - v_min) / (v_max - v_min), 0.0, 1.0,);

        let dl = igGetWindowDrawList();

        let label_y = pos.y + 3.0;
        let label_tc = lerp4(color::TEXT_MID, color::TEXT_HIGH, hov_t * 0.6 + act_t * 0.4,);
        ImDrawList_AddText_Vec2(dl, ImVec2_c { x: pos.x, y: label_y, }, u32(label_tc,), label_cstr.as_ptr(), null(),);

        let val_str = format!("{:.2}", *v);
        let val_cstr = cstr(&val_str,);
        let val_w = igCalcTextSize(val_cstr.as_ptr(), null(), false, -1.0,).x;
        let val_tc = lerp4(color::TEXT_MID, color::ACCENT_BRIGHT, act_t * 0.80 + hov_t * 0.30,);
        ImDrawList_AddText_Vec2(
            dl,
            ImVec2_c { x: pos.x + w - val_w, y: label_y, },
            u32(val_tc,),
            val_cstr.as_ptr(),
            null(),
        );

        let track_h = 5.0;
        let track_y = pos.y + height - track_h - 4.0;
        let track_r = track_h * 0.5;
        let tmn = ImVec2_c { x: pos.x, y: track_y, };
        let tmx = ImVec2_c { x: pos.x + w, y: track_y + track_h, };

        ImDrawList_AddRectFilled(dl, tmn, tmx, u32(color::BG_WIDGET,), track_r, 0,);
        ImDrawList_AddRect(dl, tmn, tmx, u32(alpha(color::ACCENT, 0.18 + hov_t * 0.12,),), track_r, 0, 0.8,);

        let fill_x = pos.x + w * frac;
        if fill_x - pos.x > track_r * 2.0
        {
            let fill_c = lerp4(color::ACCENT_DIM, color::ACCENT, hov_t * 0.55 + act_t * 0.45,);
            ImDrawList_AddRectFilled(dl, tmn, ImVec2_c { x: fill_x, y: track_y + track_h, }, u32(fill_c,), track_r, 0,);

            let t = igGetTime() as f32;
            let shim_t_val = (t * 1.6) % 2.0;
            let shim_nx = (shim_t_val - 0.3) * (fill_x - pos.x);
            let shim_w = (fill_x - pos.x) * 0.18;
            ImDrawList_PushClipRect(dl, tmn, ImVec2_c { x: fill_x, y: track_y + track_h, }, true,);
            ImDrawList_AddRectFilled(
                dl,
                ImVec2_c { x: pos.x + shim_nx, y: track_y, },
                ImVec2_c { x: pos.x + shim_nx + shim_w, y: track_y + track_h, },
                u32(alpha(color::ACCENT_BRIGHT, 0.22,),),
                track_r,
                0,
            );
            ImDrawList_PopClipRect(dl,);
        }

        let grab_r = 7.5 + act_t * 2.2 + hov_t * 1.4;
        let grab_x = pos.x + frac * w;
        let grab_y = track_y + track_h * 0.5;

        if hov_t > 0.01 || act_t > 0.01
        {
            glow_rect(
                dl,
                ImVec2_c { x: grab_x - grab_r, y: grab_y - grab_r, },
                ImVec2_c { x: grab_x + grab_r, y: grab_y + grab_r, },
                color::ACCENT,
                grab_r * 1.5,
                hov_t * 0.20 + act_t * 0.28,
            );
        }

        ImDrawList_AddCircleFilled(
            dl,
            ImVec2_c { x: grab_x + 1.0, y: grab_y + 1.5, },
            grab_r,
            im_col32(0, 0, 0, 70,),
            0,
        );

        let grab_c = lerp4(color::ACCENT, color::ACCENT_BRIGHT, act_t * 0.75 + hov_t * 0.25,);
        ImDrawList_AddCircleFilled(dl, ImVec2_c { x: grab_x, y: grab_y, }, grab_r, u32(grab_c,), 0,);

        let ring_a = hov_t * 0.38 + act_t * 0.55;
        if ring_a > 0.01
        {
            ImDrawList_AddCircle(
                dl,
                ImVec2_c { x: grab_x, y: grab_y, },
                grab_r + 3.5,
                u32(alpha(color::ACCENT, ring_a,),),
                0,
                1.3,
            );
        }

        ImDrawList_AddCircleFilled(
            dl,
            ImVec2_c { x: grab_x - grab_r * 0.28, y: grab_y - grab_r * 0.28, },
            grab_r * 0.26,
            u32(alpha(color::ACCENT_BRIGHT, 0.38 + hov_t * 0.28,),),
            0,
        );

        active
    }
}

pub fn slider_int(label: &str, v: &mut i32, v_min: i32, v_max: i32,) -> bool {
    let mut fv = *v as f32;
    let changed = slider_float(label, &mut fv, v_min as f32, v_max as f32, "%.0f", 34.0,);
    *v = fv as i32;
    changed
}

pub fn section_header(text: &str, accent: ImVec4_c,) {
    unsafe {
        let pos = igGetCursorScreenPos();
        let avail = igGetContentRegionAvail().x;
        let lh = igGetTextLineHeight();

        let dl = igGetWindowDrawList();
        let text_cstr = cstr(text,);

        ImDrawList_AddRectFilled(
            dl,
            ImVec2_c { x: pos.x, y: pos.y + 2.0, },
            ImVec2_c { x: pos.x + 3.0, y: pos.y + lh - 2.0, },
            u32(accent,),
            2.0,
            0,
        );

        ImDrawList_AddText_Vec2(
            dl,
            ImVec2_c { x: pos.x + 10.0, y: pos.y, },
            u32(color::TEXT_HIGH,),
            text_cstr.as_ptr(),
            null(),
        );

        let text_right = pos.x + 14.0 + igCalcTextSize(text_cstr.as_ptr(), null(), false, -1.0,).x;
        let rule_y = pos.y + lh * 0.5;
        ImDrawList_AddLine(
            dl,
            ImVec2_c { x: text_right + 8.0, y: rule_y, },
            ImVec2_c { x: pos.x + avail, y: rule_y, },
            u32(alpha(accent, 0.18,),),
            1.0,
        );

        igDummy(ImVec2_c { x: avail, y: lh + 8.0, },);
    }
}

pub fn progress_bar(fraction: f32, size: ImVec2_c, fill_color: ImVec4_c,) {
    unsafe {
        let pos = igGetCursorScreenPos();
        let w = if size.x < 0.0 { igGetContentRegionAvail().x } else { size.x };
        let h = size.y;
        let r = h * 0.5;
        let fraction = im_clamp_f32(fraction, 0.0, 1.0,);

        let dl = igGetWindowDrawList();

        ImDrawList_AddRectFilled(dl, pos, ImVec2_c { x: pos.x + w, y: pos.y + h, }, u32(color::BG_WIDGET,), r, 0,);
        ImDrawList_AddRect(
            dl,
            pos,
            ImVec2_c { x: pos.x + w, y: pos.y + h, },
            u32(alpha(fill_color, 0.20,),),
            r,
            0,
            0.8,
        );

        let fw = w * fraction;
        if fw > r * 2.0
        {
            ImDrawList_AddRectFilled(
                dl,
                pos,
                ImVec2_c { x: pos.x + fw, y: pos.y + h, },
                u32(alpha(fill_color, 0.85,),),
                r,
                0,
            );

            let t = igGetTime() as f32;
            let st = (t * 1.5) % 2.2;
            let sx = pos.x + (st - 0.3) * fw;
            let sw = fw * 0.15;
            ImDrawList_PushClipRect(dl, pos, ImVec2_c { x: pos.x + fw, y: pos.y + h, }, true,);
            ImDrawList_AddRectFilled(
                dl,
                ImVec2_c { x: sx, y: pos.y, },
                ImVec2_c { x: sx + sw, y: pos.y + h, },
                u32(alpha(color::ACCENT_BRIGHT, 0.22,),),
                r,
                0,
            );
            ImDrawList_PopClipRect(dl,);

            ImDrawList_AddRectFilled(
                dl,
                ImVec2_c { x: pos.x + fw - 3.0, y: pos.y, },
                ImVec2_c { x: pos.x + fw + 2.0, y: pos.y + h, },
                u32(alpha(fill_color, 0.55,),),
                1.0,
                0,
            );
        }

        igDummy(ImVec2_c { x: w, y: h, },);
    }
}

pub fn badge(text: &str, color_val: ImVec4_c,) {
    unsafe {
        let pos = igGetCursorScreenPos();
        let text_cstr = cstr(text,);
        let tsz = igCalcTextSize(text_cstr.as_ptr(), null(), false, -1.0,);
        let px = 7.0;
        let py = 2.5;
        let mn = pos;
        let mx = ImVec2_c { x: pos.x + tsz.x + px * 2.0, y: pos.y + tsz.y + py * 2.0, };

        let dl = igGetWindowDrawList();
        ImDrawList_AddRectFilled(dl, mn, mx, u32(alpha(color_val, 0.16,),), (mx.y - mn.y) * 0.5, 0,);
        ImDrawList_AddRect(dl, mn, mx, u32(alpha(color_val, 0.48,),), (mx.y - mn.y) * 0.5, 0, 1.0,);
        ImDrawList_AddText_Vec2(
            dl,
            ImVec2_c { x: pos.x + px, y: pos.y + py, },
            u32(color_val,),
            text_cstr.as_ptr(),
            null(),
        );

        igDummy(ImVec2_c { x: mx.x - mn.x, y: mx.y - mn.y, },);
    }
}

pub fn keybind_static(key: &str,) {
    unsafe {
        let pos = igGetCursorScreenPos();
        let key_cstr = cstr(key,);
        let tsz = igCalcTextSize(key_cstr.as_ptr(), null(), false, -1.0,);
        let px = 8.0;
        let py = 3.0;
        let mn = pos;
        let mx = ImVec2_c { x: pos.x + tsz.x + px * 2.0, y: pos.y + tsz.y + py * 2.0, };
        let rr = 5.0;

        let dl = igGetWindowDrawList();

        ImDrawList_AddRectFilled(dl, mn, mx, u32(color::BG_ELEVATE,), rr, 0,);
        ImDrawList_AddRectFilled(
            dl,
            ImVec2_c { x: mn.x + 1.0, y: mx.y - 2.0, },
            ImVec2_c { x: mx.x - 1.0, y: mx.y + 1.5, },
            im_col32(0, 0, 0, 80,),
            rr,
            0,
        );
        ImDrawList_AddRect(dl, mn, mx, u32(alpha(color::TEXT_MID, 0.35,),), rr, 0, 1.0,);
        ImDrawList_AddText_Vec2(
            dl,
            ImVec2_c { x: pos.x + px, y: pos.y + py, },
            u32(color::TEXT_MID,),
            key_cstr.as_ptr(),
            null(),
        );

        igDummy(ImVec2_c { x: mx.x - mn.x, y: mx.y - mn.y + 2.0, },);
    }
}

pub fn status_dot(label: &str, dot_color: ImVec4_c,) {
    unsafe {
        let pos = igGetCursorScreenPos();
        let r = 4.5;
        let lh = igGetTextLineHeight();
        let cy = pos.y + lh * 0.5;
        let label_cstr = cstr(label,);

        let dl = igGetWindowDrawList();

        ImDrawList_AddCircleFilled(dl, ImVec2_c { x: pos.x + r, y: cy, }, r + 2.5, u32(alpha(dot_color, 0.18,),), 0,);
        ImDrawList_AddCircleFilled(dl, ImVec2_c { x: pos.x + r, y: cy, }, r, u32(dot_color,), 0,);
        ImDrawList_AddText_Vec2(
            dl,
            ImVec2_c { x: pos.x + r * 2.0 + 7.0, y: pos.y, },
            u32(color::TEXT_MID,),
            label_cstr.as_ptr(),
            null(),
        );

        igDummy(ImVec2_c { x: r * 2.0 + 7.0 + igCalcTextSize(label_cstr.as_ptr(), null(), false, -1.0,).x, y: lh, },);
    }
}

pub fn separator(color_val: ImVec4_c, alpha_val: f32,) {
    unsafe {
        let pos = igGetCursorScreenPos();
        let w = igGetContentRegionAvail().x;
        let cy = pos.y + 3.0;

        let dl = igGetWindowDrawList();
        ImDrawList_AddLine(
            dl,
            ImVec2_c { x: pos.x, y: cy, },
            ImVec2_c { x: pos.x + w, y: cy, },
            u32(alpha(color_val, alpha_val,),),
            1.0,
        );

        igDummy(ImVec2_c { x: w, y: 6.0, },);
    }
}

pub struct Toast {
    msg:      [u8; 256],
    timer:    f32,
    color:    ImVec4_c,
    max_time: f32,
}

impl Toast {
    pub fn new() -> Self {
        Self { msg: [0; 256], timer: 0.0, color: color::ACCENT, max_time: 2.5, }
    }

    pub fn show(&mut self, message: &str, duration: f32, c: ImVec4_c,) {
        let bytes = message.as_bytes();
        let len = bytes.len().min(255,);
        self.msg[..len].copy_from_slice(&bytes[..len],);
        self.msg[len] = 0;
        self.timer = duration;
        self.max_time = duration;
        self.color = c;
    }

    pub fn render(&mut self, win_pos: ImVec2_c, win_size: ImVec2_c,) {
        unsafe {
            if self.timer <= 0.0
            {
                return;
            }
            self.timer -= (*igGetIO_Nil()).DeltaTime;

            let life_t = self.timer / self.max_time;
            let alpha_val = if life_t < 0.2 { life_t / 0.2 } else { 1.0 };

            let toast_w = im_min_f32(280.0, win_size.x - 32.0,);
            let toast_h = 44.0;
            let pad = 14.0;

            let mut bx = win_pos.x + win_size.x - toast_w - pad;
            let by = win_pos.y + win_size.y - toast_h - pad;

            let slide_x = (1.0 - im_min_f32(1.0, (self.max_time - self.timer) / 0.25,)) * 50.0;
            bx += slide_x;

            let fg = igGetForegroundDrawList_Nil();
            let msg_ptr = self.msg.as_ptr() as *const i8;

            ImDrawList_AddRectFilled(
                fg,
                ImVec2_c { x: bx + 2.0, y: by + 2.0, },
                ImVec2_c { x: bx + toast_w + 2.0, y: by + toast_h + 2.0, },
                im_col32(0, 0, 0, (60.0 * alpha_val) as u8,),
                9.0,
                0,
            );

            ImDrawList_AddRectFilled(
                fg,
                ImVec2_c { x: bx, y: by, },
                ImVec2_c { x: bx + toast_w, y: by + toast_h, },
                u32(alpha(ImVec4_c { x: 0.08, y: 0.08, z: 0.11, w: 1.0, }, alpha_val * 0.97,),),
                9.0,
                0,
            );

            ImDrawList_AddRectFilled(
                fg,
                ImVec2_c { x: bx, y: by + 6.0, },
                ImVec2_c { x: bx + 3.5, y: by + toast_h - 6.0, },
                u32(alpha(self.color, alpha_val,),),
                2.0,
                0,
            );

            ImDrawList_AddRect(
                fg,
                ImVec2_c { x: bx, y: by, },
                ImVec2_c { x: bx + toast_w, y: by + toast_h, },
                u32(alpha(self.color, alpha_val * 0.35,),),
                9.0,
                0,
                1.0,
            );

            let tsz = igCalcTextSize(msg_ptr, null(), false, -1.0,);
            ImDrawList_AddText_Vec2(
                fg,
                ImVec2_c { x: bx + 14.0, y: by + (toast_h - tsz.y) * 0.5, },
                u32(alpha(color::TEXT_HIGH, alpha_val,),),
                msg_ptr,
                null(),
            );
        }
    }
}

pub fn begin_combo(label: &str, preview_value: &str,) -> bool {
    unsafe {
        let label_cstr = cstr(label,);
        let preview_cstr = cstr(preview_value,);

        igPushStyleVar_Vec2(ImGuiStyleVar__ImGuiStyleVar_FramePadding, ImVec2_c { x: 10.0, y: 7.0, },);
        igPushStyleVar_Float(ImGuiStyleVar__ImGuiStyleVar_FrameRounding, 7.0,);
        igPushStyleColor_U32(ImGuiCol__ImGuiCol_FrameBg, u32(color::BG_WIDGET,),);
        igPushStyleColor_U32(ImGuiCol__ImGuiCol_FrameBgHovered, u32(ImVec4_c { x: 0.18, y: 0.18, z: 0.24, w: 1.0, },),);
        igPushStyleColor_U32(ImGuiCol__ImGuiCol_Button, u32(color::BG_WIDGET,),);
        igPushStyleColor_U32(ImGuiCol__ImGuiCol_ButtonHovered, u32(alpha(color::ACCENT, 0.28,),),);
        igPushStyleColor_U32(ImGuiCol__ImGuiCol_PopupBg, u32(ImVec4_c { x: 0.08, y: 0.08, z: 0.12, w: 0.98, },),);
        let open = igBeginCombo(label_cstr.as_ptr(), preview_cstr.as_ptr(), 0,);
        igPopStyleColor(5,);
        igPopStyleVar(2,);
        open
    }
}

pub fn end_combo() {
    unsafe {
        igEndCombo();
    }
}

pub fn input_text(label: &str, buf: &mut [u8], flags: ImGuiInputTextFlags,) -> bool {
    unsafe {
        let label_cstr = cstr(label,);
        igPushStyleVar_Float(ImGuiStyleVar__ImGuiStyleVar_FrameRounding, 7.0,);
        igPushStyleVar_Vec2(ImGuiStyleVar__ImGuiStyleVar_FramePadding, ImVec2_c { x: 10.0, y: 7.0, },);
        igPushStyleColor_U32(ImGuiCol__ImGuiCol_FrameBg, u32(color::BG_WIDGET,),);
        igPushStyleColor_U32(ImGuiCol__ImGuiCol_FrameBgHovered, u32(ImVec4_c { x: 0.19, y: 0.19, z: 0.25, w: 1.0, },),);
        igPushStyleColor_U32(ImGuiCol__ImGuiCol_FrameBgActive, u32(ImVec4_c { x: 0.04, y: 0.35, z: 0.38, w: 0.35, },),);
        let ch = igInputText(label_cstr.as_ptr(), buf.as_mut_ptr() as *mut i8, buf.len(), flags, None, null_mut(),);
        igPopStyleColor(3,);
        igPopStyleVar(2,);
        ch
    }
}

pub fn begin_panel(str_id: &str, size: ImVec2_c,) -> bool {
    unsafe {
        let id_cstr = cstr(str_id,);
        let id = igGetID_Str(id_cstr.as_ptr(),);
        igPushStyleVar_Float(ImGuiStyleVar__ImGuiStyleVar_ChildRounding, 10.0,);
        igPushStyleColor_U32(ImGuiCol__ImGuiCol_ChildBg, u32(color::BG_BASE,),);
        igPushStyleColor_U32(ImGuiCol__ImGuiCol_Border, u32(alpha(color::ACCENT, 0.22,),),);
        igPushStyleVar_Float(ImGuiStyleVar__ImGuiStyleVar_ChildBorderSize, 1.0,);
        let vis = igBeginChildEx(id_cstr.as_ptr(), id, size, ImGuiChildFlags__ImGuiChildFlags_Borders, 0,);
        igPopStyleColor(2,);
        igPopStyleVar(2,);
        vis
    }
}

pub fn end_panel() {
    unsafe {
        igEndChild();
    }
}

pub struct TabBar {
    tabs:    Vec<(String, ImVec4_c,),>,
    _str_id: String,
    height:  f32,
    _ind_x:  f32,
    _ind_w:  f32,
    _init:   bool,
}

impl TabBar {
    pub fn new(id: &str, h: f32,) -> Self {
        Self { tabs: Vec::new(), _str_id: id.to_string(), height: h, _ind_x: 0.0, _ind_w: 0.0, _init: false, }
    }

    pub fn add(&mut self, label: &str, accent: ImVec4_c,) {
        self.tabs.push((label.to_string(), accent,),);
    }

    pub fn render(&mut self, selected: &mut i32,) -> bool {
        if self.tabs.is_empty()
        {
            return false;
        }

        if *selected < 0
        {
            *selected = 0;
        }
        if *selected >= self.tabs.len() as i32
        {
            *selected = self.tabs.len() as i32 - 1;
        }

        unsafe {
            let pos = igGetCursorScreenPos();
            let avail = igGetContentRegionAvail().x;
            let dl = igGetWindowDrawList();
            let mut changed = false;

            let bar_top = ImVec4_c { x: 0.075, y: 0.075, z: 0.100, w: 1.0, };
            let bar_bot = ImVec4_c { x: 0.065, y: 0.065, z: 0.088, w: 1.0, };
            ImDrawList_AddRectFilledMultiColor(
                dl,
                pos,
                ImVec2_c { x: pos.x + avail, y: pos.y + self.height, },
                u32(bar_top,),
                u32(bar_top,),
                u32(bar_bot,),
                u32(bar_bot,),
            );

            let n = self.tabs.len() as i32;
            let tw = avail / n as f32;

            igSetCursorScreenPos(pos,);

            for i in 0..n
            {
                let (ref label, ref accent_col,) = self.tabs[i as usize];
                let ac = if accent_col.w > 0.01 { *accent_col } else { color::ACCENT };

                let tx0 = pos.x + i as f32 * tw;
                let tx1 = tx0 + tw;

                let label_cstr = cstr(label.as_str(),);
                let tid = igGetID_Str(label_cstr.as_ptr(),);
                igSetCursorScreenPos(ImVec2_c { x: tx0, y: pos.y, },);
                igInvisibleButton(label_cstr.as_ptr(), ImVec2_c { x: tw, y: self.height, }, 0,);

                let hov = igIsItemHovered(0,);
                let clk = igIsItemClicked(0,);
                if clk && *selected != i
                {
                    *selected = i;
                    changed = true;
                }

                let h_t = anim(tid, if hov { 1.0 } else { 0.0 }, 9.0,);
                let sel_t = anim(tid + 0xE000, if *selected == i { 1.0 } else { 0.0 }, 17.0,);

                let bg_alpha = h_t * 0.09 + sel_t * 0.07;
                if bg_alpha > 0.005
                {
                    ImDrawList_AddRectFilled(
                        dl,
                        ImVec2_c { x: tx0, y: pos.y, },
                        ImVec2_c { x: tx1, y: pos.y + self.height, },
                        u32(alpha(ac, bg_alpha,),),
                        0.0,
                        0,
                    );
                }

                if i < n - 1
                {
                    ImDrawList_AddLine(
                        dl,
                        ImVec2_c { x: tx1, y: pos.y + self.height * 0.18, },
                        ImVec2_c { x: tx1, y: pos.y + self.height * 0.82, },
                        u32(alpha(color::TEXT_LOW, 0.22 + h_t * 0.08,),),
                        1.0,
                    );
                }

                let tsz = igCalcTextSize(label_cstr.as_ptr(), null(), false, -1.0,);
                let lx = tx0 + (tw - tsz.x) * 0.5;
                let ly = pos.y + (self.height - tsz.y) * 0.5;

                let tc = lerp4(color::TEXT_LOW, lerp4(color::TEXT_HIGH, ac, sel_t * 0.45,), sel_t * 0.8 + h_t * 0.5,);
                ImDrawList_AddText_Vec2(dl, ImVec2_c { x: lx, y: ly, }, u32(tc,), label_cstr.as_ptr(), null(),);
            }

            let sep_y = pos.y + self.height - 0.5;
            ImDrawList_AddLine(
                dl,
                ImVec2_c { x: pos.x, y: sep_y, },
                ImVec2_c { x: pos.x + avail, y: sep_y, },
                u32(alpha(color::ACCENT, 0.12,),),
                1.0,
            );

            {
                let (_, ref accent_col,) = self.tabs[*selected as usize];
                let ac = if accent_col.w > 0.01 { *accent_col } else { color::ACCENT };

                let tgt_x = *selected as f32 * tw;
                let tgt_w = tw;

                if !self._init
                {
                    self._ind_x = tgt_x;
                    self._ind_w = tgt_w;
                    self._init = true;
                }
                else
                {
                    let dt = (*igGetIO_Nil()).DeltaTime;
                    let spd = 1.0 - (-20.0 * dt).exp();
                    self._ind_x += (tgt_x - self._ind_x) * spd;
                    self._ind_w += (tgt_w - self._ind_w) * spd;
                }

                let ix = pos.x + self._ind_x;
                let iw = self._ind_w;
                let iy = pos.y + self.height - 2.5;
                let inset = iw * 0.08;
                let i_rad = 2.0;

                ImDrawList_AddRectFilled(
                    dl,
                    ImVec2_c { x: ix + inset - 6.0, y: iy - 4.0, },
                    ImVec2_c { x: ix + iw - inset + 6.0, y: iy + 4.5, },
                    u32(alpha(ac, 0.20,),),
                    i_rad + 3.0,
                    0,
                );

                ImDrawList_AddRectFilled(
                    dl,
                    ImVec2_c { x: ix + inset, y: iy, },
                    ImVec2_c { x: ix + iw - inset, y: iy + 2.5, },
                    u32(alpha(ac, 0.92,),),
                    i_rad,
                    0,
                );

                let cx_val = ix + iw * 0.5;
                let hw = (iw - inset * 2.0) * 0.30;
                ImDrawList_AddRectFilled(
                    dl,
                    ImVec2_c { x: cx_val - hw, y: iy, },
                    ImVec2_c { x: cx_val + hw, y: iy + 2.5, },
                    u32(alpha(color::ACCENT_BRIGHT, 0.60,),),
                    i_rad,
                    0,
                );
            }

            igSetCursorScreenPos(ImVec2_c { x: pos.x, y: pos.y + self.height + 8.0, },);
            changed
        }
    }
}

pub fn info_tip(tip_text: &str, radius: f32, color_val: ImVec4_c,) {
    unsafe {
        let tip_cstr = cstr(tip_text,);
        let id = igGetID_Str(tip_cstr.as_ptr(),);
        let btn_id = format!("##tip{}\0", id);
        let btn_cstr = cstr(btn_id.as_str(),);

        let pos = igGetCursorScreenPos();
        let r = radius;
        let c = ImVec2_c { x: pos.x + r, y: pos.y + r, };
        let dl = igGetWindowDrawList();

        igInvisibleButton(btn_cstr.as_ptr(), ImVec2_c { x: r * 2.0, y: r * 2.0, }, 0,);
        let hov = igIsItemHovered(0,);
        let h_t = anim(id + 0xF100, if hov { 1.0 } else { 0.0 }, 14.0,);

        if h_t > 0.01
        {
            ImDrawList_AddCircleFilled(dl, c, r + 6.0, u32(alpha(color_val, h_t * 0.13,),), 0,);
        }

        let bg =
            lerp4(color::BG_WIDGET, lerp4(ImVec4_c { x: 0.02, y: 0.30, z: 0.34, w: 1.0, }, color_val, 0.45,), h_t,);
        ImDrawList_AddCircleFilled(dl, c, r, u32(bg,), 0,);

        ImDrawList_AddCircle(dl, c, r, u32(alpha(color_val, 0.32 + h_t * 0.58,),), 0, 1.3,);

        {
            let ic = lerp4(color::TEXT_MID, color::ACCENT_BRIGHT, h_t * 0.85,);
            let icu = u32(ic,);
            let dot_r = r * 0.155;
            let stem_w = r * 0.145;

            ImDrawList_AddCircleFilled(dl, ImVec2_c { x: c.x, y: c.y - r * 0.34, }, dot_r, icu, 0,);
            ImDrawList_AddRectFilled(
                dl,
                ImVec2_c { x: c.x - stem_w, y: c.y - r * 0.10, },
                ImVec2_c { x: c.x + stem_w, y: c.y + r * 0.44, },
                icu,
                stem_w,
                0,
            );
        }

        if hov
        {
            let alpha_val = im_clamp_f32(h_t * 6.0, 0.05, 1.0,);

            igPushStyleVar_Vec2(ImGuiStyleVar__ImGuiStyleVar_WindowPadding, ImVec2_c { x: 15.0, y: 12.0, },);
            igPushStyleVar_Float(ImGuiStyleVar__ImGuiStyleVar_WindowRounding, 9.0,);
            igPushStyleVar_Float(ImGuiStyleVar__ImGuiStyleVar_PopupBorderSize, 1.0,);
            igPushStyleColor_U32(
                ImGuiCol__ImGuiCol_PopupBg,
                u32(alpha(ImVec4_c { x: 0.04, y: 0.05, z: 0.08, w: 1.0, }, alpha_val * 0.97,),),
            );
            igPushStyleColor_U32(ImGuiCol__ImGuiCol_Border, u32(alpha(color_val, alpha_val * 0.42,),),);
            igPushStyleColor_U32(ImGuiCol__ImGuiCol_Text, u32(alpha(color::TEXT_HIGH, alpha_val,),),);

            igSetNextWindowSizeConstraints(
                ImVec2_c { x: 80.0, y: 0.0, },
                ImVec2_c { x: 300.0, y: 400.0, },
                None,
                null_mut(),
            );
            igBeginTooltip();

            {
                let wp = igGetWindowPos();
                let ws = igGetWindowSize();
                let bar_t = wp.y + 7.0;
                let bar_b = wp.y + ws.y - 7.0;
                ImDrawList_AddRectFilled(
                    igGetWindowDrawList(),
                    ImVec2_c { x: wp.x + 1.5, y: bar_t, },
                    ImVec2_c { x: wp.x + 4.5, y: bar_b, },
                    u32(alpha(color_val, alpha_val * 0.88,),),
                    2.0,
                    0,
                );
            }

            let cur = igGetCursorScreenPos();
            igSetCursorScreenPos(ImVec2_c { x: cur.x + 8.0, y: cur.y, },);
            igTextUnformatted(tip_cstr.as_ptr(), null(),);

            igEndTooltip();
            igPopStyleColor(3,);
            igPopStyleVar(3,);
        }
    }
}

pub struct KeybindRecorder {
    pub main_key: ImGuiKey,
    pub ctrl:     bool,
    pub shift:    bool,
    pub alt:      bool,
    recording:    bool,
    _pulse_t:     f32,
    _flash_t:     f32,
    _clear_fl_t:  f32,
    _saved_key:   ImGuiKey,
    _saved_ctrl:  bool,
    _saved_shift: bool,
    _saved_alt:   bool,
}

impl KeybindRecorder {
    pub fn new() -> Self {
        Self {
            main_key:     ImGuiKey_ImGuiKey_None,
            ctrl:         false,
            shift:        false,
            alt:          false,
            recording:    false,
            _pulse_t:     0.0,
            _flash_t:     0.0,
            _clear_fl_t:  0.0,
            _saved_key:   ImGuiKey_ImGuiKey_None,
            _saved_ctrl:  false,
            _saved_shift: false,
            _saved_alt:   false,
        }
    }

    pub fn is_set(&self,) -> bool {
        self.main_key != ImGuiKey_ImGuiKey_None
    }

    pub fn clear(&mut self,) {
        self.main_key = ImGuiKey_ImGuiKey_None;
        self.ctrl = false;
        self.shift = false;
        self.alt = false;
    }

    pub fn get_display_string(&self,) -> String {
        if !self.is_set()
        {
            return "Not bound".to_string();
        }
        let mut s = String::new();
        if self.ctrl
        {
            s.push_str("Ctrl+",);
        }
        if self.shift
        {
            s.push_str("Shift+",);
        }
        if self.alt
        {
            s.push_str("Alt+",);
        }
        s.push_str(Self::key_name(self.main_key,),);
        s
    }

    pub fn render(&mut self, label: &str, row_height: f32,) -> bool {
        unsafe {
            let label_cstr = cstr(label,);
            let id = igGetID_Str(label_cstr.as_ptr(),);
            let pos = igGetCursorScreenPos();
            let avail = igGetContentRegionAvail().x;
            let dl = igGetWindowDrawList();
            let mut changed = false;

            let dt = (*igGetIO_Nil()).DeltaTime;
            self._pulse_t += dt * 3.4;
            if self._flash_t > 0.0
            {
                self._flash_t = im_max_f32(0.0, self._flash_t - dt * 3.2,);
            }
            if self._clear_fl_t > 0.0
            {
                self._clear_fl_t = im_max_f32(0.0, self._clear_fl_t - dt * 3.2,);
            }

            igInvisibleButton(label_cstr.as_ptr(), ImVec2_c { x: avail, y: row_height, }, 0,);
            let row_hov = igIsItemHovered(0,);
            let row_clk = igIsItemClicked(0,);

            let hov_t = anim(id, if row_hov { 1.0 } else { 0.0 }, 9.0,);
            let rec_t = anim(id + 0xF200, if self.recording { 1.0 } else { 0.0 }, 13.0,);

            if row_clk && !self.recording
            {
                self._saved_key = self.main_key;
                self._saved_ctrl = self.ctrl;
                self._saved_shift = self.shift;
                self._saved_alt = self.alt;
                self.recording = true;
            }

            if self.recording
            {
                let kc = (*igGetIO_Nil()).KeyCtrl;
                let ks = (*igGetIO_Nil()).KeyShift;
                let ka = (*igGetIO_Nil()).KeyAlt;

                if igIsKeyPressed_Bool(ImGuiKey_ImGuiKey_Escape, false,)
                {
                    self.main_key = self._saved_key;
                    self.ctrl = self._saved_ctrl;
                    self.shift = self._saved_shift;
                    self.alt = self._saved_alt;
                    self.recording = false;
                }
                else if igIsKeyPressed_Bool(ImGuiKey_ImGuiKey_Backspace, false,)
                    || igIsKeyPressed_Bool(ImGuiKey_ImGuiKey_Delete, false,)
                {
                    self.clear();
                    self.recording = false;
                    changed = true;
                    self._clear_fl_t = 1.0;
                }
                else
                {
                    let k_table: [ImGuiKey; 92] = [
                        ImGuiKey_ImGuiKey_A,
                        ImGuiKey_ImGuiKey_B,
                        ImGuiKey_ImGuiKey_C,
                        ImGuiKey_ImGuiKey_D,
                        ImGuiKey_ImGuiKey_E,
                        ImGuiKey_ImGuiKey_F,
                        ImGuiKey_ImGuiKey_G,
                        ImGuiKey_ImGuiKey_H,
                        ImGuiKey_ImGuiKey_I,
                        ImGuiKey_ImGuiKey_J,
                        ImGuiKey_ImGuiKey_K,
                        ImGuiKey_ImGuiKey_L,
                        ImGuiKey_ImGuiKey_M,
                        ImGuiKey_ImGuiKey_N,
                        ImGuiKey_ImGuiKey_O,
                        ImGuiKey_ImGuiKey_P,
                        ImGuiKey_ImGuiKey_Q,
                        ImGuiKey_ImGuiKey_R,
                        ImGuiKey_ImGuiKey_S,
                        ImGuiKey_ImGuiKey_T,
                        ImGuiKey_ImGuiKey_U,
                        ImGuiKey_ImGuiKey_V,
                        ImGuiKey_ImGuiKey_W,
                        ImGuiKey_ImGuiKey_X,
                        ImGuiKey_ImGuiKey_Y,
                        ImGuiKey_ImGuiKey_Z,
                        ImGuiKey_ImGuiKey_0,
                        ImGuiKey_ImGuiKey_1,
                        ImGuiKey_ImGuiKey_2,
                        ImGuiKey_ImGuiKey_3,
                        ImGuiKey_ImGuiKey_4,
                        ImGuiKey_ImGuiKey_5,
                        ImGuiKey_ImGuiKey_6,
                        ImGuiKey_ImGuiKey_7,
                        ImGuiKey_ImGuiKey_8,
                        ImGuiKey_ImGuiKey_9,
                        ImGuiKey_ImGuiKey_F1,
                        ImGuiKey_ImGuiKey_F2,
                        ImGuiKey_ImGuiKey_F3,
                        ImGuiKey_ImGuiKey_F4,
                        ImGuiKey_ImGuiKey_F5,
                        ImGuiKey_ImGuiKey_F6,
                        ImGuiKey_ImGuiKey_F7,
                        ImGuiKey_ImGuiKey_F8,
                        ImGuiKey_ImGuiKey_F9,
                        ImGuiKey_ImGuiKey_F10,
                        ImGuiKey_ImGuiKey_F11,
                        ImGuiKey_ImGuiKey_F12,
                        ImGuiKey_ImGuiKey_Tab,
                        ImGuiKey_ImGuiKey_Space,
                        ImGuiKey_ImGuiKey_Enter,
                        ImGuiKey_ImGuiKey_Insert,
                        ImGuiKey_ImGuiKey_Home,
                        ImGuiKey_ImGuiKey_End,
                        ImGuiKey_ImGuiKey_PageUp,
                        ImGuiKey_ImGuiKey_PageDown,
                        ImGuiKey_ImGuiKey_LeftArrow,
                        ImGuiKey_ImGuiKey_RightArrow,
                        ImGuiKey_ImGuiKey_UpArrow,
                        ImGuiKey_ImGuiKey_DownArrow,
                        ImGuiKey_ImGuiKey_Minus,
                        ImGuiKey_ImGuiKey_Equal,
                        ImGuiKey_ImGuiKey_LeftBracket,
                        ImGuiKey_ImGuiKey_RightBracket,
                        ImGuiKey_ImGuiKey_Backslash,
                        ImGuiKey_ImGuiKey_Semicolon,
                        ImGuiKey_ImGuiKey_Apostrophe,
                        ImGuiKey_ImGuiKey_GraveAccent,
                        ImGuiKey_ImGuiKey_Comma,
                        ImGuiKey_ImGuiKey_Period,
                        ImGuiKey_ImGuiKey_Slash,
                        ImGuiKey_ImGuiKey_Keypad0,
                        ImGuiKey_ImGuiKey_Keypad1,
                        ImGuiKey_ImGuiKey_Keypad2,
                        ImGuiKey_ImGuiKey_Keypad3,
                        ImGuiKey_ImGuiKey_Keypad4,
                        ImGuiKey_ImGuiKey_Keypad5,
                        ImGuiKey_ImGuiKey_Keypad6,
                        ImGuiKey_ImGuiKey_Keypad7,
                        ImGuiKey_ImGuiKey_Keypad8,
                        ImGuiKey_ImGuiKey_Keypad9,
                        ImGuiKey_ImGuiKey_KeypadAdd,
                        ImGuiKey_ImGuiKey_KeypadSubtract,
                        ImGuiKey_ImGuiKey_KeypadMultiply,
                        ImGuiKey_ImGuiKey_KeypadDivide,
                        ImGuiKey_ImGuiKey_KeypadDecimal,
                        ImGuiKey_ImGuiKey_KeypadEnter,
                        ImGuiKey_ImGuiKey_PrintScreen,
                        ImGuiKey_ImGuiKey_Pause,
                        ImGuiKey_ImGuiKey_CapsLock,
                        ImGuiKey_ImGuiKey_NumLock,
                        ImGuiKey_ImGuiKey_ScrollLock,
                    ];

                    for &k in k_table.iter()
                    {
                        if igIsKeyPressed_Bool(k, false,)
                        {
                            self.main_key = k;
                            self.ctrl = kc;
                            self.shift = ks;
                            self.alt = ka;
                            self.recording = false;
                            changed = true;
                            self._flash_t = 1.0;
                            break;
                        }
                    }
                }
            }

            {
                let bg_blend = hov_t * 0.35 + rec_t * 0.55;
                let mut row_bg = lerp4(alpha(color::BG_WIDGET, 0.50,), alpha(color::ACCENT, 0.11,), bg_blend,);

                if self._flash_t > 0.0
                {
                    row_bg = lerp4(row_bg, alpha(color::ACCENT_BRIGHT, 0.15,), self._flash_t,);
                }
                if self._clear_fl_t > 0.0
                {
                    row_bg = lerp4(row_bg, alpha(color::DANGER, 0.14,), self._clear_fl_t,);
                }

                ImDrawList_AddRectFilled(
                    dl,
                    pos,
                    ImVec2_c { x: pos.x + avail, y: pos.y + row_height, },
                    u32(row_bg,),
                    7.0,
                    0,
                );
            }

            if rec_t > 0.01
            {
                let pulse = self._pulse_t.sin() * 0.5 + 0.5;
                let ba = 0.28 + pulse * 0.48 * rec_t;
                ImDrawList_AddRect(
                    dl,
                    pos,
                    ImVec2_c { x: pos.x + avail, y: pos.y + row_height, },
                    u32(alpha(color::ACCENT, ba,),),
                    7.0,
                    0,
                    1.3,
                );

                ImDrawList_AddRectFilled(
                    dl,
                    ImVec2_c { x: pos.x + 8.0, y: pos.y, },
                    ImVec2_c { x: pos.x + avail - 8.0, y: pos.y + 1.5, },
                    u32(alpha(color::ACCENT_BRIGHT, pulse * rec_t * 0.35,),),
                    1.0,
                    0,
                );
            }
            else if hov_t > 0.01
            {
                ImDrawList_AddRect(
                    dl,
                    pos,
                    ImVec2_c { x: pos.x + avail, y: pos.y + row_height, },
                    u32(alpha(color::ACCENT, hov_t * 0.24,),),
                    7.0,
                    0,
                    0.9,
                );
            }

            ImDrawList_AddRectFilled(
                dl,
                ImVec2_c { x: pos.x, y: pos.y + 8.0, },
                ImVec2_c { x: pos.x + 3.0, y: pos.y + row_height - 8.0, },
                u32(alpha(color::ACCENT, 0.28 + hov_t * 0.45 + rec_t * 0.35,),),
                2.0,
                0,
            );

            let text_y = pos.y + (row_height - igGetTextLineHeight()) * 0.5;
            let label_c = lerp4(color::TEXT_MID, color::TEXT_HIGH, hov_t * 0.55 + rec_t * 0.45,);
            ImDrawList_AddText_Vec2(
                dl,
                ImVec2_c { x: pos.x + 12.0, y: text_y, },
                u32(label_c,),
                label_cstr.as_ptr(),
                null(),
            );

            let r_edge = pos.x + avail - 10.0;
            let chip_y = pos.y + (row_height - (igGetTextLineHeight() + 5.0)) * 0.5;

            if self.recording
            {
                let pulse2 = self._pulse_t.sin() * 0.5 + 0.5;

                let listen_txt = "Press any key...\0";
                let listen_cstr = cstr(listen_txt,);
                let listen_c = lerp4(color::ACCENT_DIM, color::ACCENT_BRIGHT, pulse2,);
                let ltsz = igCalcTextSize(listen_cstr.as_ptr(), null(), false, -1.0,);
                let esc_str = "Esc\0";
                let esc_cstr = cstr(esc_str,);
                let esc_w = igCalcTextSize(esc_cstr.as_ptr(), null(), false, -1.0,).x + 16.0;
                let lt_x = r_edge - esc_w - 8.0 - ltsz.x;
                ImDrawList_AddText_Vec2(
                    dl,
                    ImVec2_c { x: lt_x, y: text_y, },
                    u32(listen_c,),
                    listen_cstr.as_ptr(),
                    null(),
                );

                Self::draw_chip(
                    dl,
                    ImVec2_c { x: r_edge - esc_w + 2.0, y: chip_y, },
                    "Esc",
                    alpha(color::WARNING, 0.90,),
                );
            }
            else if self.is_set()
            {
                let fa = self._flash_t;

                let mk_str = Self::key_name(self.main_key,);
                let main_c = lerp4(color::ACCENT, color::ACCENT_BRIGHT, fa * 0.70,);
                let mod_c = lerp4(color::ACCENT_DIM, color::ACCENT, fa * 0.70,);

                let mut cx = r_edge;
                cx = Self::draw_chip_r(dl, cx, chip_y, &mk_str, main_c,);

                let has_any_mod = self.ctrl || self.shift || self.alt;

                if has_any_mod
                {
                    cx = Self::plus_r(dl, cx, text_y,);
                }
                if self.alt
                {
                    cx = Self::draw_chip_r(dl, cx, chip_y, "Alt", mod_c,);
                    if self.ctrl || self.shift
                    {
                        cx = Self::plus_r(dl, cx, text_y,);
                    }
                }
                if self.shift
                {
                    cx = Self::draw_chip_r(dl, cx, chip_y, "Shift", mod_c,);
                    if self.ctrl
                    {
                        cx = Self::plus_r(dl, cx, text_y,);
                    }
                }
                if self.ctrl
                {
                    Self::draw_chip_r(dl, cx, chip_y, "Ctrl", mod_c,);
                }
            }
            else
            {
                let not_bound_a = 0.38 + hov_t * 0.22;

                let nb_txt = if self._clear_fl_t > 0.0 { "Cleared\0" } else { "Not bound  -  click to record\0" };
                let nb_cstr = cstr(nb_txt,);
                let nb_c = if self._clear_fl_t > 0.0
                {
                    lerp4(color::TEXT_LOW, color::DANGER, self._clear_fl_t * 0.75,)
                }
                else
                {
                    alpha(color::TEXT_LOW, not_bound_a,)
                };
                let nbsz = igCalcTextSize(nb_cstr.as_ptr(), null(), false, -1.0,);
                ImDrawList_AddText_Vec2(
                    dl,
                    ImVec2_c { x: r_edge - nbsz.x, y: text_y, },
                    u32(nb_c,),
                    nb_cstr.as_ptr(),
                    null(),
                );
            }

            changed
        }
    }

    fn draw_chip(dl: *mut ImDrawList, pos: ImVec2_c, txt: &str, color_val: ImVec4_c,) {
        unsafe {
            let txt_cstr = cstr(txt,);
            let tsz = igCalcTextSize(txt_cstr.as_ptr(), null(), false, -1.0,);
            let px = 7.0;
            let py = 2.5;
            let rr = 4.0;
            let mn = pos;
            let mx = ImVec2_c { x: pos.x + tsz.x + px * 2.0, y: pos.y + tsz.y + py * 2.0, };

            ImDrawList_AddRectFilled(dl, mn, mx, u32(alpha(color_val, 0.18,),), rr, 0,);
            ImDrawList_AddRectFilled(
                dl,
                ImVec2_c { x: mn.x + 1.0, y: mx.y - 1.8, },
                ImVec2_c { x: mx.x - 1.0, y: mx.y + 2.2, },
                im_col32(0, 0, 0, 55,),
                rr,
                0,
            );
            ImDrawList_AddRect(dl, mn, mx, u32(alpha(color_val, 0.58,),), rr, 0, 1.0,);
            ImDrawList_AddText_Vec2(
                dl,
                ImVec2_c { x: pos.x + px, y: pos.y + py, },
                u32(alpha(color_val, 1.0,),),
                txt_cstr.as_ptr(),
                null(),
            );
        }
    }

    fn draw_chip_r(dl: *mut ImDrawList, right_x: f32, y: f32, txt: &str, color_val: ImVec4_c,) -> f32 {
        unsafe {
            let txt_cstr = cstr(txt,);
            let tsz = igCalcTextSize(txt_cstr.as_ptr(), null(), false, -1.0,);
            let px = 7.0;
            let py = 2.5;
            let rr = 4.0;
            let chip_w = tsz.x + px * 2.0;
            let chip_h = tsz.y + py * 2.0;
            let mn = ImVec2_c { x: right_x - chip_w, y, };
            let mx = ImVec2_c { x: right_x, y: y + chip_h, };

            ImDrawList_AddRectFilled(dl, mn, mx, u32(alpha(color_val, 0.18,),), rr, 0,);
            ImDrawList_AddRectFilled(
                dl,
                ImVec2_c { x: mn.x + 1.0, y: mx.y - 1.8, },
                ImVec2_c { x: mx.x - 1.0, y: mx.y + 2.2, },
                im_col32(0, 0, 0, 55,),
                rr,
                0,
            );
            ImDrawList_AddRect(dl, mn, mx, u32(alpha(color_val, 0.58,),), rr, 0, 1.0,);
            ImDrawList_AddText_Vec2(
                dl,
                ImVec2_c { x: mn.x + px, y: mn.y + py, },
                u32(alpha(color_val, 1.0,),),
                txt_cstr.as_ptr(),
                null(),
            );

            mn.x
        }
    }

    fn plus_r(dl: *mut ImDrawList, right_x: f32, text_y: f32,) -> f32 {
        unsafe {
            let plus_cstr = cstr("+",);
            let psz = igCalcTextSize(plus_cstr.as_ptr(), null(), false, -1.0,);
            let gap = 5.0;
            let px = right_x - gap - psz.x;
            ImDrawList_AddText_Vec2(
                dl,
                ImVec2_c { x: px, y: text_y, },
                u32(alpha(color::TEXT_LOW, 0.70,),),
                plus_cstr.as_ptr(),
                null(),
            );
            px - gap
        }
    }

    fn key_name(k: ImGuiKey,) -> &'static str {
        unsafe {
            let name = igGetKeyName(k,);
            if !name.is_null()
            {
                if let Ok(s,) = CStr::from_ptr(name,).to_str()
                {
                    if !s.is_empty() && s != "0"
                    {
                        return s;
                    }
                }
            }
        }
        match k
        {
            ImGuiKey_ImGuiKey_Tab => "Tab",
            ImGuiKey_ImGuiKey_LeftArrow => "Left",
            ImGuiKey_ImGuiKey_RightArrow => "Right",
            ImGuiKey_ImGuiKey_UpArrow => "Up",
            ImGuiKey_ImGuiKey_DownArrow => "Down",
            ImGuiKey_ImGuiKey_PageUp => "PgUp",
            ImGuiKey_ImGuiKey_PageDown => "PgDn",
            ImGuiKey_ImGuiKey_Home => "Home",
            ImGuiKey_ImGuiKey_End => "End",
            ImGuiKey_ImGuiKey_Insert => "Ins",
            ImGuiKey_ImGuiKey_Delete => "Del",
            ImGuiKey_ImGuiKey_Backspace => "Bksp",
            ImGuiKey_ImGuiKey_Space => "Space",
            ImGuiKey_ImGuiKey_Enter => "Enter",
            ImGuiKey_ImGuiKey_Escape => "Esc",
            ImGuiKey_ImGuiKey_F1 => "F1",
            ImGuiKey_ImGuiKey_F2 => "F2",
            ImGuiKey_ImGuiKey_F3 => "F3",
            ImGuiKey_ImGuiKey_F4 => "F4",
            ImGuiKey_ImGuiKey_F5 => "F5",
            ImGuiKey_ImGuiKey_F6 => "F6",
            ImGuiKey_ImGuiKey_F7 => "F7",
            ImGuiKey_ImGuiKey_F8 => "F8",
            ImGuiKey_ImGuiKey_F9 => "F9",
            ImGuiKey_ImGuiKey_F10 => "F10",
            ImGuiKey_ImGuiKey_F11 => "F11",
            ImGuiKey_ImGuiKey_F12 => "F12",
            ImGuiKey_ImGuiKey_A => "A",
            ImGuiKey_ImGuiKey_B => "B",
            ImGuiKey_ImGuiKey_C => "C",
            ImGuiKey_ImGuiKey_D => "D",
            ImGuiKey_ImGuiKey_E => "E",
            ImGuiKey_ImGuiKey_F => "F",
            ImGuiKey_ImGuiKey_G => "G",
            ImGuiKey_ImGuiKey_H => "H",
            ImGuiKey_ImGuiKey_I => "I",
            ImGuiKey_ImGuiKey_J => "J",
            ImGuiKey_ImGuiKey_K => "K",
            ImGuiKey_ImGuiKey_L => "L",
            ImGuiKey_ImGuiKey_M => "M",
            ImGuiKey_ImGuiKey_N => "N",
            ImGuiKey_ImGuiKey_O => "O",
            ImGuiKey_ImGuiKey_P => "P",
            ImGuiKey_ImGuiKey_Q => "Q",
            ImGuiKey_ImGuiKey_R => "R",
            ImGuiKey_ImGuiKey_S => "S",
            ImGuiKey_ImGuiKey_T => "T",
            ImGuiKey_ImGuiKey_U => "U",
            ImGuiKey_ImGuiKey_V => "V",
            ImGuiKey_ImGuiKey_W => "W",
            ImGuiKey_ImGuiKey_X => "X",
            ImGuiKey_ImGuiKey_Y => "Y",
            ImGuiKey_ImGuiKey_Z => "Z",
            ImGuiKey_ImGuiKey_0 => "0",
            ImGuiKey_ImGuiKey_1 => "1",
            ImGuiKey_ImGuiKey_2 => "2",
            ImGuiKey_ImGuiKey_3 => "3",
            ImGuiKey_ImGuiKey_4 => "4",
            ImGuiKey_ImGuiKey_5 => "5",
            ImGuiKey_ImGuiKey_6 => "6",
            ImGuiKey_ImGuiKey_7 => "7",
            ImGuiKey_ImGuiKey_8 => "8",
            ImGuiKey_ImGuiKey_9 => "9",
            ImGuiKey_ImGuiKey_Minus => "-",
            ImGuiKey_ImGuiKey_Equal => "=",
            ImGuiKey_ImGuiKey_LeftBracket => "[",
            ImGuiKey_ImGuiKey_RightBracket => "]",
            ImGuiKey_ImGuiKey_Backslash => "\\",
            ImGuiKey_ImGuiKey_Semicolon => ";",
            ImGuiKey_ImGuiKey_Apostrophe => "'",
            ImGuiKey_ImGuiKey_GraveAccent => "`",
            ImGuiKey_ImGuiKey_Comma => ",",
            ImGuiKey_ImGuiKey_Period => ".",
            ImGuiKey_ImGuiKey_Slash => "/",
            ImGuiKey_ImGuiKey_CapsLock => "Caps",
            ImGuiKey_ImGuiKey_PrintScreen => "PrtSc",
            ImGuiKey_ImGuiKey_Pause => "Pause",
            ImGuiKey_ImGuiKey_NumLock => "NumLk",
            ImGuiKey_ImGuiKey_ScrollLock => "ScrLk",
            ImGuiKey_ImGuiKey_Keypad0 => "Num0",
            ImGuiKey_ImGuiKey_Keypad1 => "Num1",
            ImGuiKey_ImGuiKey_Keypad2 => "Num2",
            ImGuiKey_ImGuiKey_Keypad3 => "Num3",
            ImGuiKey_ImGuiKey_Keypad4 => "Num4",
            ImGuiKey_ImGuiKey_Keypad5 => "Num5",
            ImGuiKey_ImGuiKey_Keypad6 => "Num6",
            ImGuiKey_ImGuiKey_Keypad7 => "Num7",
            ImGuiKey_ImGuiKey_Keypad8 => "Num8",
            ImGuiKey_ImGuiKey_Keypad9 => "Num9",
            ImGuiKey_ImGuiKey_KeypadAdd => "Num+",
            ImGuiKey_ImGuiKey_KeypadSubtract => "Num-",
            ImGuiKey_ImGuiKey_KeypadMultiply => "Num*",
            ImGuiKey_ImGuiKey_KeypadDivide => "Num/",
            ImGuiKey_ImGuiKey_KeypadDecimal => "Num.",
            ImGuiKey_ImGuiKey_KeypadEnter => "NumEnt",
            _ => "?",
        }
    }
}

pub fn keybind_row(label: &str, tip: &str, kb: &mut KeybindRecorder, row_height: f32,) -> bool {
    unsafe {
        let tip_r = 7.5;
        let tip_w = tip_r * 2.0 + 8.0;
        let avail = igGetContentRegionAvail().x;
        let pos = igGetCursorScreenPos();

        let tip_y = pos.y + (row_height - tip_r * 2.0) * 0.5;
        igSetCursorScreenPos(ImVec2_c { x: pos.x + avail - tip_w, y: tip_y, },);
        info_tip(tip, tip_r, color::ACCENT,);

        igSetCursorScreenPos(pos,);
        igPushItemWidth(avail - tip_w - 4.0,);
        let changed = kb.render(label, row_height,);
        igPopItemWidth();

        changed
    }
}

mod demo {
    use std::ptr::null_mut;

    use super::*;
    use crate::ffi::{
        ImGuiCond__ImGuiCond_FirstUseEver, ImGuiKey_ImGuiKey_F3, ImGuiKey_ImGuiKey_F12, ImGuiKey_ImGuiKey_M,
        ImGuiKey_ImGuiKey_Tab, ImGuiWindowFlags__ImGuiWindowFlags_NoScrollWithMouse,
        ImGuiWindowFlags__ImGuiWindowFlags_NoScrollbar, ImVec2_c, ImVec4_c, igGetContentRegionAvail,
        igGetCursorScreenPos, igGetIO_Nil, igGetTextLineHeight, igGetTime, igGetWindowDrawList, igGetWindowPos,
        igGetWindowSize,
    };

    static mut S_FPS: f32 = 144.0;
    static mut S_FRAMETIME: f32 = 6.9;
    static mut S_PING: f32 = 28.0;
    static mut S_PACKETLOSS: f32 = 0.0;
    static mut S_CPU_LOAD: f32 = 0.42;
    static mut S_GPU_LOAD: f32 = 0.78;
    static mut S_VRAM_USED: f32 = 0.61;
    static mut S_RAM_USED: f32 = 0.35;

    static mut S_VSYNC: bool = false;
    static mut S_FXAA: bool = true;
    static mut S_MOTION_BLUR: bool = false;
    static mut S_SHOW_FPS: bool = true;
    static mut S_SHOW_PING: bool = true;
    static mut S_SHOW_MAP: bool = false;
    static mut S_SHOW_DAMAGE: bool = true;
    static mut S_BRIGHTNESS: f32 = 0.55;
    static mut S_CONTRAST: f32 = 0.50;
    static mut S_SATURATION: f32 = 0.60;
    static mut S_FOV: f32 = 0.72;
    static mut S_SENSITIVITY: f32 = 0.38;
    static mut S_RESOLUTION_IDX: i32 = 2;
    static mut S_QUALITY_IDX: i32 = 1;
    static mut S_FPS_CAP: i32 = 7;

    static mut S_KB_SCORE_BOARD: KeybindRecorder = KeybindRecorder {
        main_key:     ImGuiKey_ImGuiKey_None,
        ctrl:         false,
        shift:        false,
        alt:          false,
        recording:    false,
        _pulse_t:     0.0,
        _flash_t:     0.0,
        _clear_fl_t:  0.0,
        _saved_key:   ImGuiKey_ImGuiKey_None,
        _saved_ctrl:  false,
        _saved_shift: false,
        _saved_alt:   false,
    };
    static mut S_KB_MAP: KeybindRecorder = KeybindRecorder {
        main_key:     ImGuiKey_ImGuiKey_None,
        ctrl:         false,
        shift:        false,
        alt:          false,
        recording:    false,
        _pulse_t:     0.0,
        _flash_t:     0.0,
        _clear_fl_t:  0.0,
        _saved_key:   ImGuiKey_ImGuiKey_None,
        _saved_ctrl:  false,
        _saved_shift: false,
        _saved_alt:   false,
    };
    static mut S_KB_PING: KeybindRecorder = KeybindRecorder {
        main_key:     ImGuiKey_ImGuiKey_None,
        ctrl:         false,
        shift:        false,
        alt:          false,
        recording:    false,
        _pulse_t:     0.0,
        _flash_t:     0.0,
        _clear_fl_t:  0.0,
        _saved_key:   ImGuiKey_ImGuiKey_None,
        _saved_ctrl:  false,
        _saved_shift: false,
        _saved_alt:   false,
    };
    static mut S_KB_SCREENSHOT: KeybindRecorder = KeybindRecorder {
        main_key:     ImGuiKey_ImGuiKey_None,
        ctrl:         false,
        shift:        false,
        alt:          false,
        recording:    false,
        _pulse_t:     0.0,
        _flash_t:     0.0,
        _clear_fl_t:  0.0,
        _saved_key:   ImGuiKey_ImGuiKey_None,
        _saved_ctrl:  false,
        _saved_shift: false,
        _saved_alt:   false,
    };
    static mut S_KB_FPS_TOGGLE: KeybindRecorder = KeybindRecorder {
        main_key:     ImGuiKey_ImGuiKey_None,
        ctrl:         false,
        shift:        false,
        alt:          false,
        recording:    false,
        _pulse_t:     0.0,
        _flash_t:     0.0,
        _clear_fl_t:  0.0,
        _saved_key:   ImGuiKey_ImGuiKey_None,
        _saved_ctrl:  false,
        _saved_shift: false,
        _saved_alt:   false,
    };

    static mut S_ACTIVE_TAB: i32 = 0;
    static mut S_PROFILE_NAME: [u8; 64] = [0; 64];
    static mut S_TOAST: Toast = Toast { msg: [0; 256], timer: 0.0, color: color::ACCENT, max_time: 2.5, };
    static mut S_KB_INITIALIZED: bool = false;

    pub fn init_keybinds() {
        unsafe {
            if S_KB_INITIALIZED
            {
                return;
            }
            S_KB_SCORE_BOARD.main_key = ImGuiKey_ImGuiKey_Tab;
            S_KB_MAP.main_key = ImGuiKey_ImGuiKey_M;
            S_KB_PING.main_key = ImGuiKey_ImGuiKey_F3;
            S_KB_PING.ctrl = true;
            S_KB_SCREENSHOT.main_key = ImGuiKey_ImGuiKey_F12;
            S_KB_INITIALIZED = true;

            if S_PROFILE_NAME[0] == 0
            {
                let name = b"Default\0";
                S_PROFILE_NAME[..name.len()].copy_from_slice(name,);
            }
        }
    }

    fn tick_simulation() {
        unsafe {
            let t = igGetTime() as f32;
            S_FPS = 144.0 + (t * 0.7).sin() * 8.0;
            S_FRAMETIME = 1000.0 / S_FPS;
            S_PING = 28.0 + (t * 1.3).sin() * 4.0;
            S_CPU_LOAD = 0.42 + (t * 0.5).sin() * 0.08;
            S_GPU_LOAD = 0.78 + (t * 0.9).sin() * 0.06;
            S_VRAM_USED = 0.61 + (t * 0.4).sin() * 0.04;
            S_RAM_USED = 0.35 + (t * 0.3).sin() * 0.03;
        }
    }

    fn load_color(v: f32,) -> ImVec4_c {
        if v < 0.60
        {
            color::SUCCESS
        }
        else if v < 0.85
        {
            color::WARNING
        }
        else
        {
            color::DANGER
        }
    }

    fn render_tab_performance() {
        unsafe {
            let _avail = igGetContentRegionAvail().x;

            section_header("System Status", color::ACCENT,);

            let ping_norm = im_clamp_f32(S_PING / 200.0, 0.0, 1.0,);
            let ping_color = load_color(ping_norm,);

            igBeginGroup();
            status_dot("Server Connected", color::SUCCESS,);
            igSameLine(0.0, 24.0,);
            status_dot("Anti-Cheat", color::SUCCESS,);
            igSameLine(0.0, 24.0,);
            status_dot("API Online", color::ACCENT,);
            igEndGroup();

            igSpacing();

            {
                let avail_local = igGetContentRegionAvail().x;

                igText("Delay (Ping)\0".as_ptr() as *const i8,);
                igSameLine(avail_local * 0.38, 0.0,);
                igPushItemWidth(avail_local * 0.42,);
                progress_bar(ping_norm, ImVec2_c { x: avail_local * 0.42, y: 7.0, }, ping_color,);
                igPopItemWidth();
                igSameLine(0.0, 8.0,);
                badge(&format!("{:.0} ms\0", S_PING), ping_color,);

                igText("Packet Loss\0".as_ptr() as *const i8,);
                igSameLine(avail_local * 0.38, 0.0,);
                progress_bar(S_PACKETLOSS, ImVec2_c { x: avail_local * 0.42, y: 7.0, }, color::SUCCESS,);
                igSameLine(0.0, 8.0,);
                badge("0 %\0", color::SUCCESS,);
            }

            igSpacing();
            separator(color::ACCENT, 0.18,);

            section_header("Frame Rate", color::ACCENT,);

            {
                let avail_local = igGetContentRegionAvail().x;

                let fps_badge_text = format!("{:.0} FPS\0", S_FPS);
                badge(
                    &fps_badge_text,
                    if S_FPS >= 120.0
                    {
                        color::SUCCESS
                    }
                    else if S_FPS >= 60.0
                    {
                        color::WARNING
                    }
                    else
                    {
                        color::DANGER
                    },
                );
                igSameLine(0.0, 10.0,);
                let ft_badge_text = format!("{:.2} ms\0", S_FRAMETIME);
                badge(&ft_badge_text, color::ACCENT_DIM,);
                igSameLine(0.0, 10.0,);
                badge(
                    if S_VSYNC { "VSync ON\0" } else { "VSync OFF\0" },
                    if S_VSYNC { color::ACCENT } else { color::TEXT_LOW },
                );

                igSpacing();

                let bar_w = (avail_local - 10.0 * 4.0) / 11.0;
                let mut fake_bars: [f32; 11] = [0.85, 0.92, 0.88, 0.95, 0.90, 0.87, 0.93, 0.96, 0.89, 0.91, 0.94,];
                fake_bars[10] = im_clamp_f32(S_FPS / 160.0, 0.0, 1.0,);
                let cursor = igGetCursorScreenPos();
                let dl = igGetWindowDrawList();
                let bh = 40.0;
                for i in 0..11
                {
                    let bx = cursor.x + i as f32 * (bar_w + 4.0);
                    let by = cursor.y + bh * (1.0 - fake_bars[i as usize]);
                    let bc = load_color(1.0 - fake_bars[i as usize],);

                    ImDrawList_AddRectFilled(
                        dl,
                        ImVec2_c { x: bx + 1.0, y: by + 1.0, },
                        ImVec2_c { x: bx + bar_w, y: cursor.y + bh + 1.0, },
                        im_col32(0, 0, 0, 40,),
                        3.0,
                        0,
                    );
                    ImDrawList_AddRectFilled(
                        dl,
                        ImVec2_c { x: bx, y: by, },
                        ImVec2_c { x: bx + bar_w, y: cursor.y + bh, },
                        u32(alpha(bc, 0.80,),),
                        3.0,
                        0,
                    );

                    ImDrawList_AddRectFilled(
                        dl,
                        ImVec2_c { x: bx, y: by, },
                        ImVec2_c { x: bx + bar_w, y: by + 2.0, },
                        u32(alpha(color::ACCENT_BRIGHT, 0.25,),),
                        1.0,
                        0,
                    );
                }
                igDummy(ImVec2_c { x: avail_local, y: bh + 4.0, },);
            }

            igSpacing();
            separator(color::ACCENT, 0.18,);

            section_header("Hardware", color::ACCENT,);

            let avail_local = igGetContentRegionAvail().x;

            let items: [(&str, f32, &str,); 4] = [
                ("CPU\0", S_CPU_LOAD, "\0",),
                ("GPU\0", S_GPU_LOAD, "\0",),
                ("VRAM\0", S_VRAM_USED, "\0",),
                ("RAM\0", S_RAM_USED, "\0",),
            ];

            for &(name, val, _unit,) in items.iter()
            {
                let c = load_color(val,);
                igText(name.as_ptr() as *const i8,);
                igSameLine(avail_local * 0.14, 0.0,);
                progress_bar(val, ImVec2_c { x: avail_local * 0.62, y: 8.0, }, c,);
                igSameLine(0.0, 8.0,);
                let pct_text = format!("{:.0} %\0", val * 100.0);
                badge(&pct_text, c,);
                igSpacing();
            }
        }
    }

    fn render_tab_settings() {
        unsafe {
            let avail = igGetContentRegionAvail().x;
            let half_w = avail * 0.48;

            section_header("HUD Display", color::ACCENT,);

            igColumns(2, "hud_cols\0".as_ptr() as *const i8, false,);
            igSetColumnWidth(0, half_w,);

            toggle("Show FPS\0", &mut S_SHOW_FPS,);
            igSpacing();
            toggle("Show Ping\0", &mut S_SHOW_PING,);
            igSpacing();
            toggle("Show Minimap\0", &mut S_SHOW_MAP,);

            igNextColumn();

            toggle("Show Damage\0", &mut S_SHOW_DAMAGE,);
            igSpacing();
            checkbox("V-Sync\0", &mut S_VSYNC,);
            igSpacing();
            checkbox("FXAA\0", &mut S_FXAA,);
            igSpacing();
            checkbox("Motion Blur\0", &mut S_MOTION_BLUR,);

            igColumns(1, null(), false,);

            igSpacing();
            separator(color::ACCENT, 0.18,);

            section_header("Video Settings", color::ACCENT,);

            let res_options = ["1280x720\0", "1920x1080\0", "2560x1440\0", "3840x2160\0",];
            let qual_options = ["Low\0", "Medium\0", "High\0", "Ultra\0",];

            igText("Resolution\0".as_ptr() as *const i8,);
            igSameLine(avail * 0.32, 0.0,);
            igPushItemWidth(avail * 0.65,);
            if begin_combo("##res\0", res_options[S_RESOLUTION_IDX as usize],)
            {
                for i in 0..4
                {
                    let sel = i == S_RESOLUTION_IDX;
                    if igSelectable_Bool(
                        res_options[i as usize].as_ptr() as *const i8,
                        sel,
                        0,
                        ImVec2_c { x: 0.0, y: 0.0, },
                    )
                    {
                        S_RESOLUTION_IDX = i;
                    }
                    if sel
                    {
                        igSetItemDefaultFocus();
                    }
                }
                end_combo();
            }
            igPopItemWidth();

            igSpacing();

            igText("Quality Preset\0".as_ptr() as *const i8,);
            igSameLine(avail * 0.32, 0.0,);
            igPushItemWidth(avail * 0.65,);
            if begin_combo("##qual\0", qual_options[S_QUALITY_IDX as usize],)
            {
                for i in 0..4
                {
                    let sel = i == S_QUALITY_IDX;
                    if igSelectable_Bool(
                        qual_options[i as usize].as_ptr() as *const i8,
                        sel,
                        0,
                        ImVec2_c { x: 0.0, y: 0.0, },
                    )
                    {
                        S_QUALITY_IDX = i;
                    }
                    if sel
                    {
                        igSetItemDefaultFocus();
                    }
                }
                end_combo();
            }
            igPopItemWidth();

            igSpacing();
            separator(color::ACCENT, 0.18,);

            section_header("Color & View", color::ACCENT,);

            slider_float("Brightness\0", &mut S_BRIGHTNESS, 0.0, 1.0, "%.2f\0", 34.0,);
            igSpacing();
            slider_float("Contrast\0", &mut S_CONTRAST, 0.0, 1.0, "%.2f\0", 34.0,);
            igSpacing();
            slider_float("Saturation\0", &mut S_SATURATION, 0.0, 1.0, "%.2f\0", 34.0,);

            igSpacing();
            separator(color::ACCENT, 0.18,);

            section_header("Controls", color::ACCENT,);

            slider_float("FOV\0", &mut S_FOV, 0.0, 1.0, "%.2f\0", 34.0,);
            igSpacing();
            slider_float("Sensitivity\0", &mut S_SENSITIVITY, 0.0, 1.0, "%.2f\0", 34.0,);
            igSpacing();
            slider_int("FPS Cap\0", &mut S_FPS_CAP, 0, 10,);

            igSpacing();
            separator(color::ACCENT, 0.18,);

            let btn_w = (avail - 12.0) / 3.0;
            if button("Apply\0", ImVec2_c { x: btn_w, y: 36.0, }, ButtonVariant::Primary,)
            {
                S_TOAST.show("Settings Applied!", 2.5, color::SUCCESS,);
            }
            igSameLine(0.0, 6.0,);
            if button("Reset\0", ImVec2_c { x: btn_w, y: 36.0, }, ButtonVariant::Ghost,)
            {
                S_BRIGHTNESS = 0.55;
                S_CONTRAST = 0.50;
                S_SATURATION = 0.60;
                S_FOV = 0.72;
                S_SENSITIVITY = 0.38;
                S_TOAST.show("Reset to Defaults", 2.0, color::WARNING,);
            }
            igSameLine(0.0, 6.0,);
            if button("Clear Stats\0", ImVec2_c { x: btn_w, y: 36.0, }, ButtonVariant::Danger,)
            {
                S_TOAST.show("Statistics Cleared", 2.0, color::DANGER,);
            }
        }
    }

    fn render_tab_keybinds() {
        unsafe {
            section_header("In-Game Keybinds", color::ACCENT,);

            igTextDisabled(
                "Click to start recording / Esc to cancel / Backspace or Delete to clear\0".as_ptr() as *const i8,
            );
            igSpacing();

            let mut entries: [(&str, &str, &mut KeybindRecorder,); 5] = [
                ("Scoreboard\0", "Hold to show scoreboard (Tab)\0", &mut S_KB_SCORE_BOARD,),
                ("Minimap\0", "Toggle minimap overlay\0", &mut S_KB_MAP,),
                ("Network Info\0", "Show detailed ping/loss/route info\0", &mut S_KB_PING,),
                ("Screenshot\0", "Save clean HUD-free screenshot to Screenshots dir\0", &mut S_KB_SCREENSHOT,),
                ("FPS Monitor\0", "Temporarily show frame time monitoring\0", &mut S_KB_FPS_TOGGLE,),
            ];

            for entry in entries.iter_mut()
            {
                keybind_row(entry.0, entry.1, entry.2, 36.0,);
                igSpacing();
            }

            separator(color::ACCENT, 0.18,);
            section_header("Keybind Preview", color::ACCENT,);

            let avail = igGetContentRegionAvail().x;
            igTextDisabled("These are currently active default keybinds (display only)\0".as_ptr() as *const i8,);
            igSpacing();

            let previews: [(&str, &str, Option<&str,>,); 8] = [
                ("Move\0", "W A S D\0", None,),
                ("Jump\0", "Space\0", None,),
                ("Crouch\0", "Ctrl\0", None,),
                ("Sprint\0", "Shift\0", None,),
                ("Aim\0", "RMB\0", None,),
                ("Reload\0", "R\0", None,),
                ("Swap Weapon\0", "Q\0", None,),
                ("Throwable\0", "G\0", None,),
            ];

            let mut col = 0;
            igColumns(2, "kb_preview_cols\0".as_ptr() as *const i8, false,);
            igSetColumnWidth(0, avail * 0.50,);
            for &(action, key1, key2,) in previews.iter()
            {
                let tc = color::TEXT_MID;
                let action_cstr = cstr(action,);
                let cursor = igGetCursorScreenPos();
                ImDrawList_AddText_Vec2(igGetWindowDrawList(), cursor, u32(tc,), action_cstr.as_ptr(), null(),);
                igDummy(ImVec2_c {
                    x: igCalcTextSize(action_cstr.as_ptr(), null(), false, -1.0,).x,
                    y: igGetTextLineHeight(),
                },);
                igSameLine(0.0, 6.0,);
                keybind_static(key1,);
                if let Some(k2,) = key2
                {
                    igSameLine(0.0, 4.0,);
                    keybind_static(k2,);
                }
                igSpacing();
                col += 1;
                if col == 4
                {
                    igNextColumn();
                }
            }
            igColumns(1, null(), false,);
        }
    }

    fn render_tab_about() {
        unsafe {
            let avail = igGetContentRegionAvail().x;

            section_header("About", color::ACCENT,);

            igBeginGroup();
            badge("v1.0.0\0", color::ACCENT,);
            igSameLine(0.0, 8.0,);
            badge("Stable\0", color::SUCCESS,);
            igSameLine(0.0, 8.0,);
            badge("64-bit\0", color::TEXT_MID,);
            igSameLine(0.0, 8.0,);
            badge("DirectX 12\0", color::ACCENT_DIM,);
            igEndGroup();

            igSpacing();

            status_dot("Renderer Active", color::SUCCESS,);
            igSameLine(0.0, 20.0,);
            status_dot("Data Collected", color::SUCCESS,);
            igSameLine(0.0, 20.0,);
            status_dot("Latest Driver", color::WARNING,);

            igSpacing();
            separator(color::ACCENT, 0.18,);

            section_header("Profile中文测试", color::ACCENT,);

            input_text("Profile Name\0", &mut S_PROFILE_NAME, 0,);
            igSpacing();

            let btn_w = (avail - 8.0) / 2.0;
            if button("Save Profile\0", ImVec2_c { x: btn_w, y: 34.0, }, ButtonVariant::Ghost,)
            {
                S_TOAST.show("Profile saved! (Demo only)", 2.0, color::WARNING,);
            }
            igSameLine(0.0, 8.0,);
            if button("Load Profile\0", ImVec2_c { x: btn_w, y: 34.0, }, ButtonVariant::Ghost,)
            {
                S_TOAST.show("Profile loaded! (Demo only)", 2.0, color::WARNING,);
            }

            igSpacing();
            separator(color::ACCENT, 0.18,);

            section_header("InfoTip Demo", color::ACCENT,);

            igText("Hover over the dot icons to see tooltips.\0".as_ptr() as *const i8,);
            igSpacing();

            let tips: [(&str, &str, ImVec4_c,); 4] = [
                ("FPS\0", "Current render frame rate in FPS. Target >= 60.\0", color::ACCENT,),
                ("Ping\0", "Round-trip time to game server (RTT). < 50ms is recommended.\0", color::WARNING,),
                ("Packet Loss\0", "UDP packet loss percentage. < 2% for smooth gameplay.\0", color::DANGER,),
                ("Resolution\0", "Render resolution — higher is sharper but more GPU load.\0", color::SUCCESS,),
            ];

            for &(label, tip, col_val,) in tips.iter()
            {
                let label_cstr = cstr(label,);
                igText(label_cstr.as_ptr(),);
                igSameLine(0.0, 6.0,);
                info_tip(tip, 8.5, col_val,);
                igSameLine(0.0, 20.0,);
            }
            igNewLine();

            igSpacing();
            separator(color::ACCENT, 0.18,);

            section_header("Button Styles", color::ACCENT,);

            let bw = (avail - 16.0) / 3.0;
            if button("Primary\0", ImVec2_c { x: bw, y: 36.0, }, ButtonVariant::Primary,)
            {
                S_TOAST.show("Primary button clicked", 2.0, color::ACCENT,);
            }
            igSameLine(0.0, 8.0,);
            if button("Ghost\0", ImVec2_c { x: bw, y: 36.0, }, ButtonVariant::Ghost,)
            {
                S_TOAST.show("Ghost button clicked", 2.0, color::ACCENT_DIM,);
            }
            igSameLine(0.0, 8.0,);
            if button("Danger\0", ImVec2_c { x: bw, y: 36.0, }, ButtonVariant::Danger,)
            {
                S_TOAST.show("Danger button clicked", 2.5, color::DANGER,);
            }

            igSpacing();
            separator(color::ACCENT, 0.18,);

            section_header("Progress Bar Variants", color::ACCENT,);

            let demo_progress = (igGetTime() as f32 * 0.8).sin() * 0.5 + 0.5;

            progress_bar(demo_progress, ImVec2_c { x: -1.0, y: 8.0, }, color::ACCENT,);
            igSpacing();
            progress_bar(demo_progress * 0.8, ImVec2_c { x: -1.0, y: 8.0, }, color::SUCCESS,);
            igSpacing();
            progress_bar(demo_progress * 0.6 + 0.3, ImVec2_c { x: -1.0, y: 8.0, }, color::WARNING,);
            igSpacing();
            progress_bar(demo_progress * 0.4 + 0.5, ImVec2_c { x: -1.0, y: 8.0, }, color::DANGER,);
        }
    }

    pub fn render_fps_tool_demo() {
        unsafe {
            init_keybinds();

            tick_simulation();

            apply_theme();

            let io = &mut *igGetIO_Nil();

            igSetNextWindowSize(ImVec2_c { x: 560.0, y: 680.0, }, ImGuiCond__ImGuiCond_FirstUseEver,);

            igSetNextWindowPos(
                ImVec2_c { x: io.DisplaySize.x * 0.5, y: io.DisplaySize.y * 0.5, },
                ImGuiCond__ImGuiCond_FirstUseEver,
                ImVec2_c { x: 0.5, y: 0.5, },
            );

            igBegin(
                "##fps_tool\0".as_ptr() as *const i8,
                null_mut(),
                ImGuiWindowFlags__ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags__ImGuiWindowFlags_NoScrollWithMouse,
            );

            {
                let avail = igGetContentRegionAvail().x;
                let dl = igGetWindowDrawList();
                let wp = igGetWindowPos();
                let ws = igGetWindowSize();

                ImDrawList_AddRectFilledMultiColor(
                    dl,
                    wp,
                    ImVec2_c { x: wp.x + ws.x, y: wp.y + 3.0, },
                    im_col32(0, 0, 0, 0,),
                    im_col32(0, 0, 0, 0,),
                    u32(color::ACCENT,),
                    u32(color::ACCENT_DIM,),
                );

                badge("FPS TOOL\0", color::ACCENT,);
                igSameLine(0.0, 8.0,);
                badge("DEMO\0", color::ACCENT_DIM,);
                igSameLine(avail - 120.0, 0.0,);

                let fps_text = format!("{:.0} fps  {:.1} ms\0", S_FPS, S_FRAMETIME);
                let fps_cstr = cstr(&fps_text,);
                ImDrawList_AddText_Vec2(
                    dl,
                    igGetCursorScreenPos(),
                    u32(alpha(color::TEXT_MID, 0.60,),),
                    fps_cstr.as_ptr(),
                    null(),
                );
                igDummy(ImVec2_c { x: 120.0, y: igGetTextLineHeight(), },);
                igSpacing();
            }

            static mut TAB_BAR: Option<TabBar,> = None;
            static mut TABS_ADDED: bool = false;

            if !TABS_ADDED
            {
                let mut tb = TabBar::new("##main_tabs\0", 38.0,);
                tb.add("  Performance  \0", ImVec4_c { x: 0.0, y: 0.0, z: 0.0, w: 0.0, },);
                tb.add("  Settings  \0", ImVec4_c { x: 0.0, y: 0.0, z: 0.0, w: 0.0, },);
                tb.add("  Keybinds  \0", ImVec4_c { x: 0.0, y: 0.0, z: 0.0, w: 0.0, },);
                tb.add("  About  \0", ImVec4_c { x: 0.0, y: 0.0, z: 0.0, w: 0.0, },);
                TAB_BAR = Some(tb,);
                TABS_ADDED = true;
            }

            if let Some(ref mut tab_bar,) = TAB_BAR
            {
                tab_bar.render(&mut S_ACTIVE_TAB,);
            }

            let panel_h = igGetContentRegionAvail().y - 4.0;
            if begin_panel("##content\0", ImVec2_c { x: 0.0, y: panel_h, },)
            {
                igSpacing();

                match S_ACTIVE_TAB
                {
                    0 =>
                    {
                        render_tab_performance();
                    }
                    1 =>
                    {
                        render_tab_settings();
                    }
                    2 =>
                    {
                        render_tab_keybinds();
                    }
                    3 =>
                    {
                        render_tab_about();
                    }
                    _ =>
                    {}
                }
                end_panel();
            }

            igEnd();

            S_TOAST.render(igGetWindowPos(), igGetWindowSize(),);
        }
    }
}

use std::ptr::null_mut;

pub use demo::render_fps_tool_demo;
