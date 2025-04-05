use crate::collider::Collider;
mod newton_gravity;

pub struct Newton{
    pub force: f32
}

impl Newton{
    pub fn new(force: f32) -> Self{
        Self{ force }
    }
}
