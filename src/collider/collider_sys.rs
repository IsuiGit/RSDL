use crate::collider::{Collider, State};
use std::time::{SystemTime, UNIX_EPOCH};

impl Collider {
    pub fn global_collide(&self, max: [f32; 2]) -> (bool, bool, bool, bool) {
        // Checks for collisions with the global boundaries defined by the maximum dimensions.
        //
        // This function determines whether the current collider is colliding with the global boundaries
        // of the defined area (e.g., the edges of the screen or a game world). It checks if the collider
        // is outside the left, top, right, or bottom boundaries and returns a tuple of boolean values
        // indicating the collision status for each boundary.
        //
        // # Parameters
        //
        // - `max`: An array of two `f32` values representing the maximum dimensions of the area.
        //   - `max[0]`: The maximum width of the area (e.g., screen width).
        //   - `max[1]`: The maximum height of the area (e.g., screen height).
        //
        // # Returns
        //
        // Returns a tuple of four boolean values:
        // - The first value indicates if the collider is colliding with the left boundary (true if colliding).
        // - The second value indicates if the collider is colliding with the top boundary (true if colliding).
        // - The third value indicates if the collider is colliding with the right boundary (true if colliding).
        // - The fourth value indicates if the collider is colliding with the bottom boundary (true if colliding).
        //
        // # Functionality
        //
        // - The function checks the following conditions:
        //   - **Left Boundary**: Returns true if the left edge of the collider (`self.x`) is less than or
        //     equal to `0.0`.
        //   - **Top Boundary**: Returns true if the top edge of the collider (`self.y`) is less than or
        //     equal to `0.0`.
        //   - **Right Boundary**: Returns true if the right edge of the collider (`self.x + self.w`) is
        //     greater than or equal to the maximum width (`max[0]`).
        //   - **Bottom Boundary**: Returns true if the bottom edge of the collider (`self.y + self.h`) is
        //     greater than or equal to the maximum height (`max[1]`).
        //
        // # Notes
        //
        // - Ensure that the `Collider` object is properly initialized with its position and dimensions
        //   before calling this function.
        // - This function is useful for determining if the collider is within the allowed movement area
        //   and can be used to prevent it from moving outside the defined boundaries.
        // code -----------------------------------------------------------------------------------
        (
            self.pos[0] <= 0.0,
            self.pos[1] <= 0.0,
            self.pos[0] + self.size[0] >= max[0],
            self.pos[1] + self.size[1] >= max[1]
        )
        // ----------------------------------------------------------------------------------------
    }
    pub fn resize(&mut self, sc_x: f32, sc_y: f32){
        // Resizes the object by scaling its size and position.
        //
        // This method modifies the size and position of the object by dividing
        // the current size and position coordinates by the specified scaling factors.
        // It is typically used to adjust the dimensions and location of the object
        // in response to changes in the rendering context or viewport size.
        //
        // # Parameters
        //
        // - `sc_x`: The scaling factor for the x-axis. A value greater than 1.0
        //   will reduce the size and position, while a value less than 1.0 will
        //   increase them.
        // - `sc_y`: The scaling factor for the y-axis. Similar to `sc_x`, a value
        //   greater than 1.0 will reduce the size and position, while a value
        //   less than 1.0 will increase them.
        // Ensure that the scaling factors are not zero to avoid division by zero,
        // which would result in undefined behavior or runtime errors.
        // code -----------------------------------------------------------------------------------
        self.size[0] /= sc_x;
        self.size[1] /= sc_y;
        self.pos[0] /= sc_x;
        self.pos[1] /= sc_y;
        // ----------------------------------------------------------------------------------------
    }
    pub fn update(&mut self){
        // Updates the timestamp of the object to the current time in seconds since the UNIX epoch.
        //
        // This method retrieves the current system time and calculates the duration
        // since the UNIX epoch (January 1, 1970). It then updates the `timestamp`
        // field of the object with the current time in seconds.
        //
        // # Note
        //
        // The method uses `unwrap()` to handle the result of `duration_since`, which
        // will panic if the current time is before the UNIX epoch. Ensure that the
        // system time is valid to avoid runtime panics.
        // code -----------------------------------------------------------------------------------
        self.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        match self.ppos[0] != self.pos[0] || self.ppos[1] != self.pos[1]{
            true => {
                self.state = State::Moving;
                self.speed = self.velocity / self.delay as f32;
                self.elapsed += self.velocity / self.speed;
            }
            false => {
                self.state = State::Stable;
                self.speed = 0.0;
                self.elapsed = 0.0;
            }
        }
        self.ppos = self.pos;
        // ----------------------------------------------------------------------------------------
    }
    pub fn debug(&self) -> String {
        let debug_info = format!(
            "Type: {}\nTimestamp: {}\nSpan: {}\nColor: ({},{},{},{})\nImage: {}\nPos: ({},{})\nPPos: ({}, {})\nSize: ({},{})\nVelocity: {}\nState: {:?}\nSpeed: {}\nElapsed (ms): {}\nDelay: {}",
            &self.type_, &self.timestamp, &self.span, &self.color.0, &self.color.1, &self.color.2, &self.color.3,
            &self.image, &self.pos[0], &self.pos[1], &self.ppos[0], &self.ppos[1], &self.size[0], &self.size[1],
            &self.velocity, &self.state, &self.speed, &self.elapsed, &self.delay
        );
        debug_info
    }
}
