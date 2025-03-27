use crate::artist::{
    Artist,
    artist_consts::*
};

use crate::collider::Collider;

use crate::sdl3::{
    SDL3,
    sdl3_render::*,
    sdl3_structs::*
};

use std::ffi::c_void;

impl Artist{
    pub fn draw(&self, sdl3: &mut SDL3, renderer: *mut c_void, playable: &Collider, objects: &Vec<Collider>){
        // Renders the playable character and the surrounding objects on the screen.
        //
        // This function draws the playable character and all objects in the provided vector
        // using the SDL3 rendering context. It sets the appropriate colors for each object
        // based on their properties and fills the corresponding rectangles on the screen.
        //
        // # Parameters
        //
        // - `sdl3`: A mutable reference to the SDL3 context used for rendering.
        // - `renderer`: A pointer to the SDL renderer used for drawing operations.
        // - `playable`: A reference to the `Collider` representing the playable character.
        // - `objects`: A reference to a vector of `Collider` objects representing the environment.
        //
        // # Functionality
        //
        // - The function first checks the `span` property of the `playable` character to determine
        //   if it should be drawn as a rectangle. If so, it creates a rectangle (`SDL_FRect`)
        //   based on the character's position and dimensions, sets the drawing color using the
        //   character's color properties, and fills the rectangle on the screen.
        //
        // - Next, the function iterates over the provided vector of `objects`. For each object,
        //   it checks the `span` property to see if it should be drawn as a rectangle. If it should,
        //   it creates a rectangle for the object, sets the drawing color based on the object's
        //   color properties, and fills the rectangle on the screen.
        //
        // # Notes
        //
        // - Ensure that the `color` properties of the `Collider` objects are set correctly to
        //   achieve the desired visual representation.
        // - The function assumes that the SDL3 context and renderer have been properly initialized
        //   before this function is called.
        // code -----------------------------------------------------------------------------------
        match playable.span{
            ARTIST_RECTANGLE => {
                let rect = SDL_FRect{x: playable.x, y: playable.y, w: playable.w, h: playable.h};
                sdl3_set_render_draw_color(sdl3, renderer, playable.color.0, playable.color.1, playable.color.2, playable.color.3);
                sdl3_render_fill_rect(sdl3, renderer, &rect as *const SDL_FRect);
            },
            _ => {}
        }
        for obj in objects{
            match obj.span{
                ARTIST_RECTANGLE => {
                    let rect = SDL_FRect{x: obj.x, y: obj.y, w: obj.w, h: obj.h};
                    sdl3_set_render_draw_color(sdl3, renderer, obj.color.0, obj.color.1, obj.color.2, obj.color.3);
                    sdl3_render_fill_rect(sdl3, renderer, &rect as *const SDL_FRect);
                },
                _ => {}
            }
        }
        // ----------------------------------------------------------------------------------------
    }
}
