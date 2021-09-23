#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

mod utils;

mod forge_os;
mod forge_ray;
mod forge_renderer;
mod tiny_image_format;

pub mod os {
    pub use crate::forge_os::*;
    use std::os::raw::c_char;

    extern "C" {
        #[link_name = "wrap_fsFileModeFromString"]
        pub fn fsFileModeFromString(modeStr: *const c_char) -> FileMode;

        #[link_name = "wrap_fsFileModeToString"]
        pub fn fsFileModeToString(mode: FileMode) -> *const c_char;

        #[link_name = "wrap_fsOverwriteFileModeToString"]
        pub fn fsOverwriteFileModeToString(mode: FileMode) -> *const c_char;
    }
}

pub mod renderer {
    pub use crate::forge_renderer::*;
    pub use crate::tiny_image_format::*;
}

pub mod raytracing {
    pub use crate::forge_ray::*;
}

pub trait ForgeAppInterface {
    fn new() -> Self;
    fn init(&mut self) -> bool;
    fn exit(&mut self);
    fn load(&mut self) -> bool;
    fn unload(&mut self);
    fn update(&mut self, delta_time: f32);
    fn draw(&mut self);
}

#[macro_export]
macro_rules! define_application_main {
    ($type: ty) => {
        unsafe extern "C" fn init(ptr: *mut std::ffi::c_void) -> bool {
            (*(ptr as *mut $type)).init()
        }

        unsafe extern "C" fn exit(ptr: *mut std::ffi::c_void) {
            (*(ptr as *mut $type)).exit()
        }

        unsafe extern "C" fn load(ptr: *mut std::ffi::c_void) -> bool {
            (*(ptr as *mut $type)).load()
        }

        unsafe extern "C" fn unload(ptr: *mut std::ffi::c_void) {
            (*(ptr as *mut $type)).unload()
        }

        unsafe extern "C" fn update(ptr: *mut std::ffi::c_void, delta_time: f32) {
            (*(ptr as *mut $type)).update(delta_time)
        }

        unsafe extern "C" fn draw(ptr: *mut std::ffi::c_void) {
            (*(ptr as *mut $type)).draw()
        }

        unsafe extern "C" fn get_name(_: *mut std::ffi::c_void) -> *const std::os::raw::c_char {
            b"The Rusty Forge\0".as_ptr() as _
        }

        #[repr(C)]
        pub struct RustAppGlue {
            pub init: unsafe extern "C" fn(ptr: *mut std::ffi::c_void) -> bool,
            pub exit: unsafe extern "C" fn(ptr: *mut std::ffi::c_void),
            pub load: unsafe extern "C" fn(ptr: *mut std::ffi::c_void) -> bool,
            pub unload: unsafe extern "C" fn(ptr: *mut std::ffi::c_void),
            pub update: unsafe extern "C" fn(ptr: *mut std::ffi::c_void, delta_time: f32),
            pub draw: unsafe extern "C" fn(ptr: *mut std::ffi::c_void),
            pub get_name: unsafe extern "C" fn(ptr: *mut std::ffi::c_void) -> *const std::os::raw::c_char,
        }

        extern "C" {
            fn the_forge_main(app: *mut $type, glue: *mut RustAppGlue) -> i32;
        }

        fn main() {
            let mut app = <$type>::new();
            let mut glue = RustAppGlue {
                init,
                exit,
                load,
                unload,
                update,
                draw,
                get_name,
            };

            let error_code = unsafe { the_forge_main(&mut app, &mut glue) };
            assert_eq!(error_code, 0);
        }
    };
}
