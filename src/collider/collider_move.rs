use crate::collider::Collider;

impl Collider{

    // Просчет движения относительно экрана и коллизий с объектами
    pub fn move_left(&mut self, max:[f32; 2]) {
        if !self.global_collide(max).0 {
            self.x -= self.vlx;
        }
    }

    // Просчет движения относительно экрана и коллизий с объектами
    pub fn move_right(&mut self, max: [f32; 2]) {
        if !self.global_collide(max).2 {
            self.x += self.vrx;
        }
    }

    // Просчет движения относительно экрана и коллизий с объектами
    pub fn move_top(&mut self, max: [f32; 2]) {
        if !self.global_collide(max).1 {
            self.y -= self.vty;
        }
    }

    // Просчет движения относительно экрана и коллизий с объектами
    pub fn move_bottom(&mut self, max: [f32; 2]) {
        if !self.global_collide(max).3 {
            self.y += self.vby;
        }
    }
}
