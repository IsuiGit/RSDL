use crate::sdl3::{
    SDL3,
    sdl3_consts::*,
    sdl3_structs::*,
    sdl3_sys::*,
    sdl3_window::*,
    sdl3_render::*
};

use crate::collider::Collider;
use crate::observer::Observer;

use std::{
    mem::zeroed,
    ptr::null_mut
};

pub fn sdl3_iq_test(){
    let mut sdl3 = SDL3::new();
    sdl3_init(&mut sdl3, SDL_INIT_VIDEO);
    sdl3_quit(&mut sdl3);
}

pub fn sdl3_window_test(){
    let mut sdl3 = SDL3::new();
    sdl3_init(&mut sdl3, SDL_INIT_VIDEO);
    let window = sdl3_create_window(&mut sdl3, "Window Test", 800, 600, SDL_VOID);
    sdl3_destroy_window(&mut sdl3, window);
    sdl3_quit(&mut sdl3);
}

pub fn sdl3_render_test(){
    let mut sdl3 = SDL3::new();
    sdl3_init(&mut sdl3, SDL_INIT_VIDEO);
    // Создание коллайдеров сцены
    let mut m_rect: Collider = Collider{x: 100.0, y: 100.0, w: 128.0, h: 72.0, playable: true, max_dx: 10.0, max_dy: 10.0, velocity: 1.0, ..Collider::default()};
    let mut o1_rect: Collider = Collider{x: 800.0, y: 400.0, w:300.0, h: 150.0, ..Collider::default()};
    let mut o2_rect: Collider = Collider{x: 100.0, y: 600.0, w:300.0, h: 150.0, ..Collider::default()};
    // Создания наблюдателя сцены с настройками окна и вектором коллайдеров
    let mut observer = Observer::init(vec![m_rect, o1_rect, o2_rect], [1280.0, 720.0]);
    // Создание окна
    let window = sdl3_create_window(
        &mut sdl3, "Movement Test App",
        observer.window[0] as u32,
        observer.window[1] as u32,
        SDL_WINDOW_RESIZABLE | SDL_WINDOW_OPENGL
    );
    // SDL3 docs code
    let renderer = sdl3_create_renderer(&mut sdl3, window, "");
    let mut collide_map = observer.collision_map();
    let mut loopShouldStop = false;
    // SDL3 docs code end
    let mut keys: [bool; 4] = [false; 4];
    while !loopShouldStop {
        unsafe{
            let mut event: SDL_Event = zeroed();
            collide_map = observer.objects_collide();
            while sdl3_poll_event(&mut sdl3)(&mut event as *mut SDL_Event){
                let size = sdl3_get_window_size(&mut sdl3, window);
                match event.type_{
                    SDL_EVENT_QUIT => {
                        loopShouldStop = true;
                        break;
                    },
                    SDL_EVENT_WINDOW_RESIZED => {
                        observer.resize([size.0 as f32, size.1 as f32]);
                    }
                    SDL_EVENT_KEY_DOWN => {
                        let size = sdl3_get_window_size(&mut sdl3, window);
                        match event.key.key {
                            SDLK_A => keys[0] = true,
                            SDLK_D => keys[1] = true,
                            SDLK_W => keys[2] = true,
                            SDLK_S => keys[3] = true,
                            _ => {},
                        }
                    }
                    SDL_EVENT_KEY_UP => {
                        match event.key.key {
                            SDLK_A => keys[0] = false,
                            SDLK_D => keys[1] = false,
                            SDLK_W => keys[2] = false,
                            SDLK_S => keys[3] = false,
                            _ => {},
                        }
                    },
                    _ => println!("{:?}", event),
                }
            }
            let size = sdl3_get_window_size(&mut sdl3, window);
            if keys[0] {
                if collide_map.0 {
                    observer.objects[0].move_left(size, collide_map);
                } else {
                    println!("Left collision: {:?}", collide_map);
                    observer.objects[0].overlap_padding_l(collide_map.1);
                }
            }
            if keys[1] {
                if collide_map.0 {
                    observer.objects[0].move_right(size, collide_map);
                } else {
                    println!("Right collision: {:?}", collide_map);
                    observer.objects[0].overlap_padding_r(collide_map.1);
                }
            }
            if keys[2] {
                if collide_map.0 {
                    observer.objects[0].move_top(size, collide_map);
                } else {
                    println!("Top collision: {:?}", collide_map);
                    observer.objects[0].overlap_padding_t(collide_map.1);
                }
            }
            if keys[3] {
                if collide_map.0 {
                    observer.objects[0].move_bottom(size, collide_map);
                } else {
                    println!("Bottom collision: {:?}", collide_map);
                    observer.objects[0].overlap_padding_b(collide_map.1);
                }
            }
            // SDL3 docs code
            sdl3_set_render_draw_color(&mut sdl3, renderer, 0, 0, 0, 255);
            sdl3_render_clear(&mut sdl3, renderer);
            sdl3_set_render_draw_color(&mut sdl3, renderer, 255, 255, 255, 255);
            // Создание квадратов по параметрам коллайдеров
            let m_rect: SDL_FRect = SDL_FRect{x: observer.objects[0].x, y: observer.objects[0].y, w: observer.objects[0].w, h: observer.objects[0].h};
            let o1_rect: SDL_FRect = SDL_FRect{x: observer.objects[1].x, y: observer.objects[1].y, w: observer.objects[1].w, h: observer.objects[1].h};
            let o2_rect: SDL_FRect = SDL_FRect{x: observer.objects[2].x, y: observer.objects[2].y, w: observer.objects[2].w, h: observer.objects[2].h};
            // Отрисовка всех объектов
            let rects = [m_rect, o1_rect, o2_rect];
            sdl3_render_fill_rects(&mut sdl3, renderer, &rects as *const SDL_FRect, 3);
            sdl3_render_present(&mut sdl3, renderer);
            // SDL3 docs code end
            event.drop_fields();
            sdl3_delay(&mut sdl3, 16);
        }
    }
    sdl3_destroy_renderer(&mut sdl3, renderer);
    sdl3_destroy_window(&mut sdl3, window);
    sdl3_quit(&mut sdl3);
}
