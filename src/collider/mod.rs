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
        // Creates a new instance of the `Collider` struct.
        //
        // This function initializes a `Collider` object with the specified properties, including its
        // type, span, color, position, size, and velocity.
        //
        // # Parameters
        //
        // - `type_`: A `u32` value representing the type of the collider (e.g., block, trigger).
        // - `span`: A `u32` value that defines the span or range of the collider.
        // - `color`: A tuple of four `u8` values representing the RGBA color of the collider.
        // - `pos`: An array of two `f32` values representing the x and y coordinates of the collider's
        //   position.
        // - `size`: An array of two `f32` values representing the width and height of the collider.
        // - `velocity`: A `f32` value representing the velocity of the collider.
        //
        // # Returns
        //
        // Returns a new instance of the `Collider` struct initialized with the provided values.
        //
        // # Notes
        //
        // Ensure that the `color` values are specified in the range of 0 to 255 for each channel.
        // The `pos` and `size` should be set according to the intended placement and dimensions of
        // the collider in the game world.
        // code -----------------------------------------------------------------------------------
        Collider{type_: type_, span: span, color: color, pos: pos, size: size, velocity: velocity}
        // ----------------------------------------------------------------------------------------
    }
}
