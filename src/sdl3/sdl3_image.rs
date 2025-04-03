use crate::sdl3::{
    SDL3,
    IMG_Load,
    IMG_LoadTexture,
    sdl3_sys::sdl3_get_error
};
use std::ffi::{c_void, CString};
use libloading::Symbol;

pub fn img_load_texture(sdl3: &mut SDL3, renderer: *mut c_void, file: &str) -> *mut c_void{
    unsafe{
        let ptr_file = match file.len() == 0 {
            true => {
                CString::new("src/sdl3/static/placeholder.png").unwrap()
            },
            false => {
                CString::new(file).unwrap()
            }
        };
        let _img_load_texture: Symbol<IMG_LoadTexture> = sdl3.img_lib.get(b"IMG_LoadTexture")
            .expect("Failed to get symbol IMG_LoadTexture");
        let texture = _img_load_texture(renderer, ptr_file.as_ptr());
        if !texture.is_null(){ texture } else { panic!("IMG could not load texture! SDL_Error: {}", sdl3_get_error(sdl3)); }
    }
}

pub fn img_load(sdl3: &mut SDL3, file: &str) -> *mut c_void{
    unsafe{
        let ptr_file = match file.len() == 0 {
            true => {
                CString::new("src/sdl3/static/placeholder.png").unwrap()
            },
            false => {
                CString::new(file).unwrap()
            }
        };
        let _img_load: Symbol<IMG_Load> = sdl3.img_lib.get(b"IMG_Load")
            .expect("Failed to get symbol IMG_Load");
        let surface = _img_load(ptr_file.as_ptr());
        if !surface.is_null(){ surface } else { panic!("IMG could not create surface! SDL_Error: {}", sdl3_get_error(sdl3)); }
    }
}
