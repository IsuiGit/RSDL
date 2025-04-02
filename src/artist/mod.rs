mod artist_draw;
pub mod artist_consts;

use crate::sdl3::SDL3;
use crate::sdl3::sdl3_ttf::{ttf_open_font, ttf_close_font};
use std::ffi::c_void;

pub struct Artist{
    pub font: *mut c_void
}

impl Artist{
    pub fn new(sdl3: &mut SDL3, file: &str, ptrsize: f32) -> Self{
        let font = ttf_open_font(sdl3, file, ptrsize);
        Artist{font: font}
    }
    pub fn destroy(&self, sdl3: &mut SDL3){
        ttf_close_font(sdl3, self.font);
    }
}
