use crate::collider::Collider;

impl Collider {

    // Подсчет глобальной коллизи с окном по четырем направлениям
    pub fn global_collide(&self, max: (i32, i32)) -> (bool, bool, bool, bool) {
        (
            self.x <= 0.0,
            self.y <= 0.0,
            self.x + self.w >= max.0 as f32,
            self.y + self.h >= max.1 as f32
        )
    }

    // Выравнивание относительно наложения объектов друг на друга и направления
    pub fn overlap_padding_l(&mut self, padding: (f32, f32)) {
        self.x += padding.0;;
    }

    // Выравнивание относительно наложения объектов друг на друга и направления
    pub fn overlap_padding_r(&mut self, padding: (f32, f32)) {
        self.x -= padding.0;
    }

    // Выравнивание относительно наложения объектов друг на друга и направления
    pub fn overlap_padding_t(&mut self, padding: (f32, f32)) {
        self.y += padding.1;
    }

    // Выравнивание относительно наложения объектов друг на друга и направления
    pub fn overlap_padding_b(&mut self, padding: (f32, f32)) {
        self.y -= padding.1;
    }
}
