use crate::collider::Collider;

impl Collider {
    // Подсчет глобальной коллизи с окном по четырем направлениям
    pub fn global_collide(&self, max: [f32; 2]) -> (bool, bool, bool, bool) {
        (
            self.x <= 0.0,
            self.y <= 0.0,
            self.x + self.w >= max[0],
            self.y + self.h >= max[1]
        )
    }
    
}
