use crate::observer::Observer;
use crate::collider::Collider;

impl Observer {
    // Просчет перекрытия объектов друг друга в момент коллизии
    fn collision_overlap(&self, playable: &Collider, obj: &Collider) -> (f32, f32) {
        let x_overlap = ((playable.x + playable.w).min(obj.x + obj.w) - (playable.x).max(obj.x)).max(0.0);
        let y_overlap = ((playable.y + playable.h).min(obj.y + obj.h) - (playable.y).max(obj.y)).max(0.0);
        (x_overlap, y_overlap)
    }

    // Просчет наличия коллизий с неигровыми объектами
    pub fn objects_collide(&self) -> (bool, (f32, f32)) {
        if let Some(playable) = self.objects.iter().find(|obj| obj.playable){
            for object in self.objects.iter().filter(|obj| !obj.playable) {
                if playable.x < object.x + object.w &&
                   playable.x + playable.w > object.x &&
                   playable.y < object.y + object.h &&
                   playable.y + playable.h > object.y  {
                       let overlap_padding = self.collision_overlap(playable, object);
                       return (false, overlap_padding);
                }
            }
        }
        (true, (0.0, 0.0))
    }

    // Иниициализация карты коллизий
    pub fn collision_map(&self) -> (bool, (f32, f32)) {
        (true, (0.0, 0.0))
    }
}
