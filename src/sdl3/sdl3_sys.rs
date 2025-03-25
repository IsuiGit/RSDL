// INCLUDE SDL3 MODS-------------------------------------------------------------------------------
use crate::sdl3::{
    SDL3,
    SDL_Init,
    SDL_Delay,
    SDL_GetError,
    SDL_PollEvent,
    SDL_Quit
};
// ------------------------------------------------------------------------------------------------
// STANDART MODS-----------------------------------------------------------------------------------
use libloading::Symbol;
use std::ffi::CStr;
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

pub fn sdl3_quit(sdl3: &mut SDL3){
    unsafe {
        let _sdl3_quit: Symbol<SDL_Quit> = sdl3.lib.get(b"SDL_Quit")
            .expect("Failed to get symbol SDL_Quit");
        _sdl3_quit();
    }
}
