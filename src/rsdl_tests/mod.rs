use crate::sdl3::{
    SDL3,
    sdl3_consts::*,
    sdl3_structs::*,
    sdl3_sys::{
        sdl3_poll_event,
        sdl3_delay
    },
    sdl3_window::sdl3_get_window_size
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
use crate::screenwriter::Scene;
use std::{collections::HashMap, mem::zeroed};

pub fn sdl3_osa_system_test(){
    let mut sdl3 = SDL3::new();
    // Создание "играбельного объекта"-------------------------------------------------------------
    let playable = Collider{
        type_: COLLIDER_PLAYABLE,
        span: ARTIST_RECTANGLE,
        color: (255, 255, 255, 255),
        pos: [100.0, 100.0],
        size: [50.0, 50.0],
        velocity: [5.0, 5.0, 5.0, 5.0]
    };
    // --------------------------------------------------------------------------------------------
    // Создание структуры "наблюдателя", с содержимым в виде "играбельного" объекта, окружения
    // сцены, окна и настроек по умолчанию (default)-----------------------------------------------
    let mut scenes: HashMap<u64, Scene> = HashMap::new();
    let scene_0 = Scene{
        objects: vec![
            Collider{
                type_: COLLIDER_BLOCK,
                span: ARTIST_RECTANGLE,
                color: (127, 65, 250, 255),
                pos: [800.0, 400.0],
                size: [300.0, 150.0],
                velocity: [0.0, 0.0, 0.0, 0.0]
            },
            Collider{
                type_: COLLIDER_BLOCK,
                span: ARTIST_RECTANGLE,
                color: (17, 145, 112, 255),
                pos: [100.0, 600.0],
                size: [300.0, 150.0],
                velocity: [0.0, 0.0, 0.0, 0.0]
            }
        ],
        next_scene: 1
    };
    let scene_1 = Scene{
        objects: vec![
            Collider{
                type_: COLLIDER_BLOCK,
                span: ARTIST_RECTANGLE,
                color: (127, 65, 250, 255),
                pos: [400.0, 800.0],
                size: [200.0, 800.0],
                velocity: [0.0, 0.0, 0.0, 0.0]
            },
            Collider{
                type_: COLLIDER_BLOCK,
                span: ARTIST_RECTANGLE,
                color: (17, 145, 112, 255),
                pos: [1820.0, 0.0],
                size: [100.0, 150.0],
                velocity: [0.0, 0.0, 0.0, 0.0]
            }
        ],
        next_scene: 0
    };
    scenes.insert(0, scene_0);
    scenes.insert(1, scene_1);
    let mut observer = Observer::init(
        &mut sdl3,
        playable,
        scenes,
        [1920.0, 1080.0],
        SDL_INIT_VIDEO | SDL_INIT_AUDIO,
        SDL_WINDOW_FULLSCREEN | SDL_WINDOW_OPENGL
    );
    // Инициализируем стандартную клавиатуру (WASD+ESC)
    observer.init_default_keyboard();
    // --------------------------------------------------------------------------------------------
    // Создание "художника" для отрисовки объектов ------------------------------------------------
    let artist = Artist{};
    // --------------------------------------------------------------------------------------------
    // Основной цикл ------------------------------------------------------------------------------
    let mut run = true;
    while run {
        unsafe{
            let mut event: SDL_Event = zeroed();
            while sdl3_poll_event(&mut sdl3)(&mut event as *mut SDL_Event){
                let size = sdl3_get_window_size(&mut sdl3, observer.window);
                match event.type_{
                    SDL_EVENT_QUIT => {
                        run = false;
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
            observer.proceed_events(&mut sdl3);
            // Artist section (SDL3 impl)----------------------------------------------------------
            artist.draw(
                &mut sdl3,
                observer.renderer,
                &observer.playable,
                &observer.scenes.get(&observer.current_scene).unwrap().objects,
                (0, 0, 0, 255)
            );
            // ------------------------------------------------------------------------------------
            event.drop_fields();
            sdl3_delay(&mut sdl3, 16);
        }
    }
    // --------------------------------------------------------------------------------------------
    observer.destroy(&mut sdl3);
}
