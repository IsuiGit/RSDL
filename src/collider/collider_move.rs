use crate::collider::Collider;

impl Collider{

    pub fn velocity(&self) -> f32 {
        ((self.vlx*self.vrx+self.vty*self.vby+self.velocity)/(self.max_dx*self.max_dy)).sqrt()
    }

    // Просчет движения относительно экрана и коллизий с объектами
    pub fn move_left(&mut self, max: (i32, i32), collide: (bool, (f32, f32))) {
        if !self.global_collide(max).0 && collide.0 {
            if self.vlx < self.max_dx {
                self.vlx += self.velocity();
            }
            self.x -= self.vlx;
        }
    }

    // Просчет движения относительно экрана и коллизий с объектами
    pub fn move_right(&mut self, max: (i32, i32), collide: (bool, (f32, f32))) {
        if !self.global_collide(max).2 && collide.0 {
            if self.vrx < self.max_dx {
                self.vrx += self.velocity();
            }
            self.x += self.vrx;
        }
    }

    // Просчет движения относительно экрана и коллизий с объектами
    pub fn move_top(&mut self, max: (i32, i32), collide: (bool, (f32, f32))) {
        if !self.global_collide(max).1 && collide.0 {
            if self.vty < self.max_dy {
                self.vty += self.velocity();
            }
            self.y -= self.vty;
        }
    }

    // Просчет движения относительно экрана и коллизий с объектами
    pub fn move_bottom(&mut self, max: (i32, i32), collide: (bool, (f32, f32))) {
        if !self.global_collide(max).3 && collide.0 {
            if self.vby < self.max_dy {
                self.vby += self.velocity();
            }
            self.y += self.vby;
        }
    }
}
