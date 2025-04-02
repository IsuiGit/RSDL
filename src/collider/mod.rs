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
    pub velocity: f32
}

impl Collider{
    pub fn new(type_: u32, span: u32, color: (u8, u8, u8, u8), pos: [f32; 2], size: [f32; 2], velocity: f32) -> Self {
        Collider{type_: type_, span: span, color: color, pos: pos, size: size, velocity: velocity}
    }
}
