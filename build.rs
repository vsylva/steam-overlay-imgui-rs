fn main() -> std::io::Result<(),> {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"),);

    let cimgui_dir = manifest_dir.join("cimgui",);
    let imgui_dir = cimgui_dir.join("imgui",);

    println!("cargo:rustc-link-lib=d3d12");
    println!("cargo:rustc-link-lib=dxgi");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=shcore");

    let mut build = cc::Build::new();
    build.cpp(true,);

    build
        // .define("IMGUI_DISABLE_OBSOLETE_FUNCTIONS", None,)
        // .define("IMGUI_USE_WCHAR32", None,)
        .define("CIMGUI_NO_EXPORT", None,)
        // .define("IMGUI_DISABLE_WIN32_FUNCTIONS", None,)
        .define("IMGUI_DISABLE_OSX_FUNCTIONS", None,);

    build.static_crt(true,);

    if build.get_compiler().is_like_msvc()
    {
        build.flag("/std:c++23",);
        build.flag("/EHsc",);
    }

    build.include(&cimgui_dir,);
    build.include(&imgui_dir,);

    let files = [
        cimgui_dir.join("cimgui.cpp",),
        imgui_dir.join("imgui.cpp",),
        imgui_dir.join("imgui_draw.cpp",),
        imgui_dir.join("imgui_tables.cpp",),
        imgui_dir.join("imgui_widgets.cpp",),
        // imgui_dir.join("imgui_impl_dx11.cpp",),
        imgui_dir.join("imgui_impl_dx12.cpp",),
        imgui_dir.join("imgui_impl_win32.cpp",),
        imgui_dir.join("imgui_demo.cpp",),
    ];

    for file in files.iter()
    {
        build.file(file,);
    }

    build.compile("libimgui",);

    Ok((),)
}
