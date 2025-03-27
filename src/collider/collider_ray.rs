use crate::collider::{
    Collider,
    Direction
};

impl Collider {
    // Рэйкастинг по направлению движения для определения расстояния до объекта
    pub fn ray_cast(&self, object: &Collider, direction: Direction) -> f32 {
        match direction {
            Direction::Left => {
                // Расстояние до левой границы объекта
                (self.x - (object.x + object.w)).abs()
            }
            Direction::Right => {
                // Расстояние до правой границы объекта
                ((object.x - self.x) - self.w).abs()
            }
            Direction::Top => {
                // Расстояние до верхней границы объекта
                (self.y - (object.y + object.h)).abs()
            }
            Direction::Bottom => {
                // Расстояние до нижней границы объекта
                ((object.y - self.y) - self.h).abs()
            }
        }
    }

}
