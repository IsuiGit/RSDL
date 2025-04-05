use crate::observer::{Observer, observer_consts::*};
use crate::sdl3::sdl3_consts::*;
use std::collections::HashMap;

impl Observer{
    pub fn default_keyboard(&mut self){
        // Initializes the default keyboard mappings for the `Observer`.
        //
        // This function sets up a `HashMap` that maps specific keyboard events to their corresponding
        // SDL key codes. The default mappings are as follows:
        // - `OBSERVER_MOVE_TOP_EVENT` is mapped to the 'W' key.
        // - `OBSERVER_MOVE_LEFT_EVENT` is mapped to the 'A' key.
        // - `OBSERVER_MOVE_BOTTOM_EVENT` is mapped to the 'S' key.
        // - `OBSERVER_MOVE_RIGHT_EVENT` is mapped to the 'D' key.
        // - `OBSERVER_EXIT_EVENT` is mapped to the 'Escape' key.
        // - `OBSERVER_CHANGE_SCENE_EVENT` is mapped to the 'Q' key.
        //
        // # Notes
        //
        // This function should be called to set up the keyboard mappings before processing any input
        // events. It is important to ensure that the keyboard is properly initialized to allow for
        // user interaction with the application.
        // code -----------------------------------------------------------------------------------
        let mut keyboard = HashMap::new();
        keyboard.insert(OBSERVER_MOVE_TOP_EVENT, SDLK_W);
        keyboard.insert(OBSERVER_MOVE_LEFT_EVENT, SDLK_A);
        keyboard.insert(OBSERVER_MOVE_BOTTOM_EVENT, SDLK_S);
        keyboard.insert(OBSERVER_MOVE_RIGHT_EVENT, SDLK_D);
        keyboard.insert(OBSERVER_EXIT_EVENT, SDLK_ESCAPE);
        keyboard.insert(OBSERVER_CHANGE_SCENE_EVENT, SDLK_Q);
        keyboard.insert(OBSERVER_DEBUG_ENABLE_EVENT, SDLK_F1);
        keyboard.insert(OBSERVER_DEBUG_DISABLE_EVENT, SDLK_F2);
        self.keyboard = keyboard;
        // ----------------------------------------------------------------------------------------
    }
    pub fn platformer_keyboard(&mut self){
        // Initializes the platforemer keyboard mappings for the `Observer`.
        //
        // This function sets up a `HashMap` that maps specific keyboard events to their corresponding
        // SDL key codes. The default mappings are as follows:
        // - `OBSERVER_MOVE_LEFT_EVENT` is mapped to the 'A' key.
        // - `OBSERVER_MOVE_RIGHT_EVENT` is mapped to the 'D' key.
        // - `OBSERVER_EXIT_EVENT` is mapped to the 'Escape' key.
        // - `OBSERVER_JUMP_EVENT` is mapped to the 'Space' key.
        // - `OBSERVER_CHANGE_SCENE_EVENT` is mapped to the 'Q' key.
        //
        // # Notes
        //
        // This function should be called to set up the keyboard mappings before processing any input
        // events. It is important to ensure that the keyboard is properly initialized to allow for
        // user interaction with the application.
        // code -----------------------------------------------------------------------------------
        let mut keyboard = HashMap::new();
        keyboard.insert(OBSERVER_MOVE_LEFT_EVENT, SDLK_A);
        keyboard.insert(OBSERVER_MOVE_RIGHT_EVENT, SDLK_D);
        keyboard.insert(OBSERVER_JUMP_EVENT, SDLK_SPACE);
        keyboard.insert(OBSERVER_EXIT_EVENT, SDLK_ESCAPE);
        keyboard.insert(OBSERVER_CHANGE_SCENE_EVENT, SDLK_Q);
        keyboard.insert(OBSERVER_DEBUG_ENABLE_EVENT, SDLK_F1);
        keyboard.insert(OBSERVER_DEBUG_DISABLE_EVENT, SDLK_F2);
        self.keyboard = keyboard;
        // ----------------------------------------------------------------------------------------
    }
}
