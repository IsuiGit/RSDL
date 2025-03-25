// INCLUDE SDL3 MODS-------------------------------------------------------------------------------
pub mod sdl3_types;
pub mod sdl3_consts;
pub mod sdl3_structs;
pub mod sdl3_sys;
pub mod sdl3_window;
pub mod sdl3_render;

use sdl3_types::*;
// ------------------------------------------------------------------------------------------------
// INCLUDE STANDART MODS---------------------------------------------------------------------------
use libloading::Library;
use std::path::Path;
// ------------------------------------------------------------------------------------------------
// SDL3 MAIN STRUCT--------------------------------------------------------------------------------
pub struct SDL3{lib: Library}
// ------------------------------------------------------------------------------------------------
impl SDL3{
    pub fn new() -> Self {
        unsafe {
            let lib = Library::new(Path::new("src/sdl3/bin/SDL3.dll"))
                .expect("Failed to load SDL3.dll");
            SDL3 {lib}
        }
    }
}
