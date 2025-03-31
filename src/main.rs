mod sdl3;
mod collider;
mod observer;
mod artist;
mod screenwriter;

mod rsdl_tests;

fn main(){
    println!("SDL MOVEMENT & SCENE RENDER TEST\nPRESS A/W/S/D KEYS TO MOVE");
    rsdl_tests::sdl3_osa_system_test();
    println!("SDL RENDER PASSED");
}
