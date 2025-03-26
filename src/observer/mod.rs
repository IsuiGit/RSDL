use crate::collider::Collider;
use std::collections::HashSet;

mod observer_sys;
mod observer_events;

#[derive(Debug, Default)]
pub struct Observer{
    pub playable: Collider,
    pub objects: Vec<Collider>,
    pub window: [f32; 2],
    pub events: HashSet<u32>,
}
