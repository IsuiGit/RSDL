use crate::collider::{Collider, Direction, State};

impl Collider{
    pub fn direction_move(&mut self, max: [f32; 2], direction: Direction) {
        // Moves the object in the specified direction while checking for global collisions.
        //
        // This function updates the position of the object based on the provided direction
        // (Left, Top, Right, or Bottom). Before moving, it checks for potential collisions
        // with the boundaries defined by the `max` parameter. If a collision is detected in
        // the specified direction, the object's position is not updated.
        //
        // # Parameters
        //
        // - `max`: An array of two `f32` values representing the maximum boundaries of the
        //   area in which the object can move.
        //   - `max[0]`: The maximum width of the area (e.g., screen width).
        //   - `max[1]`: The maximum height of the area (e.g., screen height).
        //
        // - `direction`: An instance of the `Direction` enum that specifies the direction
        //   in which the object should move.
        //
        // # Functionality
        //
        // - The function uses a match statement to determine the direction of movement.
        // - For each direction, it checks if moving in that direction would result in a
        //   collision with the boundaries defined by `max` using the `global_collide` method.
        // - If no collision is detected, the object's position is updated accordingly:
        //  - Moving left decreases the x-coordinate by the object's left velocity (`velocity`).
        //   - Moving up decreases the y-coordinate by the object's top velocity (`velocity`).
        //   - Moving right increases the x-coordinate by the object's right velocity (`velocity`).
        //   - Moving down increases the y-coordinate by the object's bottom velocity (`velocity`).
        //
        // # Notes
        //
        // - Ensure that the `global_collide` method is implemented correctly to accurately
        //   detect collisions with the boundaries.
        // - The velocity should be set appropriately to control
        //   the speed of movement in each direction.
        // code -----------------------------------------------------------------------------------
        match direction {
            Direction::Left => {
                if !self.global_collide(max).0 {
                    self.state = State::Moving;
                    self.pos[0] -= self.velocity;
                }
            }
            Direction::Top => {
                if !self.global_collide(max).1 {
                    self.state = State::Moving;
                    self.pos[1] -= self.velocity;
                }
            }
            Direction::Right => {
                if !self.global_collide(max).2 {
                    self.state = State::Moving;
                    self.pos[0] += self.velocity;
                }
            }
            Direction::Bottom => {
                if !self.global_collide(max).3 {
                    self.state = State::Moving;
                    self.pos[1] += self.velocity;
                }
            }
        }
        // ----------------------------------------------------------------------------------------
    }
}
