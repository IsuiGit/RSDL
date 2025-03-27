use crate::sdl3::{
    SDL3,
    sdl3_consts::*,
    sdl3_structs::*,
    sdl3_sys::*,
    sdl3_window::*,
    sdl3_render::*
};

use crate::collider::{
    Collider,
    collider_consts::*
};

use crate::artist::{
    Artist,
    artist_consts::*
};

use crate::observer::Observer;

use std::mem::zeroed;

pub fn sdl3_movement_system_test(){
    let mut sdl3 = SDL3::new();
    sdl3_init(&mut sdl3, SDL_INIT_VIDEO | SDL_INIT_AUDIO);
    // Создание "играбельного объекта"-------------------------------------------------------------
    let mut playable = Collider{
        type_: COLLIDER_PLAYABLE,
        span: ARTIST_RECTANGLE,
        color: (255, 255, 255, 255),
        x: 100.0,
        y: 100.0,
        w: 50.0,
        h: 50.0,
        ..Collider::default()
    };
    playable.init(5.0);
    // --------------------------------------------------------------------------------------------
    // Создание структуры "наблюдателя", с содержимым в виде "играбельного" объекта, окружения
    // сцены, окна и настроек по умолчанию (default)-----------------------------------------------
    let mut observer = Observer{
        playable: playable,
        objects: vec![
            Collider{
                type_: COLLIDER_BLOCK,
                span: ARTIST_RECTANGLE,
                color: (127, 65, 250, 255),
                x: 800.0,
                y: 400.0,
                w:300.0,
                h: 150.0,
                ..Collider::default()
            },
            Collider{
                type_: COLLIDER_BLOCK,
                span: ARTIST_RECTANGLE,
                color: (17, 145, 112, 255),
                x: 100.0,
                y: 600.0,
                w:300.0,
                h: 150.0,
                ..Collider::default()
            }
        ],
        window: [1280.0, 720.0],
        ..Observer::default()
    };
    // --------------------------------------------------------------------------------------------
    // Создание окна и рендера---------------------------------------------------------------------
    let window = sdl3_create_window(
        &mut sdl3, "Movement Test App",
        observer.window[0] as u32,
        observer.window[1] as u32,
        SDL_WINDOW_RESIZABLE | SDL_WINDOW_OPENGL
    );
    let renderer = sdl3_create_renderer(&mut sdl3, window, "");
    // --------------------------------------------------------------------------------------------
    // Создание "художника" для отрисовки объектов ------------------------------------------------
    let artist = Artist{};
    // --------------------------------------------------------------------------------------------
    // Основной цикл ------------------------------------------------------------------------------
    let mut loopShouldStop = false;
    while !loopShouldStop {
        unsafe{
            let mut event: SDL_Event = zeroed();
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
                    SDL_EVENT_KEY_DOWN => { observer.events.insert(event.key.key); },
                    SDL_EVENT_KEY_UP => { observer.events.remove(&event.key.key); },
                    _ => {},
                }
            }
            observer.proceed_events();
            // Artist section (SDL3 impl)----------------------------------------------------------
            sdl3_set_render_draw_color(&mut sdl3, renderer, 0, 0, 0, 255);
            sdl3_render_clear(&mut sdl3, renderer);
            artist.draw(&mut sdl3, renderer, &observer.playable, &observer.objects);
            sdl3_render_present(&mut sdl3, renderer);
            // ------------------------------------------------------------------------------------
            event.drop_fields();
            sdl3_delay(&mut sdl3, 16);
        }
    }
    // --------------------------------------------------------------------------------------------
    sdl3_destroy_renderer(&mut sdl3, renderer);
    sdl3_destroy_window(&mut sdl3, window);
    sdl3_quit(&mut sdl3);
}
