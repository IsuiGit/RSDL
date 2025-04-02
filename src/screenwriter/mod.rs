use crate::collider::Collider;

#[derive(Debug)]
pub struct Scene{
    pub objects: Vec<Collider>,
    pub next_scene: u64,
    pub background: (u8, u8, u8, u8),
    pub text: String,
    pub point: [f32; 2]
}

impl Scene{
    pub fn new(
        objects: Vec<Collider>,
        next_scene: u64,
        background: (u8, u8, u8, u8),
        text: String,
        point: [f32; 2]
    ) -> Self{
        Scene{
            objects: objects,
            next_scene: next_scene,
            background: background,
            text: text,
            point: point
        }
    }
}
