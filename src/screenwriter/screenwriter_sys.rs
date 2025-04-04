use crate::screenwriter::Scene;

impl Scene{
    pub fn resize(&mut self, sc_x: f32, sc_y: f32){
        self.point[0] /= sc_x;
        self.point[1] /= sc_y;
    }
}
