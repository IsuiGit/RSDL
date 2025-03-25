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

pub fn sdl3_render_test(){
    let mut sdl3 = SDL3::new();
    sdl3_init(&mut sdl3, SDL_INIT_VIDEO);
    // Создание коллайдеров сцены
    let mut m_rect: Collider = Collider{x: 100.0, y: 100.0, w: 128.0, h: 72.0, playable: true, velocity: 1.0, ..Collider::default()};
    let mut o1_rect: Collider = Collider{x: 800.0, y: 400.0, w:300.0, h: 150.0, ..Collider::default()};
    let mut o2_rect: Collider = Collider{x: 100.0, y: 600.0, w:300.0, h: 150.0, ..Collider::default()};
    // Создания наблюдателя сцены с настройками окна и вектором коллайдеров
    let mut observer = Observer::init(vec![m_rect, o1_rect, o2_rect], [1280.0, 720.0]);
    let mut collide_map = observer.collision_map();
    // Создание окна
    let window = sdl3_create_window(
        &mut sdl3, "Movement Test App",
        observer.window[0] as u32,
        observer.window[1] as u32,
        SDL_WINDOW_RESIZABLE | SDL_WINDOW_OPENGL
    );
    // SDL3 docs code
    let renderer = sdl3_create_renderer(&mut sdl3, window, "");
    let mut loopShouldStop = false;
    // SDL3 docs code end
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
                    },
                    SDL_EVENT_KEY_DOWN => {},
                    SDL_EVENT_KEY_UP => {},
                    _ => println!("{:?}", event),
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
