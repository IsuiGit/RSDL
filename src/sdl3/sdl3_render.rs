// INCLUDE SDL3 MODS-------------------------------------------------------------------------------
use crate::sdl3::{
    SDL3,
    SDL_CreateRenderer,
    SDL_CreateTextureFromSurface,
    SDL_SetRenderDrawColor,
    SDL_RenderClear,
    SDL_RenderPresent,
    SDL_RenderRect,
    SDL_RenderFillRect,
    SDL_RenderRects,
    SDL_RenderFillRects,
    SDL_DestroyRenderer,
    SDL_RenderTexture,
    sdl3_structs::*,
    sdl3_sys::sdl3_get_error
};
// ------------------------------------------------------------------------------------------------
// STANDART MODS-----------------------------------------------------------------------------------
use libloading::Symbol;
use std::{ffi::{c_void, CString}, ptr::null};
//-------------------------------------------------------------------------------------------------
pub fn sdl3_create_renderer(sdl3: &mut SDL3, window: *mut c_void, name: &str) -> *mut c_void{
    unsafe{
        let ptr_name = CString::new(name).unwrap();
        let _sdl3_create_renderer: Symbol<SDL_CreateRenderer> = sdl3.lib.get(b"SDL_CreateRenderer")
            .expect("Failed to get symbol SDL_CreateRenderer");
        let renderer = _sdl3_create_renderer(window, ptr_name.as_ptr());
        if !renderer.is_null(){ renderer } else { panic!("SDL3 could not create renderer! SDL_Error: {}", sdl3_get_error(sdl3)); }
    }
}

pub fn sdl3_create_texture_from_surface(sdl3: &mut SDL3, render: *mut c_void, surface: *mut c_void) -> *mut c_void{
    unsafe{
        let _sdl3_create_texture_from_surface: Symbol<SDL_CreateTextureFromSurface> = sdl3.lib.get(b"SDL_CreateTextureFromSurface")
            .expect("Failed to get symbol SDL_CreateTextureFromSurface");
        let texture = _sdl3_create_texture_from_surface(render, surface);
        if !texture.is_null(){ texture } else { panic!("SDL3 could not create texture! SDL_Error: {}", sdl3_get_error(sdl3)); }
    }
}

pub fn sdl3_set_render_draw_color(sdl3: &mut SDL3, render: *mut c_void, r: u8, g: u8, b: u8, a: u8) -> bool{
    unsafe {
        let _sdl3_set_render_deaw_color: Symbol<SDL_SetRenderDrawColor> = sdl3.lib.get(b"SDL_SetRenderDrawColor")
            .expect("Failed to get symbol SDL_SetRenderDrawColor");
        _sdl3_set_render_deaw_color(render, r, g, b, a)
    }
}

pub fn sdl3_render_clear(sdl3: &mut SDL3, render: *mut c_void) -> bool{
    unsafe {
        let _sdl3_render_clear: Symbol<SDL_RenderClear> = sdl3.lib.get(b"SDL_RenderClear")
            .expect("Failed to get symbol SDL_RenderClear");
        _sdl3_render_clear(render)
    }
}

pub fn sdl3_render_present(sdl3: &mut SDL3, render: *mut c_void) -> bool{
    unsafe {
        let _sdl3_render_present: Symbol<SDL_RenderPresent> = sdl3.lib.get(b"SDL_RenderPresent")
            .expect("Failed to get symbol SDL_RenderPresent");
        _sdl3_render_present(render)
    }
}

pub fn sdl3_render_rect(sdl3: &mut SDL3, render: *mut c_void, rect: *const SDL_FRect) -> bool{
    unsafe {
        let _sdl3_render_rect: Symbol<SDL_RenderRect> = sdl3.lib.get(b"SDL_RenderRect")
            .expect("Failed to get symbol SDL_RenderRect");
        _sdl3_render_rect(render, rect)
    }
}

pub fn sdl3_render_fill_rect(sdl3: &mut SDL3, render: *mut c_void, rect: *const SDL_FRect) -> bool{
    unsafe{
        let _sdl3_render_fill_rect: Symbol<SDL_RenderFillRect> = sdl3.lib.get(b"SDL_RenderFillRect")
            .expect("Failed to get symbol SDL_RenderFillRect");
        _sdl3_render_fill_rect(render, rect)
    }
}

pub fn sdl3_render_rects(sdl3: &mut SDL3, render: *mut c_void, rect: *const SDL_FRect, count: i32) -> bool{
    unsafe{
        let _sdl3_render_rects: Symbol<SDL_RenderRects> = sdl3.lib.get(b"SDL_RenderRects")
            .expect("Failed to get symbol SDL_RenderRects");
        _sdl3_render_rects(render, rect, count)
    }
}

pub fn sdl3_render_fill_rects(sdl3: &mut SDL3, render: *mut c_void, rect: *const SDL_FRect, count: i32) -> bool{
    unsafe{
        let _sdl3_render_fill_rects: Symbol<SDL_RenderFillRects> = sdl3.lib.get(b"SDL_RenderFillRects")
            .expect("Failed to get symbol SDL_RenderFillRects");
        _sdl3_render_fill_rects(render, rect, count)
    }
}

pub fn sdl3_render_texture(
    sdl3: &mut SDL3,
    renderer: *mut c_void,
    texture: *mut c_void,
    srcrect: Option<*const SDL_FRect>,
    dstrect: Option<*const SDL_FRect>
) -> bool{
    unsafe{
        let _sdl3_render_texture: Symbol<SDL_RenderTexture> = sdl3.lib.get(b"SDL_RenderTexture")
            .expect("Failed to get symbol SDL_RenderTexture");
        let t_srcrect = match srcrect.is_some(){ true => { srcrect }, false => { Some(null()) } };
        let t_dstrect = match dstrect.is_some(){ true => { dstrect }, false => { Some(null()) } };
        _sdl3_render_texture(renderer, texture, t_srcrect.unwrap(), t_dstrect.unwrap())
    }
}

pub fn sdl3_destroy_renderer(sdl3: &mut SDL3, renderer: *mut c_void){
    unsafe {
        let _sdl3_destroy_renderer: Symbol<SDL_DestroyRenderer> = sdl3.lib.get(b"SDL_DestroyRenderer")
            .expect("Failed to get symbol SDL_DestroyRenderer");
        _sdl3_destroy_renderer(renderer);
    }
}
