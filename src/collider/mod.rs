mod collider_sys;
mod collider_move;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Collider{
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub playable: bool,
    pub velocity: f32,
}
