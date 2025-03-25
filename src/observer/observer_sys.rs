use crate::observer::Observer;

impl Observer{
    // Функция скейлинга относительно изменений размера экрана и начальных настроек
    pub fn resize(&mut self, size: [f32; 2]){
        let sc_x = self.window[0] as f32 / size[0];
        let sc_y = self.window[1] as f32 / size[1];
        for obj in &mut self.objects{
            obj.w = obj.w / sc_x;
            obj.h = obj.h / sc_y;
            obj.x = obj.x / sc_x;
            obj.y = obj.y / sc_y;
        }
        self.window = size;
    }
}
