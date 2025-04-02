use crate::observer::{Observer, observer_consts::*};
use crate::collider::{Collider, Direction, collider_consts::*};
use crate::sdl3::{SDL3, sdl3_consts::*, sdl3_structs::SDL_Event, sdl3_sys::sdl3_push_event};
use std::{mem::zeroed, thread, time::Duration};

impl Observer {
    pub fn proceed_events(&mut self, sdl3: &mut SDL3){
        // Processes input events and updates the position of the playable character accordingly.
        //
        // This function checks for specific key events (A, W, D, S) to determine the direction
        // in which the playable character should move. It also checks for potential collisions
        // with other colliders of type `COLLIDER_BLOCK` before allowing movement in the specified
        // direction. If a collision is detected, the movement in that direction is prevented.
        //
        // # Functionality
        //
        // - The function first filters the list of objects to include only those of type `COLLIDER_BLOCK`.
        // - It then checks for key presses corresponding to movement directions:
        //   - **A** (Left): If the left movement key is pressed, it checks for collisions to the left.
        //   - **W** (Up): If the up movement key is pressed, it checks for collisions above.
        //   - **D** (Right): If the right movement key is pressed, it checks for collisions to the right.
        //   - **S** (Down): If the down movement key is pressed, it checks for collisions below.
        // - For each direction, if no collision is detected (i.e., the distance to the collider is greater
        //   than the velocity in that direction), the playable character's position is updated using
        //   the `direction_move` method.
        //
        // # Notes
        //
        // - Ensure that the `playable` character and the colliders are properly initialized before
        //   calling this function.
        // - The function assumes that the `ray_cast` and `distance_to` methods are implemented
        //   correctly to handle collision detection.
        // code -----------------------------------------------------------------------------------
        let objects: Vec<&Collider> = self.scenes.get(&self.current_scene).unwrap()
            .objects.iter().filter(|obj| obj.type_ == COLLIDER_BLOCK).collect();

        if self.keyboard.is_empty(){
            println!("Keyboard not initialized!");
            return;
        }

        for obj in &self.scenes.get(&self.current_scene).unwrap().objects{
            self.playable.overlap(self.size, &obj);
        }

        if self.events.contains(&self.keyboard.get(&OBSERVER_CHANGE_SCENE_EVENT).unwrap_or(&0)) {
            let next = self.scenes.get(&self.current_scene).unwrap().next_scene;
            if self.scenes.contains_key(&next){
                self.current_scene = next;
                thread::sleep(Duration::from_secs(1));
            }
        }

        if self.events.contains(&self.keyboard.get(&OBSERVER_EXIT_EVENT).unwrap_or(&0)){
            unsafe {
                let mut event: SDL_Event = zeroed();
                event.type_ = SDL_EVENT_QUIT;
                sdl3_push_event(sdl3, &mut event as *mut SDL_Event);
            }
        }

        if self.events.contains(&self.keyboard.get(&OBSERVER_MOVE_LEFT_EVENT).unwrap_or(&0)){
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

        if self.events.contains(&self.keyboard.get(&OBSERVER_MOVE_TOP_EVENT).unwrap_or(&0)){
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

        if self.events.contains(&self.keyboard.get(&OBSERVER_MOVE_RIGHT_EVENT).unwrap_or(&0)){
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

        if self.events.contains(&self.keyboard.get(&OBSERVER_MOVE_BOTTOM_EVENT).unwrap_or(&0)){
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
    // --------------------------------------------------------------------------------------------
    }
}
