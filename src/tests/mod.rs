use crate::sdl3::{SDL3, sdl3_consts::*, sdl3_structs::*, sdl3_sys::{sdl3_poll_event, sdl3_delay}, sdl3_window::sdl3_get_window_size};
use crate::collider::{Collider, collider_consts::*};
use crate::artist::{Artist, artist_consts::*};
use crate::observer::{Observer, observer_consts::*};
use crate::screenwriter::Scene;

use std::{collections::HashMap, mem::zeroed};

pub fn sdl3_main_test(){
    let delay = SDL_MIN_DELAY;
    // Создаем экземпляр библиотек ----------------------------------------------------------------
    let mut sdl3 = SDL3::new();
    // --------------------------------------------------------------------------------------------
    // Создание "играбельного объекта"-------------------------------------------------------------
    let playable = Collider::new(
        COLLIDER_PLAYABLE,
        ARTIST_IMAGE,
        OBSERVER_COLOR_DEFAULT,
        "C:\\Users\\Isui\\Desktop\\src\\Bobo.png",
        [0.0, 1016.0],
        [64.0, 64.0],
        5.0, Some(SDL_MIN_DELAY)
    );
    // --------------------------------------------------------------------------------------------
    // Создание структуры "наблюдателя", с содержимым в виде "играбельного" объекта, окружения
    // сцены, окна и настроек по умолчанию (default)-----------------------------------------------
    let mut scenes: HashMap<u64, Scene> = HashMap::new();
    // --------------------------------------------------------------------------------------------
    // Создаем сцены игры -------------------------------------------------------------------------
    let scene_0 = Scene::new(
        vec![
            Collider::new(COLLIDER_BLOCK, ARTIST_IMAGE, OBSERVER_COLOR_DEFAULT, "", [400.0, 552.0], [200.0, 650.0], 0.0, None),
            Collider::new(COLLIDER_BLOCK, ARTIST_IMAGE, OBSERVER_COLOR_DEFAULT, "", [1111.0, 243.0], [200.0, 950.0], 0.0, None),
            Collider::new(COLLIDER_BLOCK, ARTIST_IMAGE, OBSERVER_COLOR_DEFAULT, "", [284.0, 650.0], [1300.0, 200.0], 0.0, None)
        ],
        1, (0, 0, 0, 255), String::from("Q to change scene\nWASD to move\nESC to exit"), [1670.0, 20.0]
    );
    let scene_1 = Scene::new(vec![], 0, (0, 0, 0, 255), String::from("Q to change scene\nWASD to move\nESC to exit"), [1670.0, 20.0]);
    // --------------------------------------------------------------------------------------------
    // Добавляем сцены в структуру сцен -----------------------------------------------------------
    scenes.insert(0, scene_0);
    scenes.insert(1, scene_1);
    // --------------------------------------------------------------------------------------------
    // Инициализируем "наблюдателя" ---------------------------------------------------------------
    let mut observer = Observer::new(
    &mut sdl3,
        playable,
        scenes,
        [1920.0, 1080.0],
        SDL_INIT_VIDEO | SDL_INIT_AUDIO,
        SDL_WINDOW_FULLSCREEN | SDL_WINDOW_OPENGL,
        "Main test"
    );
    // --------------------------------------------------------------------------------------------
    // Инициализируем стандартную клавиатуру
    observer.platformer_keyboard();
    // --------------------------------------------------------------------------------------------
    // Создание "художника" для отрисовки объектов ------------------------------------------------
    let artist = Artist::new(&mut sdl3, observer.renderer, "", 16.0);
    // --------------------------------------------------------------------------------------------
    // Основной цикл ------------------------------------------------------------------------------
    let mut run = true;
    while run {
        unsafe{
            let mut event: SDL_Event = zeroed();
            while sdl3_poll_event(&mut sdl3)(&mut event as *mut SDL_Event){
                let size = sdl3_get_window_size(&mut sdl3, observer.window);
                match event.type_{
                    SDL_EVENT_QUIT => { run = false; break; },
                    SDL_EVENT_WINDOW_RESIZED => { observer.resize([size.0 as f32, size.1 as f32]); },
                    SDL_EVENT_KEY_DOWN => { observer.events.insert(event.key.key); },
                    SDL_EVENT_KEY_UP => { observer.events.remove(&event.key.key); },
                    _ => {},
                }
            }
            observer.proceed_events(&mut sdl3);
            // Artist section ---------------------------------------------------------------------
            artist.drawing(&mut sdl3, observer.observer_to_artist_context());
            // ------------------------------------------------------------------------------------
            event.drop_fields();
            sdl3_delay(&mut sdl3, delay);
        }
    }
    // --------------------------------------------------------------------------------------------
    artist.destroy(&mut sdl3);
    observer.destroy(&mut sdl3);
}
