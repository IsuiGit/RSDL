use crate::artist::{Artist, ArtistCache, artist_consts::*};
use crate::observer::ObserverContext;
use crate::sdl3::{SDL3, sdl3_render::*, sdl3_structs::*, sdl3_ttf::*};
use crate::collider::Collider;
use std::ffi::c_void;

impl Artist{
    pub fn drawing(&self, sdl3: &mut SDL3, context: ObserverContext) {
        // Renders objects and text on the screen.
        //
        // This function performs the rendering of all game objects and text on the screen using
        // the SDL3 library. It takes a mutable reference to the SDL3 context and an `ObserverContext`
        // that contains information about the current game state, including the objects to be rendered
        // and the text to be displayed on the screen.
        //
        // # Parameters
        //
        // - `sdl3`: A mutable reference to the SDL3 object that manages the graphical context and
        //   rendering.
        // - `context`: An `ObserverContext` structure that contains information about the current
        //   game state, including:
        //   - `renderer`: The renderer used for drawing objects and text.
        //   - `playable`: A reference to the object that the player can control.
        //   - `objects`: A list of objects that should be rendered on the screen.
        //   - `background`: The background that will be displayed behind the objects and text.
        //   - `text`: A string of text that needs to be rendered on the screen.
        //   - `point`: The coordinates where the text will be drawn on the screen.
        //
        // # Logic
        //
        // 1. Calls the `draw_objects` method, which is responsible for rendering all game objects
        //    on the screen. It takes the renderer, the controllable object, the list of objects,
        //    and the background as parameters.
        // 2. Checks if there is text to render. If the length of `context.text` is not zero,
        //    it calls the `draw_text` method to render the text on the screen at the specified point.
        // 3. Calls the `sdl3_render_present` function to update the screen and display all changes
        //    made in the current frame.
        //
        // # Notes
        //
        // Ensure that the passed parameters are valid and that the SDL3 context and renderer are
        // initialized before calling this function. Incorrect parameters may lead to rendering errors.
        // code -----------------------------------------------------------------------------------
        self.draw_objects(sdl3, context.renderer, &context.playable, &context.objects, context.background, context.cache);
        if context.text.len() != 0 {
            self.draw_text(sdl3, context.renderer, context.text, context.point);
        }
        if context.debug{
            self.draw_debug_info(sdl3, &context.playable, context.renderer);
        }
        sdl3_render_present(sdl3, context.renderer);
        // ----------------------------------------------------------------------------------------
    }
    fn draw_objects(
        &self,
        sdl3: &mut SDL3,
        renderer: *mut c_void,
        playable: &Collider,
        objects: &Vec<Collider>,
        background: (u8, u8, u8, u8),
        cache: ArtistCache
    ) {
        // Renders game objects and the playable character on the screen.
        //
        // This function is responsible for setting the rendering color, clearing the screen,
        // and drawing the playable character and other game objects based on their properties.
        // It uses the SDL3 library to perform the rendering operations.
        //
        // # Parameters
        //
        // - `sdl3`: A mutable reference to the SDL3 object that manages the graphical context and
        //   rendering.
        // - `renderer`: A mutable pointer to the renderer used for drawing objects on the screen.
        // - `playable`: A reference to the `Collider` object representing the playable character.
        // - `objects`: A reference to a vector of `Collider` objects representing other game entities.
        // - `background`: A tuple containing the RGBA values (u8) for the background color.
        //
        // # Logic
        //
        // 1. Sets the rendering color to the specified background color using `sdl3_set_render_draw_color`.
        // 2. Clears the rendering target with `sdl3_render_clear`.
        // 3. Checks the `span` property of the `playable` character:
        //    - If it matches `ARTIST_RECTANGLE`, it creates an `SDL_FRect` for the playable character
        //      using its position and size, sets the drawing color to the character's color, and
        //      fills the rectangle on the screen using `sdl3_render_fill_rect`.
        // 4. Iterates over the `objects` vector:
        //    - For each object, it checks the `span` property:
        //      - If it matches `ARTIST_RECTANGLE`, it creates an `SDL_FRect` for the object using its
        //        position and size, sets the drawing color to the object's color, and fills the rectangle
        //        on the screen using `sdl3_render_fill_rect`.
        //
        // # Notes
        //
        // Ensure that the `renderer` and `sdl3` context are properly initialized before calling this
        // function. The function assumes that the `playable` and `objects` have valid properties
        // for rendering. If the `span` does not match `ARTIST_RECTANGLE`, the object will not be drawn.
        // code -----------------------------------------------------------------------------------
        sdl3_set_render_draw_color(sdl3, renderer, background.0, background.1, background.2, background.3);
        sdl3_render_clear(sdl3, renderer);
        match playable.span{
            ARTIST_RECTANGLE => {
                let rect = SDL_FRect{x: playable.pos[0], y: playable.pos[1], w: playable.size[0], h: playable.size[1]};
                sdl3_set_render_draw_color(sdl3, renderer, playable.color.0, playable.color.1, playable.color.2, playable.color.3);
                sdl3_render_fill_rect(sdl3, renderer, &rect as *const SDL_FRect);
            },
            ARTIST_IMAGE => {
                let playable_texture = cache.cache.get(&playable.image).unwrap();
                sdl3_render_texture(
                    sdl3,
                    renderer,
                    playable_texture.clone(),
                    None,
                    Some(&SDL_FRect{x: playable.pos[0], y: playable.pos[1], w: playable.size[0], h: playable.size[1]} as *const SDL_FRect)
                );
            },
            _ => {}
        }
        for obj in objects{
            match obj.span{
                ARTIST_RECTANGLE => {
                    let rect = SDL_FRect{x: obj.pos[0], y: obj.pos[1], w: obj.size[0], h: obj.size[1]};
                    sdl3_set_render_draw_color(sdl3, renderer, obj.color.0, obj.color.1, obj.color.2, obj.color.3);
                    sdl3_render_fill_rect(sdl3, renderer, &rect as *const SDL_FRect);
                },
                ARTIST_IMAGE => {
                    let obj_texture = cache.cache.get(&obj.image).unwrap();
                    sdl3_render_texture(
                        sdl3,
                        renderer,
                        obj_texture.clone(),
                        None,
                        Some(&SDL_FRect{x: obj.pos[0], y: obj.pos[1], w: obj.size[0], h: obj.size[1]} as *const SDL_FRect)
                    );
                },
                _ => {}
            }
        }
        // ----------------------------------------------------------------------------------------
    }
    fn draw_text(&self, sdl3: &mut SDL3, renderer: *mut c_void, text: String, ptext: [f32; 2]) {
        // Renders text on the screen using the SDL3 and TTF libraries.
        //
        // This function creates a text rendering engine, generates a texture from the provided text,
        // and draws it at the specified position on the screen.
        //
        // # Parameters
        //
        // - `sdl3`: A mutable reference to the SDL3 object that manages the graphical context and
        //   rendering.
        // - `renderer`: A mutable pointer to the renderer used for drawing the text on the screen.
        // - `text`: A `String` containing the text to be rendered.
        // - `ptext`: An array of two `f32` values representing the x and y coordinates where the text
        //   will be drawn on the screen.
        //
        // # Logic
        //
        // 1. Calls `ttf_create_render_text_engine` to create a text rendering engine using the SDL3
        //    context and the renderer.
        // 2. Uses `ttf_create_text` to generate a texture from the provided `text` string, using the
        //    specified font.
        // 3. Calls `ttf_draw_render_text` to draw the generated text texture at the specified position
        //    (`ptext[0]` for the x-coordinate and `ptext[1]` for the y-coordinate) on the screen.
        //
        // # Notes
        //
        // Ensure that the SDL3 and TTF libraries are properly initialized before calling this function.
        // The font used for rendering text should be loaded and set in the `self.font` field. If the
        // text rendering fails, appropriate error handling should be implemented to manage the failure.
        // code -----------------------------------------------------------------------------------
        let text = ttf_create_text(sdl3, self.engine, self.font, text.as_str());
        ttf_draw_render_text(sdl3, text, ptext[0], ptext[1]);
        // ----------------------------------------------------------------------------------------
    }
}
