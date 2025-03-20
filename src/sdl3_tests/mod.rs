use crate::sdl3::{
    SDL3,
    sdl3_consts::*,
    sdl3_structs::*,
    sdl3_sys::*,
    sdl3_render::*
};

use std::mem::zeroed;

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

pub fn sdl3_render_test(){
    let mut sdl3 = SDL3::new();
    sdl3.sdl3_init(SDL_INIT_VIDEO);
    let window = sdl3.sdl3_create_window("Render Test", 800, 600, SDL_VOID);
    // SDL3 docs code
    let renderer = sdl3.sdl3_create_renderer(window, "");
    let rects = [
        SDL_FRect{x: 100.0, y: 100.0, w:100.0, h:125.0},
        SDL_FRect{x: 200.0, y: 200.0, w:50.0, h:70.0},
        SDL_FRect{x: 300.0, y: 300.0, w:70.0, h:50.0},
        SDL_FRect{x: 400.0, y: 400.0, w:125.0, h:100.0},
    ];
    let mut loopShouldStop = false;
    // SDL3 docs code end
    while !loopShouldStop {
        unsafe{
            let mut event: SDL_Event = zeroed();
            while sdl3_poll_event(&mut sdl3)(&mut event as *mut SDL_Event){
                match event.type_{
                    SDL_EVENT_QUIT => {
                        loopShouldStop = true;
                        break;
                    },
                    _ => {
                        println!("{:?}", event);
                    }
                }
            }
            // SDL3 docs code
            sdl3_set_render_draw_color(&mut sdl3, renderer, 0, 0, 0, 255);
            sdl3_render_clear(&mut sdl3, renderer);
            sdl3_set_render_draw_color(&mut sdl3, renderer, 255, 255, 255, 255);
            sdl3_render_fill_rects(&mut sdl3, renderer, &rects as *const SDL_FRect, 4);
            sdl3_render_present(&mut sdl3, renderer);
            // SDL3 docs code end
            event.drop_fields();
            sdl3_delay(&mut sdl3, 16);
        }
    }
    sdl3.sdl3_destroy_renderer(renderer);
    sdl3.sdl3_destroy_window(window);
    sdl3.sdl3_quit();
}

pub fn sdl3_events_test(){
    let mut sdl3 = SDL3::new();
    sdl3.sdl3_init(SDL_INIT_VIDEO);
    let window = sdl3.sdl3_create_window("Events Test", 800, 600, SDL_VOID);
    // SDL3 docs code
    let mut loopShouldStop = false;
    // SDL3 docs code end
    while !loopShouldStop {
        unsafe{
            let mut event: SDL_Event = zeroed();
            while sdl3_poll_event(&mut sdl3)(&mut event as *mut SDL_Event){
                match event.type_{
                    SDL_EVENT_QUIT => {
                        loopShouldStop = true;
                        break;
                    },
                    _ => {
                        println!("{:?}", event);
                    }
                }
            }
            event.drop_fields();
            sdl3_delay(&mut sdl3, 16);
        }
    }
    sdl3.sdl3_destroy_window(window);
    sdl3.sdl3_quit();
}
