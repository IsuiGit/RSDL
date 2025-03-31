mod sdl3;
mod sdl3_tests;
mod collider;
mod observer;
mod artist;
mod screenwriter;

fn main(){
    println!("SDL MOVEMENT & SCENE RENDER TEST\nPRESS A/W/S/D KEYS TO MOVE");
    sdl3_tests::sdl3_osa_system_test();
    println!("SDL RENDER PASSED");
}
