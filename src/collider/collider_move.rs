use crate::collider::Collider;

impl Collider{

    // Просчет движения относительно экрана и коллизий с объектами
    pub fn move_left(&mut self, max: (i32, i32), collide: (bool, (f32, f32))) {
        if !self.global_collide(max).0 && collide.0 {
            self.x += self.velocity;
        }
    }

    // Просчет движения относительно экрана и коллизий с объектами
    pub fn move_right(&mut self, max: (i32, i32), collide: (bool, (f32, f32))) {
        if !self.global_collide(max).2 && collide.0 {
            self.x -= self.velocity;
        }
    }

    // Просчет движения относительно экрана и коллизий с объектами
    pub fn move_top(&mut self, max: (i32, i32), collide: (bool, (f32, f32))) {
        if !self.global_collide(max).1 && collide.0 {
            self.y += self.velocity;
        }
    }

    // Просчет движения относительно экрана и коллизий с объектами
    pub fn move_bottom(&mut self, max: (i32, i32), collide: (bool, (f32, f32))) {
        if !self.global_collide(max).3 && collide.0 {
            self.y += self.velocity;
        }
    }
}
