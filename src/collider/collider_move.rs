use crate::collider::{
    Collider,
    Direction
};

impl Collider{

    pub fn direction_move(&mut self, max: [f32; 2], direction: Direction) {
        match direction {
            Direction::Left => {
                if !self.global_collide(max).0 {
                    self.x -= self.vlx;
                }
            }
            Direction::Top => {
                if !self.global_collide(max).1 {
                    self.y -= self.vty;
                }
            }
            Direction::Right => {
                if !self.global_collide(max).2 {
                    self.x += self.vrx;
                }
            }
            Direction::Bottom => {
                if !self.global_collide(max).3 {
                    self.y += self.vby;
                }
            }
        }
    }

}
