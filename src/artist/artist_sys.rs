use crate::artist::Artist;
use crate::collider::Collider;
use crate::sdl3::{
    SDL3,
    sdl3_ttf::{
        ttf_create_render_text_engine,
        ttf_create_text,
        ttf_draw_render_text
    }
};
use std::ffi::c_void;
impl Artist{
    pub fn draw_debug_info(&self, sdl3: &mut SDL3, playable: &Collider, renderer: *mut c_void){
        let engine = ttf_create_render_text_engine(sdl3, renderer);
        let text = ttf_create_text(sdl3, engine, self.font, playable.debug().as_str());
        ttf_draw_render_text(sdl3, text, 10.0, 10.0);
    }
}
