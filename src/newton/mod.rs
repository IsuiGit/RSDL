use crate::collider::Collider;
mod newton_gravity;

pub struct Newton{
    pub force: f32
}

impl Newton{
    pub fn new() -> Self{
        Self{ force: 9.81 }
    }
}
