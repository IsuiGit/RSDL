use crate::observer::Observer;
use crate::collider::{
    Collider,
    collider_consts::*,
    collider_ray::Direction,
};
use crate::sdl3::sdl3_consts::*;

impl Observer {
    pub fn proceed_events(&mut self){
        let objects: Vec<&Collider> = self.objects.iter().filter(|obj| obj.type_ == COLLIDER_BLOCK).collect();
        match self.events.contains(&SDLK_A){
            true => {
                let mut m_left = true;
                for obj in &objects{
                    if self.playable.ray_cast(obj, Direction::Left) < self.playable.vlx  && self.playable.distance_to(obj) == 0.0 {
                        m_left = false;
                    }
                }
                if m_left{
                    self.playable.move_left(self.window);
                }
            }
            _ => {}
        }
        match self.events.contains(&SDLK_W){
            true => {
                let mut m_top = true;
                for obj in &objects{
                    if self.playable.ray_cast(obj, Direction::Top) < self.playable.vty  && self.playable.distance_to(obj) == 0.0 {
                        m_top = false;
                    }
                }
                if m_top{
                    self.playable.move_top(self.window);
                }
            }
            _ => {}
        }
        match self.events.contains(&SDLK_D){
            true => {
                let mut m_right = true;
                for obj in &objects{
                    if self.playable.ray_cast(obj, Direction::Right) < self.playable.vrx  && self.playable.distance_to(obj) == 0.0 {
                        m_right = false;
                    }
                }
                if m_right{
                    self.playable.move_right(self.window);
                }
            }
            _ => {}
        }
        match self.events.contains(&SDLK_S){
            true => {
                let mut m_bottom = true;
                for obj in &objects{
                    if self.playable.ray_cast(obj, Direction::Bottom) < self.playable.vby && self.playable.distance_to(obj) == 0.0 {
                        m_bottom = false;
                    }
                }
                if m_bottom{
                    self.playable.move_bottom(self.window);
                }
             }
            _ => {}
        }
    }
}
