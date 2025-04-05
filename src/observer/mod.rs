use crate::collider::Collider;
use crate::screenwriter::Scene;
use crate::artist::ArtistCache;
use std::{collections::{HashSet, HashMap}, ffi::c_void};
use crate::sdl3::{
    SDL3,
    sdl3_window::{
        sdl3_create_window,
        sdl3_destroy_window
    },
    sdl3_render::{
        sdl3_create_renderer,
        sdl3_destroy_renderer
    },
    sdl3_sys::{
        sdl3_init,
        sdl3_ttf_init,
        sdl3_quit,
        sdl3_ttf_quit
    }
};

mod observer_sys;
mod observer_events;
mod observer_keyboard;
pub mod observer_consts;

#[derive(Debug)]
pub struct Observer{
    pub playable: Collider,
    pub scenes: HashMap<u64, Scene>,
    pub current_scene: u64,
    pub size: [f32; 2],
    pub events: HashSet<u32>,
    pub keyboard: HashMap<u16, u32>,
    pub window: *mut c_void,
    pub renderer: *mut c_void,
    pub cache: ArtistCache,
    pub debug: bool
}

pub struct ObserverContext{
    pub renderer: *mut c_void,
    pub playable: Collider,
    pub objects: Vec<Collider>,
    pub background: (u8, u8, u8, u8),
    pub text: String,
    pub point: [f32; 2],
    pub cache: ArtistCache,
    pub debug: bool
}

impl Observer {
    pub fn new(sdl3: &mut SDL3, playable: Collider, scenes: HashMap<u64, Scene>, size: [f32; 2], iflags: u32, wflags: u64) -> Self {
        // Creates a new instance of the `Observer` struct and initializes the SDL3 context.
        //
        // This function sets up the SDL3 environment, creates a window and a renderer, and initializes
        // the necessary components for the application. It takes parameters for the playable character,
        // scenes, window size, and various flags for initialization.
        //
        // # Parameters
        //
        // - `sdl3`: A mutable reference to the SDL3 object that manages the graphical context and
        //   rendering.
        // - `playable`: A `Collider` object representing the playable character in the game.
        // - `scenes`: A `HashMap` containing the scenes of the game, indexed by unique identifiers.
        // - `size`: An array of two `f32` values representing the width and height of the window.
        // - `iflags`: A `u32` value representing initialization flags for SDL3.
        // - `wflags`: A `u64` value representing window creation flags.
        //
        // # Returns
        //
        // Returns a new instance of the `Observer` struct, which contains the initialized state of the
        // application.
        //
        // # Logic
        //
        // 1. Initializes the SDL3 context with the provided initialization flags using `sdl3_init`.
        // 2. Initializes the TTF subsystem with `sdl3_ttf_init`.
        // 3. Creates an empty `HashSet` for events and an empty `HashMap` for keyboard mappings.
        // 4. Creates a window using `sdl3_create_window`. If the window creation fails (i.e., the
        //    window pointer is null), it calls `sdl3_quit` and panics with an error message.
        // 5. Creates a renderer using `sdl3_create_renderer`. If the renderer creation fails (i.e., the
        //    renderer pointer is null), it destroys the window, calls `sdl3_quit`, and panics with an
        //    error message.
        // 6. Constructs and returns a new `Observer` instance with the initialized components.
        //
        // # Notes
        //
        // Ensure that the SDL3 and TTF libraries are properly linked and initialized before calling
        // this function. The provided window size should be valid, and the flags should be set according
        // to the desired behavior of the application.
        // code -----------------------------------------------------------------------------------
        sdl3_init(sdl3, iflags);
        sdl3_ttf_init(sdl3);
        let events: HashSet<u32> = HashSet::new();
        let keyboard: HashMap<u16, u32> = HashMap::new();
        let window = sdl3_create_window(sdl3, "Movement Test App", size[0] as u32, size[1] as u32, wflags);
        if window.is_null(){sdl3_quit(sdl3); panic!("window pointer is null!");}
        let renderer = sdl3_create_renderer(sdl3, window, "");
        if renderer.is_null(){sdl3_destroy_window(sdl3, window); sdl3_quit(sdl3); panic!("renderer pointer is null!");}
        let cache = ArtistCache::new(sdl3, renderer, &playable, scenes.get(&0).unwrap().clone());
        Self {
            playable: playable,
            scenes: scenes,
            current_scene: 0,
            size: size,
            events: events,
            keyboard: keyboard,
            window: window,
            renderer: renderer,
            cache: cache,
            debug: false
        }
        // ----------------------------------------------------------------------------------------
    }
    pub fn destroy(&self, sdl3: &mut SDL3){
        // Releases the resources associated with the `Observer` struct.
        //
        // This function cleans up the SDL3 context by destroying the renderer and window, and
        // quitting the TTF and SDL3 subsystems. It ensures that all resources are properly released
        // to prevent memory leaks.
        //
        // # Parameters
        //
        // - `sdl3`: A mutable reference to the SDL3 object that manages the graphical context and
        //   rendering.
        //
        // # Notes
        //
        // This function should be called when the `Observer` instance is no longer needed to ensure
        // that all associated resources are released properly. It is important to call this function
        // to avoid memory leaks and ensure that the SDL3 and TTF subsystems are properly shut down.
        // Ensure that the SDL3 context is valid before calling this function.
        // code -----------------------------------------------------------------------------------
        sdl3_destroy_renderer(sdl3, self.renderer);
        sdl3_destroy_window(sdl3, self.window);
        sdl3_ttf_quit(sdl3);
        sdl3_quit(sdl3);
        // ----------------------------------------------------------------------------------------
    }
    pub fn observer_to_artist_context(&self) -> ObserverContext {
        // Converts the current state of the `Observer` into an `ObserverContext`.
        //
        // This function constructs and returns an `ObserverContext` that encapsulates the current
        // rendering state, including the renderer, playable character, game objects, background,
        // text, and the position for rendering text. It retrieves the relevant data from the current
        // scene based on the `current_scene` index.
        //
        // # Returns
        //
        // Returns an `ObserverContext` struct containing the following fields:
        // - `renderer`: The renderer used for drawing.
        // - `playable`: The playable character in the game.
        // - `objects`: A vector of game objects from the current scene.
        // - `background`: The background color or texture for the current scene.
        // - `text`: The text to be rendered in the current scene.
        // - `point`: The coordinates where the text will be drawn.
        //
        // # Panics
        //
        // This function will panic if the `current_scene` does not exist in the `scenes` map,
        // as it calls `unwrap()` on the result of `get()`. Ensure that the `current_scene` is valid
        // before calling this function.
        //
        // # Notes
        //
        // This function is useful for passing the current rendering context to other components
        // responsible for drawing, such as the artist or rendering engine.
        // code -----------------------------------------------------------------------------------
        ObserverContext
        {
            renderer: self.renderer,
            playable: self.playable.clone(),
            objects: self.scenes.get(&self.current_scene).unwrap().objects.clone(),
            background: self.scenes.get(&self.current_scene).unwrap().background,
            text: self.scenes.get(&self.current_scene).unwrap().text.clone(),
            point: self.scenes.get(&self.current_scene).unwrap().point,
            cache: self.cache.clone(),
            debug: self.debug
        }
        // ----------------------------------------------------------------------------------------
    }
}
