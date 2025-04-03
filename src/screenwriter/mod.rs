use crate::collider::Collider;

mod screenwriter_text;

#[derive(Debug, Clone)]
pub struct Scene{
    pub objects: Vec<Collider>,
    pub next_scene: u64,
    pub background: (u8, u8, u8, u8),
    pub text: String,
    pub point: [f32; 2]
}

impl Scene{
    pub fn new(objects: Vec<Collider>, next_scene: u64, background: (u8, u8, u8, u8), text: String, point: [f32; 2]) -> Self{
        // Creates a new instance of the `Scene` struct.
        //
        // This function initializes a `Scene` object with the provided parameters, including a vector
        // of colliders, the identifier for the next scene, background color, text to be displayed,
        // and a point for rendering.
        //
        // # Parameters
        //
        // - `objects`: A vector of `Collider` objects that represent the interactive elements in the
        //   scene.
        // - `next_scene`: A `u64` value representing the identifier of the next scene to transition to.
        // - `background`: A tuple of four `u8` values representing the RGBA color of the background.
        // - `text`: A `String` containing the text to be displayed in the scene.
        // - `point`: An array of two `f32` values representing the coordinates for rendering the text.
        //
        // # Returns
        //
        // Returns a new instance of the `Scene` struct initialized with the provided values.
        //
        // # Notes
        //
        // Ensure that the provided `objects` and `text` are valid for the intended use in the scene.
        // The `background` color should be specified in the range of 0 to 255 for each channel.
        // code -----------------------------------------------------------------------------------
        Scene{objects: objects, next_scene: next_scene, background: background, text: text, point: point}
        // ----------------------------------------------------------------------------------------
    }
}
