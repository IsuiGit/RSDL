use crate::collider::Collider;

impl Collider{
    // Просчет дистанции между объектами
    pub fn distance_to(&self, object: &Collider) -> f32 {
            // Вычисляем расстояния до границ
            let dx = if self.x + self.w < object.x {
                // self находится слева от object
                object.x - (self.x + self.w)
            } else if self.x > object.x + object.w {
                // self находится справа от object
                self.x - (object.x + object.w)
            } else {
                // Объекты пересекаются по оси X
                0.0
            };
            let dy = if self.y + self.h < object.y {
                // self находится выше object
                object.y - (self.y + self.h)
            } else if self.y > object.y + object.h {
                // self находится ниже object
                self.y - (object.y + object.h)
            } else {
                // Объекты пересекаются по оси Y
                0.0
            };
            // Если объекты пересекаются, расстояние равно 0
            if dx == 0.0 && dy == 0.0 {
                return 0.0;
            }
            // Вычисляем евклидово расстояние
            (dx * dx + dy * dy).sqrt()
        }

}
