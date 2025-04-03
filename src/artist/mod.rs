mod artist_draw;
pub mod artist_consts;

use crate::sdl3::SDL3;
use crate::sdl3::sdl3_ttf::{ttf_open_font, ttf_close_font};
use std::ffi::c_void;

pub struct Artist{
    pub font: *mut c_void
}

impl Artist{
    pub fn new(sdl3: &mut SDL3, file: &str, ptsize: f32) -> Self{
        // Creates a new instance of the `Artist` struct by loading a font.
        //
        // This function initializes a new `Artist` object by opening a font file using the SDL3
        // and TTF libraries. It takes a mutable reference to the SDL3 context, the path to the font
        // file, and the font size as parameters.
        //
        // # Parameters
        //
        // - `sdl3`: A mutable reference to the SDL3 object that manages the graphical context and
        //   rendering.
        // - `file`: A string slice representing the path to the font file to be loaded.
        // - `ptsize`: A `f32` value representing the size of the font to be loaded.
        //
        // # Returns
        //
        // Returns a new instance of the `Artist` struct, which contains the loaded font.
        //
        // # Notes
        //
        // Ensure that the provided font file path is valid and that the SDL3 and TTF libraries are
        // properly initialized before calling this function. If the font loading fails, appropriate
        // error handling should be implemented to manage the failure.
        // code -----------------------------------------------------------------------------------
        let font = ttf_open_font(sdl3, file, ptsize);
        Artist{font: font}
        // ----------------------------------------------------------------------------------------
    }
    pub fn destroy(&self, sdl3: &mut SDL3){
        // Releases the resources associated with the font in the `Artist` struct.
        //
        // This function closes the font that was previously opened and associated with the `Artist`
        // instance. It ensures that the resources are properly cleaned up to prevent memory leaks.
        //
        // # Parameters
        //
        // - `sdl3`: A mutable reference to the SDL3 object that manages the graphical context and
        //   rendering.
        //
        // # Notes
        //
        // This function should be called when the `Artist` instance is no longer needed to ensure
        // that the font resources are released properly. Ensure that the SDL3 and TTF libraries are
        // properly initialized before calling this function. If the font has already been closed or
        // is invalid, appropriate error handling should be implemented to manage such cases.
        // code -----------------------------------------------------------------------------------
        ttf_close_font(sdl3, self.font);
        // ----------------------------------------------------------------------------------------
    }
}
