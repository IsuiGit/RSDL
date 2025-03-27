mod sdl3;
mod sdl3_tests;
mod collider;
mod observer;
mod artist;

fn main(){
    println!("SDL MOVEMENT & RENDER TEST\nPRESS A/W/S/D KEYS TO MOVE");
    sdl3_tests::sdl3_movement_system_test();
    println!("SDL RENDER PASSED");
}
