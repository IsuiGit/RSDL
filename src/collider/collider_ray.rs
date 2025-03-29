use crate::collider::{
    Collider,
    Direction
};

impl Collider {
    pub fn ray_cast(&self, object: &Collider, direction: Direction) -> f32 {
        // Calculates the distance from the current collider to another collider in a specified direction.
        //
        // This function performs a ray-casting operation to determine the distance between the
        // current collider and the specified `object` collider in the given direction (Left, Right,
        // Top, or Bottom). The distance is calculated as the absolute difference between the edges
        // of the two colliders in the specified direction.
        //
        // # Parameters
        //
        // - `object`: A reference to another `Collider` object to which the distance is calculated.
        // - `direction`: An instance of the `Direction` enum that specifies the direction in which
        //   the distance is measured.
        //
        // # Returns
        //
        // Returns the distance as a `f32` value. The distance represents how far the current collider
        // is from the specified `object` in the given direction.
        //
        // # Functionality
        //
        // - The function uses a match statement to determine the direction of the ray cast.
        // - For each direction, it calculates the distance based on the positions and dimensions
        //   of the current collider and the `object`:
        //   - **Left**: The distance is calculated as the absolute difference between the left edge
        //     of the current collider and the right edge of the `object`.
        //   - **Right**: The distance is calculated as the absolute difference between the right edge
        //     of the current collider and the left edge of the `object`.
        //   - **Top**: The distance is calculated as the absolute difference between the top edge
        //     of the current collider and the bottom edge of the `object`.
        //   - **Bottom**: The distance is calculated as the absolute difference between the bottom edge
        //     of the current collider and the top edge of the `object`.
        //
        // # Notes
        //
        // - Ensure that the `Collider` objects are properly initialized with their positions and dimensions
        //   before calling this function.
        // - The function assumes that the colliders are axis-aligned rectangles.
        // code -----------------------------------------------------------------------------------
        match direction {
            Direction::Left => {
                (self.pos[0] - (object.pos[0] + object.size[0])).abs()
            }
            Direction::Right => {
                ((object.pos[0] - self.pos[0]) - self.size[0]).abs()
            }
            Direction::Top => {
                (self.pos[1] - (object.pos[1] + object.size[1])).abs()
            }
            Direction::Bottom => {
                ((object.pos[1] - self.pos[1]) - self.size[1]).abs()
            }
        }
        // ----------------------------------------------------------------------------------------
    }
}
