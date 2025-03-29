use crate::observer::{
    Observer,
    observer_consts::*
};
use crate::sdl3::{
    SDL3,
    sdl3_consts::*
};
use std::collections::HashMap;

impl Observer{

    pub fn init_default_keyboard(&mut self){
        let mut keyboard = HashMap::new();
        keyboard.insert(OBSERVER_MOVE_TOP_EVENT, SDLK_W);
        keyboard.insert(OBSERVER_MOVE_LEFT_EVENT, SDLK_A);
        keyboard.insert(OBSERVER_MOVE_BOTTOM_EVENT, SDLK_S);
        keyboard.insert(OBSERVER_MOVE_RIGHT_EVENT, SDLK_D);
        keyboard.insert(OBSERVER_EXIT_EVENT, SDLK_ESCAPE);
        self.keyboard = keyboard;
    }

}
