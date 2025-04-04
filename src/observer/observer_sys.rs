use crate::observer::Observer;

impl Observer{
    pub fn resize(&mut self, size: [f32; 2]){
        // Handles resizing of the game window and adjusts the sizes and positions of the playable character
        // and all objects accordingly.
        //
        // This function is responsible for updating the dimensions and positions of the playable character
        // and all other objects in the game when the window size changes. It calculates the scaling factors
        // based on the new window size and applies these factors to resize the playable character and objects.
        //
        // # Parameters
        //
        // - `size`: An array of two `f32` values representing the new size of the window.
        //   - `size[0]`: The new width of the window.
        //   - `size[1]`: The new height of the window.
        //
        // # Functionality
        //
        // - The function calculates the scaling factors (`sc_x` and `sc_y`) for the width and height
        //   based on the ratio of the current window size to the new size.
        // - It then calls `resize_playable` to adjust the size and position of the playable character
        //   according to the calculated scaling factors.
        // - After that, it calls `resize_objects` to resize and reposition all other objects in the game.
        // - Finally, it updates the current window size to the new size.
        //
        // # Notes
        //
        // - Ensure that the `resize_playable` and `resize_objects` methods are called after calculating
        //   the scaling factors to maintain the correct proportions of the game elements.
        // - This function assumes that the `playable` character and `objects` are properly initialized
        //   before resizing.
        // code -----------------------------------------------------------------------------------
        let sc_x = self.size[0] / size[0];
        let sc_y = self.size[1] / size[1];
        self.playable.resize(sc_x, sc_y);
        for obj in &mut self.scenes.get_mut(&self.current_scene).unwrap().objects{
            obj.resize(sc_x, sc_y);
        }
        self.scenes.get_mut(&self.current_scene).unwrap().resize(sc_x, sc_y);
        self.size = size;
        // -----------------------------------------------------------------------------------------
    }
}
