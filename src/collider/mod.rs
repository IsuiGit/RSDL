mod collider_sys;
mod collider_move;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Collider{
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub playable: bool,
    pub max_dx: f32,
    pub max_dy: f32,
    pub velocity: f32,
    pub vlx: f32,
    pub vrx: f32,
    pub vty: f32,
    pub vby: f32,
    pub min_dx: f32,
    pub min_dy: f32,
}
