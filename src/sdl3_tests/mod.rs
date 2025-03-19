use crate::sdl3::{SDL3, sdl3_consts::*, sdl3_structs::*, sdl3_sys::*};
use std::{thread, time::Duration};

pub fn sdl3_iq_test(){
    let mut sdl3 = SDL3::new();
    sdl3.sdl3_init(SDL_INIT_VIDEO);
    sdl3.sdl3_quit();
}

pub fn sdl3_window_test(){
    let mut sdl3 = SDL3::new();
    sdl3.sdl3_init(SDL_INIT_VIDEO);
    let window = sdl3.sdl3_create_window("Window Test", 800, 600, SDL_VOID);
    sdl3.sdl3_destroy_window(window);
    sdl3.sdl3_quit();
}

pub fn sdl3_poll_event_test(){
    let mut sdl3 = SDL3::new();
    sdl3.sdl3_init(SDL_INIT_VIDEO);
    let window = sdl3.sdl3_create_window("PollEvent Test", 800, 600, SDL_VOID);
    // SDL3 docs code
    let mut loopShouldStop = false;
    // SDL3 docs code end
    while !loopShouldStop {
        unsafe{
            let mut event: SDL_Event = SDL_Event{type_: 0};
            while sdl3_poll_event(&mut sdl3)(&mut event as *mut SDL_Event){
                match event.type_{
                    SDL_EVENT_QUIT => {
                        loopShouldStop = true;
                        break;
                    },
                    _ => {
                        println!("PollEvent type {}", event.type_);
                    }
                }
            }
            sdl3_delay(&mut sdl3, 16);
            println!("Outer PollEvent");
        }
    }
    sdl3.sdl3_destroy_window(window);
    sdl3.sdl3_quit();
}

pub fn sdl3_draw_test(){
    let mut sdl3 = SDL3::new();
    sdl3.sdl3_init(SDL_INIT_VIDEO);
    let window = sdl3.sdl3_create_window("Draw Test", 800, 600, SDL_VOID);
    // SDL3 docs code
    let renderer = sdl3.sdl3_create_renderer(window, "");
    if !sdl3_set_renderer_draw_color(&mut sdl3, renderer, 255, 255, 255, 255){
        println!("SDL_SetRenderDrawColor exit with error {}", sdl3_get_error(&mut sdl3));
        sdl3.sdl3_destroy_renderer(renderer);
        sdl3.sdl3_destroy_window(window);
        sdl3.sdl3_quit();
    }
    let mut loopShouldStop = false;
    // SDL3 docs code end
    while !loopShouldStop {
        unsafe{
            let mut event: SDL_Event = SDL_Event{type_: 0};
            while sdl3_poll_event(&mut sdl3)(&mut event as *mut SDL_Event){
                match event.type_{
                    SDL_EVENT_QUIT => {
                        loopShouldStop = true;
                        break;
                    },
                    _ => {
                        println!("Draw PollEvent type {}", event.type_);
                    }
                }
            }
            // SDL3 docs code
            sdl3_render_clear(&mut sdl3, renderer);
            sdl3_render_present(&mut sdl3, renderer);
            // SDL3 docs code end
            sdl3_delay(&mut sdl3, 16);
            println!("Outer Draw");
        }
    }
    sdl3.sdl3_destroy_renderer(renderer);
    sdl3.sdl3_destroy_window(window);
    sdl3.sdl3_quit();
}
