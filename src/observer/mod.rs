use crate::collider::Collider;

mod observer_sys;
mod observer_collision;

#[derive(Debug)]
pub struct Observer{
    pub objects: Vec<Collider>,
    pub window: [f32; 2],
}

impl Observer{
    pub fn init(obj: Vec<Collider>, win: [f32; 2]) -> Self{
        Observer{
            objects: obj,
            window: win,
        }
    }
}
