use crate::artist::Artist;
use crate::collider::Collider;
use crate::sdl3::{
    SDL3,
    sdl3_ttf::{
        ttf_create_text,
        ttf_draw_render_text,
        ttf_destroy_text
    }
};
use std::ffi::c_void;
impl Artist{
    pub fn draw_debug_info(&self, sdl3: &mut SDL3, playable: &Collider, renderer: *mut c_void, fps: u16){
        let text = ttf_create_text(sdl3, self.engine, self.font, format!("{}\nFPS: {}", playable.debug(), fps).as_str());
        ttf_draw_render_text(sdl3, text, 10.0, 10.0);
        ttf_destroy_text(sdl3, text);
    }
}
