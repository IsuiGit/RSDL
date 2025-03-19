use crate::sdl3::{
    SDL3,
    SDL_Delay,
    SDL_GetError,
    SDL_SetRenderDrawColor,
    SDL_PollEvent,
    SDL_RenderClear,
    SDL_RenderPresent
};
use libloading::Symbol;
use std::ffi::{
    c_void,
    CStr
};

pub fn sdl3_delay(sdl3: &mut SDL3, ms: u32){
    unsafe {
        let _sdl3_delay: Symbol<SDL_Delay> = sdl3.lib.get(b"SDL_Delay").expect("Failed to get symbol SDL_Delay");
        _sdl3_delay(ms);
    }
}

pub fn sdl3_set_renderer_draw_color(sdl3: &mut SDL3, renderer: *mut c_void, r: u8, g: u8, b: u8, a: u8) -> bool {
    unsafe {
        let _sdl3_set_renderer_deaw_color: Symbol<SDL_SetRenderDrawColor> = sdl3.lib.get(b"SDL_SetRenderDrawColor").expect("Failed to get symbol SDL_SetRenderDrawColor");
        _sdl3_set_renderer_deaw_color(renderer, r, g, b, a)
    }
}

pub fn sdl3_render_clear(sdl3: &mut SDL3, renderer: *mut c_void) -> bool {
    unsafe {
        let _sdl3_render_clear: Symbol<SDL_RenderClear> = sdl3.lib.get(b"SDL_RenderClear").expect("Failed to get symbol SDL_RenderClear");
        _sdl3_render_clear(renderer)
    }
}

pub fn sdl3_render_present(sdl3: &mut SDL3, renderer: *mut c_void) -> bool {
    unsafe {
        let _sdl3_render_present: Symbol<SDL_RenderPresent> = sdl3.lib.get(b"SDL_RenderPresent").expect("Failed to get symbol SDL_RenderPresent");
        _sdl3_render_present(renderer)
    }
}

pub fn sdl3_get_error(sdl3: &mut SDL3) -> String {
    unsafe {
        let _sdl3_get_error: Symbol<SDL_GetError> = sdl3.lib.get(b"SDL_GetError").expect("Failed to get symbol SDL_GetError");
        let c_err = CStr::from_ptr(_sdl3_get_error());
        c_err.to_string_lossy().into_owned()
    }
}

pub fn sdl3_poll_event(sdl3: &mut SDL3) -> Symbol<SDL_PollEvent> {
    unsafe {
        let _sdl3_poll_event: Symbol<SDL_PollEvent> = sdl3.lib.get(b"SDL_PollEvent").expect("Failed to get symbol SDL_PollEvent");
        _sdl3_poll_event
    }
}
