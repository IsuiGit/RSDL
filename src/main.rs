mod sdl3;
mod sdl3_tests;

fn main(){
    println!("SDL RENDER TEST");
    sdl3_tests::sdl3_render_test();
    println!("SDL RENDER PASSED");
    println!("SDL EVENTS TEST");
    sdl3_tests::sdl3_events_test();
    println!("SDL EVENTS PASSED");
}
