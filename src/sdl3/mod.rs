// INCLUDE SDL3 MODS-------------------------------------------------------------------------------
mod sdl3_types;
pub mod sdl3_consts;
pub mod sdl3_structs;
pub mod sdl3_sys;
pub mod sdl3_render;

use sdl3_types::*;
// ------------------------------------------------------------------------------------------------
// INCLUDE STANDART MODS---------------------------------------------------------------------------
use libloading::{Library, Symbol};
use std::{path::Path, ffi::{c_void, CString}};
// ------------------------------------------------------------------------------------------------
// SDL3 MAIN STRUCT--------------------------------------------------------------------------------
pub struct SDL3{
    lib: Library,
}
// ------------------------------------------------------------------------------------------------
impl SDL3{
    pub fn new() -> Self {
        unsafe {
            let lib = Library::new(Path::new("src/sdl3/bin/SDL3.dll"))
                .expect("Failed to load SDL3.dll");
            SDL3 {lib}
        }
    }

    pub fn sdl3_init(&mut self, flags: u32) {
        unsafe {
            let _sdl3_init: Symbol<SDL_Init> = self.lib.get(b"SDL_Init")
                .expect("Failed to get symbol SDL_Init");
            if !_sdl3_init(flags) {
                panic!("SDL could not initialize! SDL_Error: {}", sdl3_sys::sdl3_get_error(self));
            }
        }
    }

    pub fn sdl3_quit(&mut self){
        unsafe {
            let _sdl3_quit: Symbol<SDL_Quit> = self.lib.get(b"SDL_Quit")
                .expect("Failed to get symbol SDL_Quit");
            _sdl3_quit();
        }
    }

    pub fn sdl3_create_window(&mut self, title: &str, w:u32, h:u32, flags:u32) -> *mut c_void {
        unsafe{
            let title_ptr = CString::new(title).unwrap();
            let _sdl3_create_window: Symbol<SDL_CreateWindow> = self.lib.get(b"SDL_CreateWindow")
                .expect("Failed to get symbol SDL_CreateWindow");
            _sdl3_create_window(title_ptr.as_ptr(), w, h, flags)
        }
    }

    pub fn sdl3_create_renderer(&mut self, window: *mut c_void, name: &str) -> *mut c_void {
        unsafe{
            let ptr_name = CString::new(name).unwrap();
            let _sdl3_create_renderer: Symbol<SDL_CreateRenderer> = self.lib.get(b"SDL_CreateRenderer")
                .expect("Failed to get symbol SDL_CreateRenderer");
            _sdl3_create_renderer(window, ptr_name.as_ptr())
        }
    }

    pub fn sdl3_destroy_window(&mut self, window: *mut c_void) {
        unsafe {
            let _sdl3_destroy_window: Symbol<SDL_DestroyWindow> = self.lib.get(b"SDL_DestroyWindow")
                .expect("Failed to get symbol SDL_DestroyWindow");
            _sdl3_destroy_window(window);
        }
    }

    pub fn sdl3_destroy_renderer(&mut self, renderer: *mut c_void) {
        unsafe {
            let _sdl3_destroy_renderer: Symbol<SDL_DestroyRenderer> = self.lib.get(b"SDL_DestroyRenderer")
                .expect("Failed to get symbol SDL_DestroyRenderer");
            _sdl3_destroy_renderer(renderer);
        }
    }
}
