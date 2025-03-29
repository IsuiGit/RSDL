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
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub velocity: [f32; 4],
}
