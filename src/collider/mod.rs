mod collider_sys;
mod collider_move;
pub mod collider_ray;
pub mod collider_collision;
pub mod collider_consts;

#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Collider{
    pub type_: u32,
    pub span: u32,
    pub color: (u8, u8, u8, u8),
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub vlx: f32,
    pub vty: f32,
    pub vrx: f32,
    pub vby: f32,
}

impl Collider{
    pub fn init(&mut self, vel: f32){
        self.vlx = vel;
        self.vty = vel;
        self.vrx = vel;
        self.vby = vel;
    }
}
