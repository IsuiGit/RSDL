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
        let dx = if self.pos[0] + self.size[0] < object.pos[0] {
            object.pos[0] - (self.pos[0] + self.size[0])
        } else if self.pos[0] > object.pos[0] + object.size[0] {
            self.pos[0] - (object.pos[0] + object.size[0])
        } else {
            0.0
        };
        let dy = if self.pos[1] + self.size[1] < object.pos[1] {
            object.pos[1] - (self.pos[1] + self.size[1])
        } else if self.pos[1] > object.pos[1] + object.size[1] {
            self.pos[1] - (object.pos[1] + object.size[1])
        } else {
            0.0
        };
        if dx == 0.0 && dy == 0.0 {
            return 0.0;
        }
        (dx * dx + dy * dy).sqrt()
        // ----------------------------------------------------------------------------------------
    }

    pub fn nearest_edge(&mut self, size: [f32; 2], object: &Collider) {
        // ----------------------------------------------------------------------------------------
        let dx = if self.distance_to(object) == 0.0 {
            (object.pos[0] - (self.pos[0] + self.size[0]), (object.pos[0] + object.size[0]) - self.pos[0])
        } else {
            (0.0, 0.0)
        };

        let dy = if self.distance_to(object) == 0.0 {
            (object.pos[1] - (self.pos[1] + self.size[1]), (object.pos[1] + object.size[1]) - self.pos[1])
        } else {
            (0.0, 0.0)
        };
        if dx != (0.0, 0.0) || dy != (0.0, 0.0) {
            let min_x = dx.0.abs().min(dx.1.abs());
            let min_y = dy.0.abs().min(dy.1.abs());
            if min_x < min_y {
                if dx.0.abs() < dx.1.abs() {
                    if self.pos[0] + dx.0 > 0.0 {self.pos[0] += dx.0;} else {self.pos[0] += dx.1;}
                } else {
                    if self.pos[0] + dx.1 < size[0] {self.pos[0] += dx.1;} else {self.pos[0] += dx.0;}
                }
            } else {
                if dy.0.abs() < dy.1.abs() {
                    if self.pos[1] + dy.0 > 0.0 {self.pos[1] += dy.0;} else {self.pos[1] += dy.1;}
                } else {
                    if self.pos[1] + dy.1 < size[1] {self.pos[1] += dy.1;} else {self.pos[1] += dy.0;}
                }
            }
        }
    // --------------------------------------------------------------------------------------------
    }
}
