mod sdl3;
mod sdl3_tests;
mod collider;
mod observer;

fn main(){
    println!("SDL RENDER TEST");
    sdl3_tests::sdl3_render_test();
    println!("SDL RENDER PASSED");
}
