fn main() {
    let vulkan_sdk = std::env::var("VULKAN_SDK").expect("No VulkanSDK installation detected");
    println!("cargo:rustc-link-search=native={}/Lib", &vulkan_sdk);
    println!("cargo:rustc-link-lib=static=vulkan-1");

    let tf = std::path::Path::new("The-Forge");
    let tfos = tf.join("Common_3/ThirdParty/OpenSource/");

    let vulkan_sdk = std::path::Path::new(&vulkan_sdk);
    let vulkan_include = vulkan_sdk.join("Include");

    cc::Build::new()
        .define("_WINDOWS", None)
        .file(tfos.join("SPIRV_Cross/spirv_cfg.cpp"))
        .file(tfos.join("SPIRV_Cross/spirv_cpp.cpp"))
        .file(tfos.join("SPIRV_Cross/spirv_cross.cpp"))
        .file(tfos.join("SPIRV_Cross/spirv_cross_parsed_ir.cpp"))
        .file(tfos.join("SPIRV_Cross/spirv_cross_util.cpp"))
        .file(tfos.join("SPIRV_Cross/spirv_glsl.cpp"))
        .file(tfos.join("SPIRV_Cross/spirv_hlsl.cpp"))
        .file(tfos.join("SPIRV_Cross/spirv_msl.cpp"))
        .file(tfos.join("SPIRV_Cross/spirv_parser.cpp"))
        .file(tfos.join("SPIRV_Cross/spirv_reflect.cpp"))
        .file(tf.join("Common_3/Tools/SpirvTools/SpirvTools.cpp"))
        .compile("SpirvCross");

    cc::Build::new()
        .define("_WINDOWS", None)
        .define("GAINPUT_LIB_DYNAMIC", Some("0"))
        .file(tfos.join("gainput/lib/source/gainput/GainputAllocator.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/GainputInputDeltaState.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/GainputInputDevice.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/GainputInputManager.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/GainputInputMap.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/GainputInputState.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/GainputMapFilters.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/builtin/GainputInputDeviceBuiltIn.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/dev/GainputDev.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/dev/GainputMemoryStream.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/gainput.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/gestures/GainputButtonStickGesture.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/gestures/GainputDoubleClickGesture.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/gestures/GainputHoldGesture.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/gestures/GainputPinchGesture.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/gestures/GainputRotateGesture.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/gestures/GainputSimultaneouslyDownGesture.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/gestures/GainputTapGesture.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/keyboard/GainputInputDeviceKeyboard.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/mouse/GainputInputDeviceMouse.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/pad/GainputInputDevicePad.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/recorder/GainputInputPlayer.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/recorder/GainputInputRecorder.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/recorder/GainputInputRecording.cpp"))
        .file(tfos.join("gainput/lib/source/gainput/touch/GainputInputDeviceTouch.cpp"))
        .compile("gainputstatic");

    // cc::Build::new()
    //     .define("_WINDOWS", None)
    //     .define("_MBCS", None)
    //     .include(tfos.join("ozz-animation/include"))
    //     .file(tfos.join("ozz-animation/src/base/memory/allocator.cc"))
    //     .file(tfos.join("ozz-animation/src/base/platform.cc"))
    //     .file(tfos.join("ozz-animation/src/base/containers/string_archive.cc"))
    //     .file(tfos.join("ozz-animation/src/base/io/archive.cc"))
    //     .file(tfos.join("ozz-animation/src/base/maths/math_archive.cc"))
    //     .file(tfos.join("ozz-animation/src/base/maths/soa_math_archive.cc"))
    //     .file(tfos.join("ozz-animation/src/base/maths/simd_math_archive.cc"))
    //     .compile("ozz_base");

    // cc::Build::new()
    //     .define("_WINDOWS", None)
    //     .define("_MBCS", None)
    //     .include(tfos.join("ozz-animation/include"))
    //     .file(tfos.join("ozz-animation/src/animation/offline/raw_animation.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/offline/raw_animation_archive.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/offline/raw_animation_utils.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/offline/animation_builder.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/offline/animation_optimizer.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/offline/additive_animation_builder.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/offline/raw_skeleton.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/offline/raw_skeleton_archive.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/offline/skeleton_builder.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/offline/raw_track.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/offline/track_builder.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/offline/track_optimizer.cc"))
    //     .compile("ozz_animation_offline");

    // cc::Build::new()
    //     .define("_WINDOWS", None)
    //     .define("_MBCS", None)
    //     .include(tfos.join("ozz-animation/include"))
    //     .file(tfos.join("ozz-animation/src/animation/runtime/animation.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/runtime/blending_job.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/runtime/ik_aim_job.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/runtime/ik_two_bone_job.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/runtime/local_to_model_job.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/runtime/sampling_job.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/runtime/skeleton.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/runtime/skeleton_utils.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/runtime/track.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/runtime/track_sampling_job.cc"))
    //     .file(tfos.join("ozz-animation/src/animation/runtime/track_triggering_job.cc"))
    //     .compile("ozz_animation");

    cc::Build::new()
        .define("_WINDOWS", None)
        .define("DIRECT3D12", None)
        .define("VULKAN", None)
        .include(&vulkan_include)
        .object("ws2_32.lib")
        .object("shell32.lib")
        .object("user32.lib")
        .object("ole32.lib")
        .object("gdi32.lib")
        .object("Xinput9_1_0.lib")
        .file(tf.join("Common_3/OS/Camera/CameraController.cpp"))
        .file(tf.join("Common_3/OS/Core/Screenshot.cpp"))
        .file(tf.join("Common_3/OS/Core/ThreadSystem.cpp"))
        .file(tf.join("Common_3/OS/Core/Timer.c"))
        .file(tf.join("Common_3/OS/FileSystem/FileSystem.cpp"))
        .file(tf.join("Common_3/OS/FileSystem/SystemRun.cpp"))
        .file(tf.join("Common_3/OS/Fonts/FontSystem.cpp"))
        .file(tf.join("Common_3/OS/FileSystem/ZipFileSystem.c"))
        .file(tf.join("Common_3/OS/Input/InputSystem.cpp"))
        .file(tf.join("Common_3/OS/MemoryTracking/MemoryTracking.c"))
        .file(tf.join("Common_3/OS/Profiler/GpuProfiler.cpp"))
        .file(tf.join("Common_3/OS/Profiler/ProfilerBase.cpp"))
        .file(tf.join("Common_3/OS/Scripting/LuaManager.cpp"))
        .file(tf.join("Common_3/OS/Scripting/LuaManagerImpl.cpp"))
        .file(tf.join("Common_3/OS/Scripting/LuaSystem.cpp"))
        .file(tf.join("Common_3/OS/UI/UI.cpp"))
        .file(tf.join("Common_3/OS/Windows/WindowsStackTraceDump.cpp"))
        .file(tf.join("Common_3/OS/Windows/WindowsFileSystem.cpp"))
        .file(tf.join("Common_3/OS/Windows/WindowsTime.c"))
        .file(tf.join("Common_3/OS/Logging/Log.c"))
        .file(tf.join("Common_3/OS/Windows/WindowsLog.c"))
        .file(tf.join("Common_3/OS/Windows/WindowsBase.cpp"))
        .file(tf.join("Common_3/OS/Windows/WindowsThread.c"))
        .file(tfos.join("basis_universal/transcoder/basisu_transcoder.cpp"))
        .file(tfos.join("EASTL/allocator_forge.cpp"))
        .file(tfos.join("EASTL/assert.cpp"))
        .file(tfos.join("EASTL/EAStdC/EAMemory.cpp"))
        .file(tfos.join("EASTL/EAStdC/EASprintf.cpp"))
        .file(tfos.join("EASTL/fixed_pool.cpp"))
        .file(tfos.join("EASTL/hashtable.cpp"))
        .file(tfos.join("EASTL/intrusive_list.cpp"))
        .file(tfos.join("EASTL/numeric_limits.cpp"))
        .file(tfos.join("EASTL/red_black_tree.cpp"))
        .file(tfos.join("EASTL/string.cpp"))
        .file(tfos.join("EASTL/thread_support.cpp"))
        .file(tfos.join("imgui/imgui.cpp"))
        .file(tfos.join("imgui/imgui_demo.cpp"))
        .file(tfos.join("imgui/imgui_draw.cpp"))
        .file(tfos.join("imgui/imgui_widgets.cpp"))
        .file(tfos.join("lua-5.3.5/src/lapi.c"))
        .file(tfos.join("lua-5.3.5/src/lauxlib.c"))
        .file(tfos.join("lua-5.3.5/src/lbaselib.c"))
        .file(tfos.join("lua-5.3.5/src/lbitlib.c"))
        .file(tfos.join("lua-5.3.5/src/lcode.c"))
        .file(tfos.join("lua-5.3.5/src/lcorolib.c"))
        .file(tfos.join("lua-5.3.5/src/lctype.c"))
        .file(tfos.join("lua-5.3.5/src/ldblib.c"))
        .file(tfos.join("lua-5.3.5/src/ldebug.c"))
        .file(tfos.join("lua-5.3.5/src/ldo.c"))
        .file(tfos.join("lua-5.3.5/src/ldump.c"))
        .file(tfos.join("lua-5.3.5/src/lfunc.c"))
        .file(tfos.join("lua-5.3.5/src/lgc.c"))
        .file(tfos.join("lua-5.3.5/src/linit.c"))
        .file(tfos.join("lua-5.3.5/src/liolib.c"))
        .file(tfos.join("lua-5.3.5/src/llex.c"))
        .file(tfos.join("lua-5.3.5/src/lmathlib.c"))
        .file(tfos.join("lua-5.3.5/src/lmem.c"))
        .file(tfos.join("lua-5.3.5/src/loadlib.c"))
        .file(tfos.join("lua-5.3.5/src/lobject.c"))
        .file(tfos.join("lua-5.3.5/src/lopcodes.c"))
        .file(tfos.join("lua-5.3.5/src/loslib.c"))
        .file(tfos.join("lua-5.3.5/src/lparser.c"))
        .file(tfos.join("lua-5.3.5/src/lstate.c"))
        .file(tfos.join("lua-5.3.5/src/lstring.c"))
        .file(tfos.join("lua-5.3.5/src/lstrlib.c"))
        .file(tfos.join("lua-5.3.5/src/ltable.c"))
        .file(tfos.join("lua-5.3.5/src/ltablib.c"))
        .file(tfos.join("lua-5.3.5/src/ltm.c"))
        .file(tfos.join("lua-5.3.5/src/lundump.c"))
        .file(tfos.join("lua-5.3.5/src/lutf8lib.c"))
        .file(tfos.join("lua-5.3.5/src/lvm.c"))
        .file(tfos.join("lua-5.3.5/src/lzio.c"))
        .file(tfos.join("minizip/lib/brg/aescrypt.c"))
        .file(tfos.join("minizip/lib/brg/aeskey.c"))
        .file(tfos.join("minizip/lib/brg/aestab.c"))
        .file(tfos.join("minizip/lib/brg/hmac.c"))
        .file(tfos.join("minizip/lib/brg/sha1.c"))
        .file(tfos.join("minizip/lib/brg/sha2.c"))
        .file(tfos.join("minizip/mz_crypt.c"))
        .file(tfos.join("minizip/mz_crypt_brg.c"))
        .file(tfos.join("minizip/mz_os.cpp"))
        .file(tfos.join("minizip/mz_strm_raw.c"))
        .file(tfos.join("minizip/mz_strm_wzaes.c"))
        .file(tfos.join("minizip/mz_strm_zlib.c"))
        .file(tfos.join("minizip/mz_zip.c"))
        .file(tfos.join("rmem/src/rmem_get_module_info.cpp"))
        .file(tfos.join("rmem/src/rmem_hook.cpp"))
        .file(tfos.join("rmem/src/rmem_lib.cpp"))
        // .file(tf.join("Middleware_3/Animation/AnimatedObject.cpp"))
        // .file(tf.join("Middleware_3/Animation/Animation.cpp"))
        // .file(tf.join("Middleware_3/Animation/Clip.cpp"))
        // .file(tf.join("Middleware_3/Animation/ClipController.cpp"))
        // .file(tf.join("Middleware_3/Animation/ClipMask.cpp"))
        // .file(tf.join("Middleware_3/Animation/Rig.cpp"))
        // .file(tf.join("Middleware_3/Animation/SkeletonBatcher.cpp"))
        .compile("The-Forge-OS");

    cc::Build::new()
        .define("_WINDOWS", None)
        .define("DIRECT3D12", None)
        .define("VULKAN", None)
        .include(&vulkan_include)
        .object(tfos.join("DirectXShaderCompiler/lib/x64/dxcompiler.lib"))
        .object(tfos.join("winpixeventruntime/bin/WinPixEventRuntime.lib"))
        .object(tfos.join("ags/ags_lib/lib/amd_ags_x64.lib"))
        .object(tfos.join("nvapi/amd64/nvapi64.lib"))
        .file(tf.join("Common_3/Renderer/Renderer.cpp"))
        .file(tf.join("Common_3/Renderer/ResourceLoader.cpp"))
        .file(tf.join("Common_3/Renderer/CommonShaderReflection.cpp"))
        .file(tf.join("Common_3/Renderer/Direct3D12/Direct3D12.cpp"))
        .file(tf.join("Common_3/Renderer/Direct3D12/Direct3D12Hooks.cpp"))
        .file(tf.join("Common_3/Renderer/Direct3D12/Direct3D12Raytracing.cpp"))
        .file(tf.join("Common_3/Renderer/Direct3D12/Direct3D12ShaderReflection.cpp"))
        .file(tf.join("Common_3/Renderer/Vulkan/Vulkan.cpp"))
        .file(tf.join("Common_3/Renderer/Vulkan/VulkanRaytracing.cpp"))
        .file(tf.join("Common_3/Renderer/Vulkan/VulkanShaderReflection.cpp"))
        .compile("The-Forge-Renderer");

    cc::Build::new()
        .define("_WINDOWS", None)
        .define("DIRECT3D12", None)
        .define("VULKAN", None)
        .include(&vulkan_include)
        .include(&tf)
        .file("src/os_wrapper.cpp")
        .file("src/app_wrapper.cpp")
        .compile("RustForgeWrapper");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR undefined");
    let out_dir = std::path::Path::new(&out_dir).join("..").join("..").join("..");
    copy_dll(
        "dxcompiler.dll",
        tf.join("Common_3/ThirdParty/OpenSource/DirectXShaderCompiler/bin/x64"),
        &out_dir,
    );
    copy_dll(
        "dxil.dll",
        tf.join("Common_3/ThirdParty/OpenSource/DirectXShaderCompiler/bin/x64"),
        &out_dir,
    );
    copy_dll(
        "amd_ags_x64.dll",
        tf.join("Common_3/ThirdParty/OpenSource/ags/ags_lib/lib/"),
        &out_dir,
    );
    copy_dll(
        "WinPixEventRuntime.dll",
        tf.join("Common_3/ThirdParty/OpenSource/winpixeventruntime/bin/"),
        &out_dir,
    );
}

fn copy_dll(dll: &str, from: std::path::PathBuf, to: &std::path::Path) {
    std::fs::copy(from.join(dll), to.join(dll)).expect("failed to copy dll");
}
