use crate::collider::Collider;

#[derive(Debug)]
pub struct Scene{
    pub objects: Vec<Collider>,
    pub next_scene: u64,
}
