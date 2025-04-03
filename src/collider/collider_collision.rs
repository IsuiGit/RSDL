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
    pub fn overlap(&mut self, size: [f32; 2], object: &Collider) {
        // Adjusts the position of the object to the nearest edge of the specified collider if the object is
        // detected to be overlapping with it. The function calculates the distances to the left and right
        // edges (dx) and the top and bottom edges (dy) of the collider. If the object is inside the collider,
        // it determines which edge is closest and moves the object accordingly.
        //
        // # Parameters
        // - `size`: An array of two `f32` values representing the width and height of the window.
        // - `object`: A reference to a `Collider` object that represents the collider to check against.
        //
        // # Logic
        // - The function first checks if the distance to the collider is zero, indicating an overlap.
        // - It calculates the distances to the left and right edges (dx) and the top and bottom edges (dy)
        //   of the collider. If the object is not overlapping, both dx and dy are set to (0.0, 0.0).
        // - If either dx or dy is not zero, it calculates the minimum distance to the edges.
        // - It then determines whether the object should be moved horizontally or vertically based on the
        //   minimum distance. The object is moved towards the nearest edge while ensuring it remains within
        //   the bounds defined by its size.
        //
        // # Note
        // - The function uses absolute values to compare distances and ensure that the object is moved in the
        //   correct direction. It also checks the object's position to prevent it from moving out of bounds.
        // code -----------------------------------------------------------------------------------
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
            // Optional: Clamp the position to ensure it stays within bounds
            self.pos[0] = self.pos[0].max(0.0).min(size[0]);
            self.pos[1] = self.pos[1].max(0.0).min(size[1]);
        }
        // ----------------------------------------------------------------------------------------
    }
}
