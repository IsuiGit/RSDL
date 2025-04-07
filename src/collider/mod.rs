mod collider_sys;
mod collider_move;
pub mod collider_ray;
pub mod collider_collision;
pub mod collider_consts;

use std::{ffi::c_void, time::{SystemTime, UNIX_EPOCH}};

#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Top,
    Right,
    Bottom
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum State {
    #[default] Stable,
    Moving,
    Jump
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Collider{
    pub type_: u32,
    pub timestamp: u64,
    pub span: u32,
    pub color: (u8, u8, u8, u8),
    pub image: String,
    pub pos: [f32; 2],
    pub ppos: [f32; 2],
    pub size: [f32; 2],
    pub velocity: f32,
    pub state: State,
    pub elapsed: f32,
    pub speed: f32,
    pub delay: u32
}

impl Collider{
    pub fn new(type_: u32, span: u32, color: (u8, u8, u8, u8), image: &str, pos: [f32; 2], size: [f32; 2], velocity: f32, delay: Option<u32>) -> Self {
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
        let mut _delay = 0;
        if delay.is_some(){
            _delay = delay.unwrap();
        }
        Self{
            type_: type_,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            span: span,
            color: color,
            image: image.to_string(),
            pos: pos,
            ppos: pos,
            size: size,
            velocity: velocity,
            state: State::Stable,
            elapsed: 0.0,
            speed: 0.0,
            delay: _delay
        }
        // ----------------------------------------------------------------------------------------
    }
}
