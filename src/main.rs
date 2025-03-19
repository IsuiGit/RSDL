mod sdl3;
mod sdl3_tests;

fn main(){
    println!("SDL INIT/QUIT TEST");
    sdl3_tests::sdl3_iq_test();
    println!("SDL INIT/QUIT TEST PASSED");

    println!("SDL CREATE WINDOW TEST");
    sdl3_tests::sdl3_window_test();
    println!("SDL CREATE WINDOW PASSED");

    println!("SDL POLL EVENT TEST");
    sdl3_tests::sdl3_poll_event_test();
    println!("SDL POLL EVENT PASSED");

    println!("SDL DRAW TEST");
    sdl3_tests::sdl3_draw_test();
    println!("SDL DRAW TEST PASSED");
}
