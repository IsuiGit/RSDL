use crate::collider::Collider;

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
}
