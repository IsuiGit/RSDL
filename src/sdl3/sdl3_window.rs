// INCLUDE SDL3 MODS-------------------------------------------------------------------------------
use crate::sdl3::{
    SDL3,
    SDL_CreateWindow,
    SDL_DestroyWindow,
    SDL_GetWindowSize
};
// ------------------------------------------------------------------------------------------------
// STANDART MODS-----------------------------------------------------------------------------------
use libloading::Symbol;
use std::ffi::{c_void, CString};

pub fn sdl3_create_window(sdl3: &mut SDL3, title: &str, w:u32, h:u32, flags:u64) -> *mut c_void {
    unsafe{
        let title_ptr = CString::new(title).unwrap();
        let _sdl3_create_window: Symbol<SDL_CreateWindow> = sdl3.lib.get(b"SDL_CreateWindow")
            .expect("Failed to get symbol SDL_CreateWindow");
        _sdl3_create_window(title_ptr.as_ptr(), w, h, flags)
    }
}

pub fn sdl3_get_window_size(sdl3: &mut SDL3, window: *mut c_void) -> (i32, i32) {
    unsafe {
        let _sdl3_get_window_size: Symbol<SDL_GetWindowSize> = sdl3.lib.get(b"SDL_GetWindowSize")
            .expect("Failed to get symbol SDL_GetWindowSize");
        let mut _w = 0;
        let mut _h = 0;
        if _sdl3_get_window_size(window, &mut _w, &mut _h){
            (_w, _h)
        }
        else {
            (0, 0)
        }

    }
}

pub fn sdl3_destroy_window(sdl3: &mut SDL3, window: *mut c_void) {
    unsafe {
        let _sdl3_destroy_window: Symbol<SDL_DestroyWindow> = sdl3.lib.get(b"SDL_DestroyWindow")
            .expect("Failed to get symbol SDL_DestroyWindow");
        _sdl3_destroy_window(window);
    }
}
