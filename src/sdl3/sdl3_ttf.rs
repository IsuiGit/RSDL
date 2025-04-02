// INCLUDE SDL3 MODS-------------------------------------------------------------------------------
use crate::sdl3::{
    SDL3,
    TTF_OpenFont,
    TTF_CloseFont,
    TTF_RenderText_Blended,
    TTF_CreateRendererTextEngine,
    TTF_CreateText,
    TTF_DrawRendererText,
    sdl3_sys::sdl3_get_error,
    sdl3_structs::SDL_Color
};
use std::ffi::{c_void, CString};
use libloading::Symbol;

pub fn ttf_open_font(sdl3: &mut SDL3, file: &str, ptsize: f32) -> *mut c_void {
    unsafe{
        let ptr_file = match file.len() == 0 {
            true => {
                CString::new("src/sdl3/fonts/OpenSans-VariableFont_wdth,wght.ttf").unwrap()
            },
            false => {
                CString::new(file).unwrap()
            }
        };
        let _ttf_open_font: Symbol<TTF_OpenFont> = sdl3.ttf_lib.get(b"TTF_OpenFont")
            .expect("Failed to get symbol TTF_OpenFont");
        let font = _ttf_open_font(ptr_file.as_ptr(), ptsize);
        if !font.is_null(){ font } else { panic!("TTF could not initialize! SDL_Error: {}", sdl3_get_error(sdl3)); }
    }
}

pub fn ttf_close_font(sdl3: &mut SDL3, font: *mut c_void){
    unsafe{
        let _ttf_close_font: Symbol<TTF_CloseFont> = sdl3.ttf_lib.get(b"TTF_CloseFont")
            .expect("Failed to get symbol TTF_CloseFont");
        _ttf_close_font(font);
    }
}

pub fn ttf_render_text_blended(sdl3: &mut SDL3, font: *mut c_void, text: &str, fg: (u8, u8, u8, u8)) -> *mut c_void {
    unsafe{
        let ptr_text = match text.len() == 0 {
            true => {
                CString::new("Placeholder").unwrap()
            },
            false => {
                CString::new(text).unwrap()
            }
        };
        let len_text = text.len();
        let _ttf_render_text_blended: Symbol<TTF_RenderText_Blended> = sdl3.ttf_lib.get(b"TTF_RenderText_Blended")
            .expect("Failed to get symbol TTF_RenderText_Blended");
        let surface = _ttf_render_text_blended(font, ptr_text.as_ptr(), len_text, SDL_Color{r: fg.0, g: fg.1, b: fg.2, a: fg.3});
        if !surface.is_null(){ surface } else { panic!("TTF could not create surface! SDL_Error: {}", sdl3_get_error(sdl3)); }
    }
}

pub fn ttf_create_render_text_engine(sdl3: &mut SDL3, renderer: *mut c_void) -> *mut c_void{
    unsafe{
        let _ttf_create_render_text_engine: Symbol<TTF_CreateRendererTextEngine> = sdl3.ttf_lib.get(b"TTF_CreateRendererTextEngine")
            .expect("Failed to get symbol TTF_CreateRendererTextEngine");
        let engine = _ttf_create_render_text_engine(renderer);
        if !engine.is_null(){ engine } else { panic!("TTF could not create engine! SDL_Error: {}", sdl3_get_error(sdl3)); }
    }
}

pub fn ttf_create_text(sdl3: &mut SDL3, engine: *mut c_void, font: *mut c_void, text: &str) -> *mut c_void{
    unsafe{
        let ptr_text = match text.len() == 0 {
            true => {
                CString::new("Placeholder").unwrap()
            },
            false => {
                CString::new(text).unwrap()
            }
        };
        let len_text = text.len();
        let _ttf_create_text: Symbol<TTF_CreateText> = sdl3.ttf_lib.get(b"TTF_CreateText")
            .expect("Failed to get symbol TTF_CreateText");
        let text = _ttf_create_text(engine, font, ptr_text.as_ptr(), len_text);
        if !text.is_null(){ text } else { panic!("TTF could not create text! SDL_Error: {}", sdl3_get_error(sdl3)); }
    }
}

pub fn ttf_draw_render_text(sdl3: &mut SDL3, text: *mut c_void, x: f32, y: f32) -> bool{
    unsafe{
        let _ttf_draw_render_text: Symbol<TTF_DrawRendererText> = sdl3.ttf_lib.get(b"TTF_DrawRendererText")
            .expect("Failed to get symbol TTF_DrawRendererText");
        _ttf_draw_render_text(text, x, y)
    }
}
