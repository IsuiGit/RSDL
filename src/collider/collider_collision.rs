use crate::collider::Collider;

impl Collider{
    pub fn distance_to(&self, object: &Collider) -> f32 {
        // Calculates the Euclidean distance from the current collider to another collider.
        //
        // This function computes the distance between the current collider and the specified
        // `object` collider. It takes into account the positions and dimensions of both colliders
        // to determine the shortest distance between their edges. If the colliders overlap, the
        // function returns a distance of zero.
        //
        // # Parameters
        //
        // - `object`: A reference to another `Collider` object to which the distance is calculated.
        //
        // # Returns
        //
        // Returns the Euclidean distance as a `f32`. If the two colliders overlap, the function
        // returns `0.0`. Otherwise, it returns the shortest distance between the edges of the two
        // colliders.
        //
        // # Functionality
        //
        // - The function first calculates the horizontal distance (`dx`) between the two colliders.
        //   If the current collider is completely to the left or right of the `object`, it computes
        //   the distance accordingly. If they overlap horizontally, `dx` is set to `0.0`.
        //
        // - Next, it calculates the vertical distance (`dy`) in a similar manner. If the current
        //   collider is completely above or below the `object`, it computes the distance. If they
        //   overlap vertically, `dy` is set to `0.0`.
        //
        // - If both `dx` and `dy` are `0.0`, indicating that the colliders overlap, the function
        //   returns `0.0`. Otherwise, it calculates and returns the Euclidean distance using the
        //   Pythagorean theorem.
        // code -----------------------------------------------------------------------------------
        let dx = if self.x + self.w < object.x {
            object.x - (self.x + self.w)
        } else if self.x > object.x + object.w {
            self.x - (object.x + object.w)
        } else {
            0.0
        };
        let dy = if self.y + self.h < object.y {
            object.y - (self.y + self.h)
        } else if self.y > object.y + object.h {
            self.y - (object.y + object.h)
        } else {
            0.0
        };
        if dx == 0.0 && dy == 0.0 {
            return 0.0;
        }
        (dx * dx + dy * dy).sqrt()
        // ----------------------------------------------------------------------------------------
    }
}
