#![allow(unsafe_op_in_unsafe_fn)]
#![allow(function_casts_as_integer)]
#![allow(static_mut_refs)]
#![allow(unsupported_calling_conventions)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

pub mod ffi;
pub mod mui;

use std::{
    ffi::c_void,
    ptr::null_mut,
    sync::atomic::{AtomicBool, Ordering},
    vec::Vec,
};

use winapi::{
    shared::{
        dxgi::IDXGISwapChain,
        dxgi1_4::IDXGISwapChain3,
        dxgiformat::DXGI_FORMAT,
        minwindef::{BOOL, DWORD, HINSTANCE, TRUE},
        ntdef::HANDLE,
        windef::HWND,
        winerror::{FAILED, HRESULT, SUCCEEDED},
    },
    um::{
        d3d12::{
            D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_DESCRIPTOR_HEAP_DESC,
            D3D12_DESCRIPTOR_HEAP_FLAG_NONE, D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
            D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_FENCE_FLAG_NONE,
            D3D12_GPU_DESCRIPTOR_HANDLE, D3D12_RESOURCE_BARRIER, D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
            D3D12_RESOURCE_BARRIER_TYPE_TRANSITION, D3D12_RESOURCE_STATE_PRESENT, D3D12_RESOURCE_STATE_RENDER_TARGET,
            ID3D12CommandAllocator, ID3D12CommandQueue, ID3D12DescriptorHeap, ID3D12Device, ID3D12Fence,
            ID3D12GraphicsCommandList, ID3D12Resource,
        },
        libloaderapi::{DisableThreadLibraryCalls, GetModuleHandleW, GetProcAddress},
        processthreadsapi::{CreateThread, GetCurrentProcess},
        psapi::{GetModuleInformation, MODULEINFO},
        synchapi::{CreateEventA, WaitForSingleObject},
        winnt::OSVERSIONINFOW,
        winuser::{CallWindowProcA, DefWindowProcA, GWLP_WNDPROC, SetWindowLongPtrA, WNDPROC},
    },
};

use crate::ffi::{igCreateContext, igGetDrawData, igNewFrame, igRender};

const INFINITE: DWORD = 0xFFFFFFFF;

#[link(name = "libimgui")]
unsafe extern "C" {
    pub fn ImGui_ImplWin32_Init(hwnd: *const c_void,) -> bool;
    pub fn ImGui_ImplWin32_WndProcHandler(hwnd: *const c_void, msg: u32, wparam: usize, lparam: isize,) -> isize;
    pub fn ImGui_ImplWin32_NewFrame();
    pub fn ImGui_ImplWin32_Shutdown();
    pub fn ImGui_ImplWin32_EnableDpiAwareness();
    pub fn ImGui_ImplWin32_EnableAlphaCompositing(hwnd: *const c_void,);
    pub fn ImGui_ImplWin32_GetDpiScaleForMonitor(monitor: *const c_void,) -> f32;
    pub fn ImGui_ImplDX12_Init(
        device: *mut c_void,
        num_frames_in_flight: i32,
        rtv_format: DXGI_FORMAT,
        srv_descriptor_heap: *mut c_void,
        font_srv_cpu_desc_handle: D3D12_CPU_DESCRIPTOR_HANDLE,
        font_srv_gpu_desc_handle: D3D12_GPU_DESCRIPTOR_HANDLE,
    ) -> bool;
    pub fn ImGui_ImplDX12_Shutdown();
    pub fn ImGui_ImplDX12_NewFrame();
    pub fn ImGui_ImplDX12_RenderDrawData(draw_data: *mut ffi::ImDrawData, graphics_command_list: *mut c_void,);
}

type FnPresent =
    unsafe extern "system" fn(swap_chain: *mut IDXGISwapChain3, sync_interval: u32, flags: u32,) -> HRESULT;

type FnResizeBuffers = unsafe extern "system" fn(
    swap_chain: *mut IDXGISwapChain,
    buffer_count: u32,
    width: u32,
    height: u32,
    new_format: DXGI_FORMAT,
    flags: u32,
) -> HRESULT;

static G_INITIALIZED: AtomicBool = AtomicBool::new(false,);

static mut G_PRESENT: Option<FnPresent,> = None;
static mut G_RESIZE_BUFFERS: Option<FnResizeBuffers,> = None;
static mut G_WNDPROC: WNDPROC = None;
static mut G_FONT_DESCRIPTOR_HEAP: *mut ID3D12DescriptorHeap = null_mut();
static mut G_RTV_DESCRIPTOR_HEAP: *mut ID3D12DescriptorHeap = null_mut();
static mut G_COMMAND_QUEUE: *mut ID3D12CommandQueue = null_mut();
static mut G_COMMAND_LIST: *mut ID3D12GraphicsCommandList = null_mut();

static mut G_FENCE: *mut ID3D12Fence = null_mut();
static mut G_FENCE_EVENT: HANDLE = null_mut();
static mut G_FENCE_VALUE: u64 = 0;

static mut G_BUFFER_COUNT: u32 = 0;
static mut G_HWND: HWND = null_mut();

struct FrameContext {
    command_allocator: *mut ID3D12CommandAllocator,
    back_buffer:       *mut ID3D12Resource,
    rtv_handle:        D3D12_CPU_DESCRIPTOR_HANDLE,
    fence_value:       u64,
}

impl Default for FrameContext {
    fn default() -> Self {
        Self {
            command_allocator: null_mut(),
            back_buffer:       null_mut(),
            rtv_handle:        D3D12_CPU_DESCRIPTOR_HANDLE { ptr: 0, },
            fence_value:       0,
        }
    }
}

static mut G_FRAME_CONTEXTS: Vec<FrameContext,> = Vec::new();

macro_rules! as_ppv {
    ($ptr:expr) => {
        &mut $ptr as *mut *mut _ as *mut *mut winapi::ctypes::c_void
    };
}

unsafe fn get_command_queue_offset() -> Option<u32,> {
    let name: [u16; 11] = [0x6E, 0x74, 0x64, 0x6C, 0x6C, 0x2E, 0x64, 0x6C, 0x6C, 0x00, 0x00,];
    let hmod = GetModuleHandleW(name.as_ptr(),) as *mut c_void;
    if hmod.is_null()
    {
        return None;
    }

    #[allow(non_snake_case)]
    type RtlGetVersionFn = unsafe extern "system" fn(*mut OSVERSIONINFOW,) -> i32;

    let proc_name = b"RtlGetVersion\0";
    let fn_ptr = GetProcAddress(hmod as *mut _, proc_name.as_ptr() as *const i8,);
    if fn_ptr.is_null()
    {
        return None;
    }

    let rtl_get_version: RtlGetVersionFn = std::mem::transmute(fn_ptr,);

    let mut os_info = OSVERSIONINFOW {
        dwOSVersionInfoSize: std::mem::size_of::<OSVERSIONINFOW,>() as u32,
        dwMajorVersion:      0,
        dwMinorVersion:      0,
        dwBuildNumber:       0,
        dwPlatformId:        0,
        szCSDVersion:        [0; 128],
    };

    if rtl_get_version(&mut os_info,) == 0
    {
        let build = os_info.dwBuildNumber;

        let offset = if build >= 26100
        {
            0x138u32
        }
        else if build >= 21996
        {
            0x168u32
        }
        else
        {
            0x118u32
        };

        Some(offset,)
    }
    else
    {
        Some(0x118,)
    }
}

unsafe fn wait_for_last_submitted_frame() {
    let queue = G_COMMAND_QUEUE;
    let fence = G_FENCE;
    if queue.is_null() || fence.is_null() || G_FENCE_EVENT.is_null()
    {
        return;
    }
    let fence_val = G_FENCE_VALUE;
    if SUCCEEDED((*queue).Signal(fence, fence_val,),)
    {
        G_FENCE_VALUE += 1;
        if (*fence).GetCompletedValue() < fence_val
        {
            WaitForSingleObject(G_FENCE_EVENT, INFINITE,);
        }
    }
}

unsafe fn cleanup_render_target() {
    wait_for_last_submitted_frame();
    for fc in &G_FRAME_CONTEXTS
    {
        if !fc.back_buffer.is_null()
        {
            (*fc.back_buffer).Release();
        }
        if !fc.command_allocator.is_null()
        {
            (*fc.command_allocator).Release();
        }
    }
    G_FRAME_CONTEXTS.clear();
    if !G_RTV_DESCRIPTOR_HEAP.is_null()
    {
        (*G_RTV_DESCRIPTOR_HEAP).Release();
        G_RTV_DESCRIPTOR_HEAP = null_mut();
    }
}

unsafe fn init_or_update_render_targets(swap_chain: *mut IDXGISwapChain, device: *mut ID3D12Device,) {
    let mut desc: winapi::shared::dxgi::DXGI_SWAP_CHAIN_DESC = std::mem::zeroed();
    let hr = (*swap_chain).GetDesc(&mut desc,);
    if FAILED(hr,)
    {
        return;
    }

    cleanup_render_target();

    G_BUFFER_COUNT = desc.BufferCount;

    let rtv_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
        Type:           D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
        NumDescriptors: G_BUFFER_COUNT,
        Flags:          D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
        NodeMask:       0,
    };

    let mut rtv_heap: *mut ID3D12DescriptorHeap = null_mut();
    let hr = (*device).CreateDescriptorHeap(
        &rtv_heap_desc,
        &<ID3D12DescriptorHeap as winapi::Interface>::uuidof(),
        as_ppv!(rtv_heap),
    );
    if FAILED(hr,)
    {
        return;
    }

    let rtv_desc_size = (*device).GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV,) as usize;
    let rtv_start = (*rtv_heap).GetCPUDescriptorHandleForHeapStart();

    G_FRAME_CONTEXTS = (0..G_BUFFER_COUNT)
        .map(|i| {
            let rtv_handle = D3D12_CPU_DESCRIPTOR_HANDLE { ptr: rtv_start.ptr + i as usize * rtv_desc_size, };

            let mut back_buffer: *mut ID3D12Resource = null_mut();
            let hr =
                (*swap_chain).GetBuffer(i, &<ID3D12Resource as winapi::Interface>::uuidof(), as_ppv!(back_buffer),);
            if SUCCEEDED(hr,)
            {
            }
            else
            {
            }

            if !back_buffer.is_null()
            {
                (*device).CreateRenderTargetView(back_buffer, std::ptr::null(), rtv_handle,);
            }

            let mut command_allocator: *mut ID3D12CommandAllocator = null_mut();
            let hr = (*device).CreateCommandAllocator(
                D3D12_COMMAND_LIST_TYPE_DIRECT,
                &<ID3D12CommandAllocator as winapi::Interface>::uuidof(),
                as_ppv!(command_allocator),
            );
            if SUCCEEDED(hr,)
            {
            }
            else
            {
            }

            FrameContext { command_allocator, back_buffer, rtv_handle, fence_value: 0, }
        },)
        .collect();

    G_RTV_DESCRIPTOR_HEAP = rtv_heap;
}

unsafe extern "system" fn hk_wnd_proc(hwnd: HWND, msg: u32, wparam: usize, lparam: isize,) -> isize {
    let result = ImGui_ImplWin32_WndProcHandler(hwnd as *const c_void, msg, wparam, lparam,);
    if result != 0
    {
        return 1;
    }
    match G_WNDPROC
    {
        Some(proc,) => CallWindowProcA(Some(proc,), hwnd, msg, wparam, lparam,),
        None => DefWindowProcA(hwnd, msg, wparam, lparam,),
    }
}

pub unsafe extern "system" fn hk_resize_buffers(
    swap_chain: *mut IDXGISwapChain,
    buffer_count: u32,
    width: u32,
    height: u32,
    new_format: DXGI_FORMAT,
    flags: u32,
) -> HRESULT {
    cleanup_render_target();

    let hr = G_RESIZE_BUFFERS.unwrap()(swap_chain, buffer_count, width, height, new_format, flags,);

    let mut device: *mut ID3D12Device = null_mut();
    let qi_hr = (*swap_chain).GetDevice(&<ID3D12Device as winapi::Interface>::uuidof(), as_ppv!(device),);
    if SUCCEEDED(qi_hr,) && !device.is_null()
    {
        init_or_update_render_targets(swap_chain, device,);

        (*device).Release();
    }
    else
    {
    }

    hr
}

pub unsafe extern "system" fn hk_present(swap_chain: *mut IDXGISwapChain3, sync_interval: u32, flags: u32,) -> HRESULT {
    let call_original = || G_PRESENT.unwrap_unchecked()(swap_chain, sync_interval, flags,);

    if !G_INITIALIZED.load(Ordering::SeqCst,)
    {
        let mut desc: winapi::shared::dxgi::DXGI_SWAP_CHAIN_DESC = std::mem::zeroed();
        let desc_hr = (*swap_chain).GetDesc(&mut desc,);
        if FAILED(desc_hr,)
        {
            return call_original();
        }

        G_HWND = desc.OutputWindow;

        let mut device: *mut ID3D12Device = null_mut();
        let dev_hr = (*swap_chain).GetDevice(&<ID3D12Device as winapi::Interface>::uuidof(), as_ppv!(device),);
        if FAILED(dev_hr,) || device.is_null()
        {
            return call_original();
        }

        if G_COMMAND_QUEUE.is_null()
        {
            if let Some(offset,) = get_command_queue_offset()
            {
                let sc_raw_ptr: *const c_void = std::mem::transmute_copy(&swap_chain,);

                let queue_ptr_addr = (sc_raw_ptr as *const u8).add(offset as usize,) as *const *mut c_void;
                let queue_ptr = *queue_ptr_addr;

                if queue_ptr.is_null()
                {
                }
                else
                {
                    G_COMMAND_QUEUE = queue_ptr as *mut ID3D12CommandQueue;
                }
            }
            else
            {
            }
        }

        if G_COMMAND_QUEUE.is_null()
        {
            return call_original();
        }

        {
            let mut fence: *mut ID3D12Fence = null_mut();
            let hr = (*device).CreateFence(
                0,
                D3D12_FENCE_FLAG_NONE,
                &<ID3D12Fence as winapi::Interface>::uuidof(),
                as_ppv!(fence),
            );
            if SUCCEEDED(hr,)
            {
                G_FENCE = fence;
            }
            else
            {
            }
        }

        G_FENCE_EVENT = CreateEventA(null_mut(), 0, 0, std::ptr::null(),);

        let sc_base = swap_chain as *mut IDXGISwapChain;
        init_or_update_render_targets(sc_base, device,);

        if G_FRAME_CONTEXTS.is_empty()
        {
            return call_original();
        }

        let font_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type:           D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            NumDescriptors: 1,
            Flags:          D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
            NodeMask:       0,
        };
        {
            let mut font_heap: *mut ID3D12DescriptorHeap = null_mut();
            let hr = (*device).CreateDescriptorHeap(
                &font_heap_desc,
                &<ID3D12DescriptorHeap as winapi::Interface>::uuidof(),
                as_ppv!(font_heap),
            );
            if FAILED(hr,)
            {
                return call_original();
            }

            G_FONT_DESCRIPTOR_HEAP = font_heap;
        }

        {
            let allocator = G_FRAME_CONTEXTS[0].command_allocator;
            if allocator.is_null()
            {
                return call_original();
            }
            let mut cmd_list: *mut ID3D12GraphicsCommandList = null_mut();
            let hr = (*device).CreateCommandList(
                0,
                D3D12_COMMAND_LIST_TYPE_DIRECT,
                allocator,
                null_mut(),
                &<ID3D12GraphicsCommandList as winapi::Interface>::uuidof(),
                as_ppv!(cmd_list),
            );
            if FAILED(hr,)
            {
                return call_original();
            }

            let close_hr = (*cmd_list).Close();
            if SUCCEEDED(close_hr,)
            {
            }
            else
            {
            }
            G_COMMAND_LIST = cmd_list;
        }

        igCreateContext(std::ptr::null_mut(),);

        ImGui_ImplWin32_Init(G_HWND as *const c_void,);

        {
            let font_heap = &*G_FONT_DESCRIPTOR_HEAP;
            let cpu_handle = (*font_heap).GetCPUDescriptorHandleForHeapStart();
            let gpu_handle = (*font_heap).GetGPUDescriptorHandleForHeapStart();

            let device_raw: *mut c_void = std::mem::transmute_copy(&device,);
            let heap_raw: *mut c_void = std::mem::transmute_copy(&G_FONT_DESCRIPTOR_HEAP,);

            let dx12_ok = ImGui_ImplDX12_Init(
                device_raw,
                desc.BufferCount as i32,
                desc.BufferDesc.Format,
                heap_raw,
                cpu_handle,
                gpu_handle,
            );

            if !dx12_ok
            {
                return call_original();
            }
        }

        let def = SetWindowLongPtrA(G_HWND, GWLP_WNDPROC, hk_wnd_proc as isize,);

        G_WNDPROC = Some(std::mem::transmute(def as usize,),);

        (*device).Release();

        let io = ffi::igGetIO_Nil();

        let fonts = io.as_mut_unchecked().Fonts;

        static RANGES: [ffi::ImWchar; 16] = [
            0x0020, 0x00FF, 0x2000, 0x206F, 0x3000, 0x30FF, 0x31F0, 0x31FF, 0xFF00, 0xFFEF, 0xFFFD, 0xFFFD, 0x4E00,
            0x9FAF, 0, 0,
        ];

        let font = ffi::ImFontAtlas_AddFontFromFileTTF(
            fonts,
            "C:\\Windows\\Fonts\\msyh.ttc\0".as_ptr().cast(),
            20.0,
            null_mut(),
            RANGES.as_ptr(),
        );

        io.as_mut_unchecked().FontDefault = font;

        ffi::igImFontAtlasBuildMain(fonts,);

        G_INITIALIZED.store(true, Ordering::SeqCst,);
    }

    let buffer_idx = (*swap_chain).GetCurrentBackBufferIndex() as usize;

    let current_frame = &mut G_FRAME_CONTEXTS[buffer_idx];

    if (*G_FENCE).GetCompletedValue() < current_frame.fence_value
    {
        let _ = (*G_FENCE).SetEventOnCompletion(current_frame.fence_value, G_FENCE_EVENT,);
        WaitForSingleObject(G_FENCE_EVENT, INFINITE,);
    }

    (*current_frame.command_allocator).Reset();

    (*G_COMMAND_LIST).Reset(current_frame.command_allocator, null_mut(),);

    let mut barrier: D3D12_RESOURCE_BARRIER = unsafe { std::mem::zeroed() };
    barrier.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
    unsafe {
        let trans = barrier.u.Transition_mut();
        (*trans).pResource = current_frame.back_buffer;
        (*trans).Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
        (*trans).StateBefore = D3D12_RESOURCE_STATE_PRESENT;
        (*trans).StateAfter = D3D12_RESOURCE_STATE_RENDER_TARGET;
    }

    (*G_COMMAND_LIST).ResourceBarrier(1, &barrier,);

    (*G_COMMAND_LIST).OMSetRenderTargets(1, &current_frame.rtv_handle, 0, null_mut(),);

    (*G_COMMAND_LIST).SetDescriptorHeaps(1, &mut G_FONT_DESCRIPTOR_HEAP,);

    ImGui_ImplDX12_NewFrame();
    ImGui_ImplWin32_NewFrame();
    igNewFrame();

    // igShowDemoWindow(null_mut(),);
    mui::render_fps_tool_demo();

    igRender();
    let draw_data = igGetDrawData();

    ImGui_ImplDX12_RenderDrawData(draw_data, G_COMMAND_LIST as *mut c_void,);

    unsafe {
        let trans = barrier.u.Transition_mut();
        (*trans).StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
        (*trans).StateAfter = D3D12_RESOURCE_STATE_PRESENT;
    }
    (*G_COMMAND_LIST).ResourceBarrier(1, &barrier,);

    (*G_COMMAND_LIST).Close();

    let cmd_list_for_execute: *mut winapi::um::d3d12::ID3D12CommandList = G_COMMAND_LIST as *mut _;
    (*G_COMMAND_QUEUE).ExecuteCommandLists(1, &cmd_list_for_execute,);

    G_FENCE_VALUE += 1;
    (*G_COMMAND_QUEUE).Signal(G_FENCE, G_FENCE_VALUE,);
    current_frame.fence_value = G_FENCE_VALUE;

    call_original()
}

type SteamCreateHookFn = unsafe extern "fastcall" fn(
    target_addr: u64,
    detour_addr: i64,
    pp_original: *mut u64,
    unknown_flag: i32,
    hook_name: *const u8,
) -> i64;

unsafe fn d3d12_hook_thread() {
    let overlay_module_name: Vec<u16,> = "GameOverlayRenderer64.dll\0".encode_utf16().collect();

    let overlay_handle = GetModuleHandleW(overlay_module_name.as_ptr(),);
    if overlay_handle.is_null()
    {
        return;
    }

    let mut mod_info: MODULEINFO = std::mem::zeroed();
    let success = GetModuleInformation(
        GetCurrentProcess(),
        overlay_handle,
        &mut mod_info,
        std::mem::size_of::<MODULEINFO,>() as u32,
    );

    if success == 0
    {
        return;
    }

    let scan_start = mod_info.lpBaseOfDll as *mut u8;
    let size_of_image = mod_info.SizeOfImage as usize;

    let create_hook_addr = sig_scan(scan_start, size_of_image, "48 89 5C 24 ? 57 48 83 EC ? 33 C0", 0, None,);

    let present_addr = sig_scan(
        scan_start,
        size_of_image,
        "48 89 5C 24 ? 48 89 6C 24 ? 56 57 41 54 41 56 41 57 48 83 EC ? 41 8B F0",
        0,
        None,
    );

    let resize_buffers_addr = sig_scan(
        scan_start,
        size_of_image,
        "40 53 55 56 57 41 54 41 56 41 57 48 83 EC ? 44 8B E2",
        0,
        None,
    );

    if create_hook_addr == 0 || present_addr == 0 || resize_buffers_addr == 0
    {
        return;
    }

    let steam_create_hook: SteamCreateHookFn = std::mem::transmute(create_hook_addr,);

    let mut original_present: u64 = 0;
    let name_present = b"DXGISwapChain_Present\0";
    steam_create_hook(present_addr as u64, hk_present as i64, &mut original_present, 1, name_present.as_ptr(),);

    if original_present == 0
    {
        return;
    }

    let mut original_resizebuffers: u64 = 0;
    let name_resize = b"DXGISwapChain_ResizeBuffers\0";
    steam_create_hook(
        resize_buffers_addr as u64,
        hk_resize_buffers as i64,
        &mut original_resizebuffers,
        1,
        name_resize.as_ptr(),
    );

    if original_resizebuffers == 0
    {
        return;
    }

    G_PRESENT = Some(std::mem::transmute(original_present,),);
    G_RESIZE_BUFFERS = Some(std::mem::transmute(original_resizebuffers,),);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(instance: HINSTANCE, reason: u32, _: *mut c_void,) -> BOOL {
    unsafe {
        if reason == 1
        {
            DisableThreadLibraryCalls(instance,);
            CreateThread(
                null_mut(),
                0,
                Some(std::mem::transmute(d3d12_hook_thread as unsafe fn(),),),
                null_mut(),
                0,
                null_mut(),
            );
        }
    }
    TRUE
}

#[inline]
pub unsafe fn sig_scan(
    scan_start: *mut u8,
    size_of_image: usize,
    pattern: &str,
    offset: isize,
    direct_offset: Option<isize,>,
) -> usize {
    let mut pattern_bytes = [0i16; 128];
    let mut pattern_len = 0;

    for s in pattern.split_whitespace()
    {
        if pattern_len >= 128
        {
            break;
        }
        pattern_bytes[pattern_len] =
            if s == "?" || s == "??" { -1 } else { u8::from_str_radix(s, 16,).unwrap_or(0,) as i16 };
        pattern_len += 1;
    }

    if pattern_len == 0 || pattern_len > size_of_image
    {
        return 0;
    }

    for i in 0..=(size_of_image - pattern_len)
    {
        let mut found = true;

        for j in 0..pattern_len
        {
            let p_byte = pattern_bytes[j];
            if p_byte != -1 && *scan_start.add(i + j,) != p_byte as u8
            {
                found = false;
                break;
            }
        }

        if found
        {
            let sig_addr = (scan_start as usize) + i + offset as usize;

            return match direct_offset
            {
                Some(doff,) =>
                {
                    let disp_ptr = (sig_addr as isize + doff) as *const i32;
                    let displacement = std::ptr::read_unaligned(disp_ptr,) as isize;
                    let rip_base = (disp_ptr as usize) + 4;
                    (rip_base as isize + displacement) as usize
                }
                None => sig_addr,
            };
        }
    }
    0
}
