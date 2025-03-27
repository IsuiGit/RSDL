use crate::observer::Observer;
use crate::collider::{
    Collider,
    Direction,
    collider_consts::*,
};
use crate::sdl3::sdl3_consts::*;

impl Observer {
    pub fn proceed_events(&mut self){
        let objects: Vec<&Collider> = self.objects.iter().filter(|obj| obj.type_ == COLLIDER_BLOCK).collect();
        match self.events.contains(&SDLK_A){
            true => {
                let mut m_left = true;
                let direction = Direction::Left;
                for obj in &objects{
                    if self.playable.ray_cast(obj, direction.clone()) < self.playable.vlx  && self.playable.distance_to(obj) == 0.0 {
                        m_left = false;
                    }
                }
                if m_left{
                    self.playable.direction_move(self.window, direction.clone());
                }
            }
            _ => {}
        }
        match self.events.contains(&SDLK_W){
            true => {
                let mut m_top = true;
                let direction = Direction::Top;
                for obj in &objects{
                    if self.playable.ray_cast(obj, direction.clone()) < self.playable.vty  && self.playable.distance_to(obj) == 0.0 {
                        m_top = false;
                    }
                }
                if m_top{
                    self.playable.direction_move(self.window, direction.clone());
                }
            }
            _ => {}
        }
        match self.events.contains(&SDLK_D){
            true => {
                let mut m_right = true;
                let direction = Direction::Right;
                for obj in &objects{
                    if self.playable.ray_cast(obj, direction.clone()) < self.playable.vrx  && self.playable.distance_to(obj) == 0.0 {
                        m_right = false;
                    }
                }
                if m_right{
                    self.playable.direction_move(self.window, direction.clone());
                }
            }
            _ => {}
        }
        match self.events.contains(&SDLK_S){
            true => {
                let mut m_bottom = true;
                let direction = Direction::Bottom;
                for obj in &objects{
                    if self.playable.ray_cast(obj, direction.clone()) < self.playable.vby && self.playable.distance_to(obj) == 0.0 {
                        m_bottom = false;
                    }
                }
                if m_bottom{
                    self.playable.direction_move(self.window, direction.clone());
                }
             }
            _ => {}
        }
    }
}
