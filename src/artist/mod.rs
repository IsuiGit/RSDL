mod artist_draw;
pub mod artist_consts;

use crate::screenwriter::Scene;
use crate::collider::Collider;
use crate::sdl3::{SDL3, sdl3_render::*, sdl3_image::*};
use crate::sdl3::sdl3_ttf::{ttf_open_font, ttf_close_font};
use std::{ffi::c_void, collections::HashMap};

pub struct Artist{
    pub font: *mut c_void
}

#[derive(Clone)]
pub struct ArtistCache{
    pub cache: HashMap<String, *mut c_void>
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

impl ArtistCache{
    pub fn new(sdl3: &mut SDL3, renderer: *mut c_void, playable: &Collider, scene: Scene) -> Self{
        // Creates a new instance of `ArtistCache`, which is responsible for caching textures
        // for the playable character and the objects in the scene.
        //
        // # Parameters
        //
        // - `sdl3`: A mutable reference to the SDL3 context, which manages the graphical operations.
        // - `renderer`: A pointer to the SDL renderer used for drawing textures on the screen.
        // - `playable`: A reference to the `Collider` object representing the playable character,
        //   which contains information about its associated image.
        // - `scene`: An instance of `Scene`, which contains a collection of objects to be rendered.
        //
        // # Logic
        //
        // 1. Initializes a new `HashMap` to store cached textures, where the keys are image paths
        //    and the values are pointers to the corresponding SDL textures.
        // 2. Checks if the texture for the playable character's image is already cached. If not,
        //    it loads the image using `img_load`, creates a texture from the loaded surface using
        //    `sdl3_create_texture_from_surface`, and inserts the texture into the cache.
        // 3. Iterates over the objects in the provided scene. For each object, it checks if the
        //    texture for the object's image is already cached. If not, it performs the same loading
        //    and caching process as for the playable character.
        // 4. Returns a new instance of `ArtistCache` containing the populated texture cache.
        //
        // # Notes
        //
        // This implementation helps to optimize rendering performance by avoiding redundant
        // image loading and texture creation, ensuring that each unique image is only processed
        // once and reused throughout the rendering process.
        let mut cache: HashMap<String, *mut c_void> = HashMap::new();
        if !cache.contains_key(&playable.image){
            let playable_img = img_load(sdl3, playable.image.as_str());
            let texture = sdl3_create_texture_from_surface(sdl3, renderer, playable_img);
            cache.insert(playable.image.clone(), texture);
        }
        for obj in &scene.objects{
            if !cache.contains_key(&obj.image){
                let obj_img = img_load(sdl3, obj.image.as_str());
                let texture = sdl3_create_texture_from_surface(sdl3, renderer, obj_img);
                cache.insert(obj.image.clone(), texture);
            }
        }
        ArtistCache{ cache }
    }
}
