use crate::observer::{Observer, observer_consts::*};
use crate::collider::{Collider, Direction, State, collider_consts::*};
use crate::artist::ArtistCache;
use crate::sdl3::{SDL3, sdl3_consts::*, sdl3_structs::SDL_Event, sdl3_sys::sdl3_push_event};
use std::{mem::zeroed, thread, time::Duration};

impl Observer {
    pub fn proceed_events(&mut self, sdl3: &mut SDL3){
        // Processes input events and updates the state of the `Observer`.
        //
        // This function checks for keyboard events and updates the position of the playable character
        // based on user input. It handles scene changes, exits, and movement in four directions (left,
        // top, right, bottom). The function also checks for collisions with other objects in the scene.
        //
        // # Parameters
        //
        // - `sdl3`: A mutable reference to the SDL3 object that manages the graphical context and
        //   rendering.
        //
        // # Logic
        //
        // 1. Retrieves the current scene's objects that are of type `COLLIDER_BLOCK` and stores them in
        //    a vector for collision detection.
        // 2. Checks if the keyboard has been initialized. If not, it prints a warning and returns early.
        // 3. Iterates over the objects in the current scene and checks for overlaps with the playable
        //    character.
        // 4. Checks for specific keyboard events:
        //    - If the `OBSERVER_CHANGE_SCENE_EVENT` is triggered, it changes to the next scene if it
        //      exists, pausing for one second afterward.
        //    - If the `OBSERVER_EXIT_EVENT` is triggered, it pushes a quit event to the SDL3 event queue.
        //    - For movement events (`OBSERVER_MOVE_LEFT_EVENT`, `OBSERVER_MOVE_TOP_EVENT`,
        //      `OBSERVER_MOVE_RIGHT_EVENT`, `OBSERVER_MOVE_BOTTOM_EVENT`), it checks if the playable
        //      character can move in the specified direction based on collision detection with other
        //      objects. If movement is allowed, it updates the playable character's position.
        //
        // # Notes
        //
        // Ensure that the keyboard mapping is properly initialized before calling this function.
        // The function assumes that the `playable` character and objects have valid properties for
        // collision detection and movement. If any of the scene or object data is invalid, the function
        // may panic due to the use of `unwrap()`.
        // code -----------------------------------------------------------------------------------
        if self.keyboard.is_empty(){
            println!("Keyboard not initialized!");
            return;
        }

        // Update all objects
        self.update();

        // Get updatet non-playable objects
        let objects: Vec<&Collider> = self.scenes.get(&self.current_scene).unwrap()
            .objects.iter().filter(|obj| obj.type_ == COLLIDER_BLOCK).collect();

        // Counting playable overlaps
        for obj in &self.scenes.get(&self.current_scene).unwrap().objects{
            self.playable.overlap(self.size, &obj);
        }

        // Enable playable debug text
        if let Some(&event) = self.keyboard.get(&OBSERVER_DEBUG_ENABLE_EVENT){
            if self.events.contains(&event){
                self.debug = true;
            }
        }

        // Disable playable debug text
        if let Some(&event) = self.keyboard.get(&OBSERVER_DEBUG_DISABLE_EVENT){
            if self.events.contains(&event){
                self.debug = false;
            }
        }

        // Change scene event
        if let Some(&event) = self.keyboard.get(&OBSERVER_CHANGE_SCENE_EVENT){
            if self.events.contains(&event){
                let next = self.scenes.get(&self.current_scene).unwrap().next_scene;
                if self.scenes.contains_key(&next){
                    self.current_scene = next;
                    self.cache = ArtistCache::new(sdl3, self.renderer, &self.playable, self.scenes.get(&next).unwrap().clone());
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }

        // Exit event
        if let Some(&event) = self.keyboard.get(&OBSERVER_EXIT_EVENT){
            if self.events.contains(&event){
                unsafe {
                    let mut event: SDL_Event = zeroed();
                    event.type_ = SDL_EVENT_QUIT;
                    sdl3_push_event(sdl3, &mut event as *mut SDL_Event);
                }
            }
        }

        // Move left event
        if let Some(&event) = self.keyboard.get(&OBSERVER_MOVE_LEFT_EVENT){
            if self.events.contains(&event){
                let mut can_move = true;
                let direction = Direction::Left;
                for obj in &objects{
                    if self.playable.ray_cast(obj, direction.clone()) < self.playable.velocity && self.playable.distance_to(obj) == 0.0 {
                        can_move = false;
                    }
                }
                if can_move{
                    self.playable.direction_move(self.size, direction.clone());
                }
            }
        }

        // Move top event
        if let Some(&event) = self.keyboard.get(&OBSERVER_MOVE_TOP_EVENT){
            if self.events.contains(&event){
                let mut can_move = true;
                let direction = Direction::Top;
                for obj in &objects{
                    if self.playable.ray_cast(obj, direction.clone()) < self.playable.velocity && self.playable.distance_to(obj) == 0.0 {
                        can_move = false;
                    }
                }
                if can_move{
                    self.playable.direction_move(self.size, direction.clone());
                }
            }
        }

        // Move right event
        if let Some(&event) = self.keyboard.get(&OBSERVER_MOVE_RIGHT_EVENT){
            if self.events.contains(&event){
                let mut can_move = true;
                let direction = Direction::Right;
                for obj in &objects{
                    if self.playable.ray_cast(obj, direction.clone()) < self.playable.velocity && self.playable.distance_to(obj) == 0.0 {
                        can_move = false;
                    }
                }
                if can_move{
                    self.playable.direction_move(self.size, direction.clone());
                }
            }
        }

        // Move bottom event
        if let Some(&event) = self.keyboard.get(&OBSERVER_MOVE_BOTTOM_EVENT){
            if self.events.contains(&event){
                let mut can_move = true;
                let direction = Direction::Bottom;
                for obj in &objects{
                    if self.playable.ray_cast(obj, direction.clone()) < self.playable.velocity && self.playable.distance_to(obj) == 0.0 {
                        can_move = false;
                    }
                }
                if can_move{
                    self.playable.direction_move(self.size, direction.clone());
                }
            }
        }
        // ----------------------------------------------------------------------------------------
    }
}
