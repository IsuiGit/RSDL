// INCLUDE SDL3 MODS-------------------------------------------------------------------------------
use crate::sdl3::{
    SDL3,
    SDL_Init,
    TTF_Init,
    SDL_Delay,
    SDL_GetError,
    SDL_PollEvent,
    SDL_PushEvent,
    SDL_Quit,
    TTF_Quit,
    SDL_GL_GetCurrentWindow,
    SDL3_GetRenderer,
    sdl3_structs::SDL_Event
};
// ------------------------------------------------------------------------------------------------
// STANDART MODS-----------------------------------------------------------------------------------
use libloading::Symbol;
use std::ffi::{CStr, c_void};
// ------------------------------------------------------------------------------------------------
pub fn sdl3_init(sdl3: &mut SDL3, flags: u32) {
    unsafe {
        let _sdl3_init: Symbol<SDL_Init> = sdl3.lib.get(b"SDL_Init")
            .expect("Failed to get symbol SDL_Init");
        if !_sdl3_init(flags) {
            panic!("SDL could not initialize! SDL_Error: {}", sdl3_get_error(sdl3));
        }
    }
}

pub fn sdl3_ttf_init(sdl3: &mut SDL3){
    unsafe {
        let _sdl3_ttf_init: Symbol<TTF_Init> = sdl3.ttf_lib.get(b"TTF_Init")
            .expect("Failed to get symbol TTF_Init");
        if !_sdl3_ttf_init(){
            panic!("SDL_ttf could not initialize! SDL_Error: {}", sdl3_get_error(sdl3));
        }
    }
}

pub fn sdl3_delay(sdl3: &mut SDL3, ms: u32){
    unsafe {
        let _sdl3_delay: Symbol<SDL_Delay> = sdl3.lib.get(b"SDL_Delay")
            .expect("Failed to get symbol SDL_Delay");
        _sdl3_delay(ms);
    }
}

pub fn sdl3_get_error(sdl3: &mut SDL3) -> String {
    unsafe {
        let _sdl3_get_error: Symbol<SDL_GetError> = sdl3.lib.get(b"SDL_GetError")
            .expect("Failed to get symbol SDL_GetError");
        let c_err = CStr::from_ptr(_sdl3_get_error());
        c_err.to_string_lossy().into_owned()
    }
}

pub fn sdl3_poll_event(sdl3: &mut SDL3) -> Symbol<SDL_PollEvent> {
    unsafe {
        let _sdl3_poll_event: Symbol<SDL_PollEvent> = sdl3.lib.get(b"SDL_PollEvent")
            .expect("Failed to get symbol SDL_PollEvent");
        _sdl3_poll_event
    }
}

pub fn sdl3_push_event(sdl3: &mut SDL3, event: *mut SDL_Event) -> bool {
    unsafe {
        let _sdl3_push_event: Symbol<SDL_PushEvent> = sdl3.lib.get(b"SDL_PushEvent")
            .expect("Failed to get symbol SDL_PushEvent");
        _sdl3_push_event(event)
    }
}

pub fn sdl3_quit(sdl3: &mut SDL3){
    unsafe {
        let _sdl3_quit: Symbol<SDL_Quit> = sdl3.lib.get(b"SDL_Quit")
            .expect("Failed to get symbol SDL_Quit");
        _sdl3_quit();
    }
}

pub fn sdl3_ttf_quit(sdl3: &mut SDL3){
    unsafe {
        let _sdl3_ttf_quit: Symbol<TTF_Quit> = sdl3.ttf_lib.get(b"TTF_Quit")
            .expect("Failed to get symbol TTF_Quit");
        _sdl3_ttf_quit();
    }
}

pub fn sdl3_gl_get_current_window(sdl3: &mut SDL3) -> *mut c_void{
    unsafe{
        let _sdl3_gl_get_current_window: Symbol<SDL_GL_GetCurrentWindow> = sdl3.lib.get(b"SDL_GL_GetCurrentWindow")
            .expect("Failed to get symbol SDL_GL_GetCurrentWindow");
        let window = _sdl3_gl_get_current_window();
        if !window.is_null(){ window } else { panic!("SDL_GL_GetCurrentWindow could not found a window! SDL_Error: {}", sdl3_get_error(sdl3)); }
    }
}

pub fn sdl3_get_renderer(sdl3: &mut SDL3, window: *mut c_void) -> *mut c_void{
    unsafe{
        let _sdl3_get_renderer: Symbol<SDL3_GetRenderer> = sdl3.lib.get(b"SDL3_GetRenderer")
            .expect("Failed to get symbol SDL3_GetRenderer");
        let renderer = _sdl3_get_renderer(window);
        if !renderer.is_null(){ renderer } else { panic!("SDL_GetRenderer could not found a window! SDL_Error: {}", sdl3_get_error(sdl3)); }
    }
}
