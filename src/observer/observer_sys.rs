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
        self.resize_playable(sc_x, sc_y);
        self.resize_objects(sc_x, sc_y);
        self.size = size;
        // -----------------------------------------------------------------------------------------
    }

    fn resize_playable(&mut self, sc_x: f32, sc_y: f32){
        // Resizes the playable character based on the scaling factors.
        //
        // This function adjusts the width, height, and position of the playable character according
        // to the provided scaling factors. It ensures that the character maintains its proportions
        // relative to the new window size.
        //
        // # Parameters
        //
        // - `sc_x`: The scaling factor for the width.
        // - `sc_y`: The scaling factor for the height.
        //
        // # Functionality
        //
        // - The function updates the width and height of the playable character by dividing them by
        //   the respective scaling factors.
        // - It also updates the x and y positions of the playable character to maintain its position
        //   relative to the new window size.
        //
        // # Notes
        //
        // - This function should be called as part of the resizing process to ensure the playable
        //   character is correctly resized.
        // code -----------------------------------------------------------------------------------
        self.playable.size[0] /= sc_x;
        self.playable.size[1] /= sc_y;
        self.playable.pos[0] /= sc_x;
        self.playable.pos[1] /= sc_y;
        // ----------------------------------------------------------------------------------------
    }

    fn resize_objects(&mut self, sc_x: f32, sc_y: f32){
        // Resizes all objects in the game based on the scaling factors.
        //
        // This function adjusts the width, height, and position of all objects in the game according
        // to the provided scaling factors. It ensures that all objects maintain their proportions
        // relative to the new window size.
        //
        // # Parameters
        //
        // - `sc_x`: The scaling factor for the width.
        // - `sc_y`: The scaling factor for the height.
        //
        // # Functionality
        //
        // - The function iterates over all objects and updates their width, height, and positions
        //   by dividing them by the respective scaling factors.
        //
        // # Notes
        //
        // - This function should be called as part of the resizing process to ensure all objects
        //   are correctly resized.
        // code -----------------------------------------------------------------------------------
        for obj in &mut self.scenes.get_mut(&self.current_scene).unwrap().objects{
            obj.size[0] /= sc_x;
            obj.size[1] /= sc_y;
            obj.pos[0] /= sc_x;
            obj.pos[1] /= sc_y;
        }
        // ----------------------------------------------------------------------------------------
    }
}
