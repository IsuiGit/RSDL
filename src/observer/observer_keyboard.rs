use crate::observer::{Observer, observer_consts::*};
use crate::sdl3::sdl3_consts::*;
use std::collections::HashMap;

impl Observer{
    pub fn default_keyboard(&mut self){
        let mut keyboard = HashMap::new();
        keyboard.insert(OBSERVER_MOVE_TOP_EVENT, SDLK_W);
        keyboard.insert(OBSERVER_MOVE_LEFT_EVENT, SDLK_A);
        keyboard.insert(OBSERVER_MOVE_BOTTOM_EVENT, SDLK_S);
        keyboard.insert(OBSERVER_MOVE_RIGHT_EVENT, SDLK_D);
        keyboard.insert(OBSERVER_EXIT_EVENT, SDLK_ESCAPE);
        keyboard.insert(OBSERVER_CHANGE_SCENE_EVENT, SDLK_Q);
        self.keyboard = keyboard;
    }
}
