### RSDL

RSDL is a Rust library designed for 2D game development, simplifying graphics, sound, and input management. It offers a user-friendly interface for creating game objects, managing scenes, and handling events, allowing developers to focus on gameplay.

### Docs

Check the [Wiki](https://github.com/IsuiGit/RSDL/wiki)

### Structure and Example

Here’s a structured example of how to use the RSDL library for developing a 2D game, based on the provided code. This example includes comments to explain each section of the code, making it easier to understand how the different components work together.

```rust
// Importing necessary modules from the RSDL library and other components
use crate::sdl3::{SDL3, sdl3_consts::*, sdl3_structs::*, sdl3_sys::{sdl3_poll_event, sdl3_delay}, sdl3_window::sdl3_get_window_size};
use crate::collider::{Collider, collider_consts::*};
use crate::artist::{Artist, artist_consts::*};
use std::{collections::HashMap, mem::zeroed};
use crate::observer::Observer;
use crate::screenwriter::Scene;

/// Main function to test the SDL3 game system using RSDL.
fn main() {
    // Create an instance of the SDL3 library
    let mut sdl3 = SDL3::new();

    // Create a playable object (the character controlled by the player)
    let playable = Collider::new(
        COLLIDER_PLAYABLE,
        ARTIST_RECTANGLE,
        (255, 255, 255, 255), // White color
        [100.0, 100.0],       // Initial position
        [50.0, 50.0],         // Size of the playable object
        5.0                    // Velocity
    );

    // Create a structure to hold the scenes of the game
    let mut scenes: HashMap<u64, Scene> = HashMap::new();

    // Create game scenes
    let scene_0 = Scene::new(
        vec![
            Collider::new(COLLIDER_BLOCK, ARTIST_RECTANGLE, (127, 65, 250, 255), [0.0, 0.0], [300.0, 150.0], 0.0),
            Collider::new(COLLIDER_BLOCK, ARTIST_RECTANGLE, (172, 45, 112, 255), [400.0, 552.0], [200.0, 650.0], 0.0),
            Collider::new(COLLIDER_BLOCK, ARTIST_RECTANGLE, (117, 45, 112, 255), [1111.0, 243.0], [200.0, 950.0], 0.0),
            Collider::new(COLLIDER_BLOCK, ARTIST_RECTANGLE, (17, 145, 112, 255), [284.0, 650.0], [1300.0, 200.0], 0.0)
        ],
        1, // Next scene ID
        (0, 0, 0, 255), // Background color (black)
        String::from("Press Q to change scene"), // Text to display
        [1580.0, 40.0] // Position of the text
    );

    let scene_1 = Scene::new(
        vec![], // No objects in this scene
        0, // Next scene ID
        (0, 0, 0, 255), // Background color (black)
        String::from("Press Q to change scene"), // Text to display
        [1580.0, 40.0] // Position of the text
    );

    // Add scenes to the scenes structure
    scenes.insert(0, scene_0);
    scenes.insert(1, scene_1);

    // Initialize the observer with the playable object and scenes
    let mut observer = Observer::new(
        &mut sdl3,
        playable,
        scenes,
        [1920.0, 1080.0], // Window size
        SDL_INIT_VIDEO | SDL_INIT_AUDIO, // Initialization flags
        SDL_WINDOW_FULLSCREEN | SDL_WINDOW_OPENGL // Window flags
    );

    // Initialize the default keyboard mappings
    observer.default_keyboard();

    // Create an artist for rendering objects
    let artist = Artist::new(&mut sdl3, "src/sdl3/fonts/JetBrainsMono-VariableFont_wght.ttf", 20.0);

    // Main game loop
    let mut run = true;
    while run {
        unsafe {
            let mut event: SDL_Event = zeroed();
            // Poll for events
            while sdl3_poll_event(&mut sdl3)(&mut event as *mut SDL_Event) {
                let size = sdl3_get_window_size(&mut sdl3, observer.window);
                match event.type_ {
                    SDL_EVENT_QUIT => { run = false; break; }, // Exit the game
                    SDL_EVENT_WINDOW_RESIZED => { observer.resize([size.0 as f32, size.1 as f32]); }, // Resize the observer
                    SDL_EVENT_KEY_DOWN => { observer.events.insert(event.key.key); }, // Register key down event
                    SDL_EVENT_KEY_UP => { observer.events.remove(&event.key.key); }, // Register key up event
                    _ => {},
                }
            }

            // Process events and update the observer state
            observer.proceed_events(&mut sdl3);

            // Render the scene using the artist
            artist.drawing(&mut sdl3, observer.observer_to_artist_context());

            // Clean up event fields and delay for a smoother frame rate
            event.drop_fields();
            sdl3_delay(&mut sdl3, 16); // Delay to limit frame rate
        }
    }

    // Clean up resources before exiting
    artist.destroy(&mut sdl3);
    observer.destroy(&mut sdl3);
}
```
