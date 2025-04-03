use crate::screenwriter::Scene;

impl Scene{
    pub fn change_text(&mut self, text: &str){
        self.text = String::from(text);
    }
}
