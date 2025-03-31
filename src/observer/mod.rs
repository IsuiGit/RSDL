use crate::collider::Collider;
use crate::screenwriter::Scene;
use std::{collections::{HashSet, HashMap}, ffi::c_void};
use crate::sdl3::{
    SDL3,
    sdl3_window::{
        sdl3_create_window,
        sdl3_destroy_window
    },
    sdl3_render::{
        sdl3_create_renderer,
        sdl3_destroy_renderer
    },
    sdl3_sys::{
        sdl3_init,
        sdl3_ttf_init,
        sdl3_quit,
        sdl3_ttf_quit
    }
};

mod observer_sys;
mod observer_events;
mod observer_keyboard;
pub mod observer_consts;

#[derive(Debug)]
pub struct Observer{
    pub playable: Collider,
    pub scenes: HashMap<u64, Scene>,
    pub current_scene: u64,
    pub size: [f32; 2],
    pub events: HashSet<u32>,
    pub keyboard: HashMap<u16, u32>,
    pub window: *mut c_void,
    pub renderer: *mut c_void,
}

impl Observer {
    pub fn init(
        sdl3: &mut SDL3,
        playable: Collider,
        scenes: HashMap<u64, Scene>,
        size: [f32; 2],
        iflags: u32,
        wflags: u64
    ) -> Self {
        // Инициализируем библиотеку SDL3
        sdl3_init(sdl3, iflags);
        // Инициализируем SDL3_ttf для шрифтов
        sdl3_ttf_init(sdl3);
        // Создаем очередь событий и клавиатуру
        let events: HashSet<u32> = HashSet::new();
        let keyboard: HashMap<u16, u32> = HashMap::new();
        // Создаем окно и рендер
        let window = sdl3_create_window(sdl3, "Movement Test App", size[0] as u32, size[1] as u32, wflags);
        if window.is_null(){sdl3_quit(sdl3); panic!("window pointer is null!");}
        let renderer = sdl3_create_renderer(sdl3, window, "");
        if renderer.is_null(){sdl3_destroy_window(sdl3, window); sdl3_quit(sdl3); panic!("renderer pointer is null!");}
        // Возвращаем объект "наблюдателя"
        Observer {
            playable: playable,
            scenes: scenes,
            current_scene: 0,
            size: size,
            events: events,
            keyboard: keyboard,
            window: window,
            renderer: renderer
        }
    }
    pub fn destroy(&self, sdl3: &mut SDL3){
        // Уничтожаем текущий рендер и окно
        sdl3_destroy_renderer(sdl3, self.renderer);
        sdl3_destroy_window(sdl3, self.window);
        // Закрвыаем библиотеку SDL3_ttf
        sdl3_ttf_quit(sdl3);
        println!("SDL3_ttf ended!");
        // Закрвыаем библиотеку SDL3
        sdl3_quit(sdl3);
        println!("SDL3 ended!");
    }
}
