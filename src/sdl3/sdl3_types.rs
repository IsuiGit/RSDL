use crate::sdl3::sdl3_structs::*;
use std::{ffi::{c_void, c_char}};
// SDL3 types register

// SDL3_TTF Types ---------------------------------------------------------------------------------
pub type SDLF_Init = unsafe extern "C" fn() -> bool;
pub type SDLF_Quit = unsafe extern "C" fn();
// ------------------------------------------------------------------------------------------------

// SDL3 Event types -------------------------------------------------------------------------------
pub type SDL_PushEvent = unsafe extern "C" fn(*mut SDL_Event) -> bool;
// ------------------------------------------------------------------------------------------------

// SDL3 Sys types ---------------------------------------------------------------------------------
// SDL3_Init type
pub type SDL_Init = unsafe extern "C" fn(u32) -> bool;
// SDL3_Quit type
pub type SDL_Quit = unsafe extern "C" fn();
// SDL3_GetError type
pub type SDL_GetError = unsafe extern "C" fn() -> *const c_char;
// SDL3_Delay type
pub type SDL_Delay = unsafe extern "C" fn(u32);
// SDL3_PollEvent
pub type SDL_PollEvent = unsafe extern "C" fn(*mut SDL_Event) -> bool;
// ------------------------------------------------------------------------------------------------

// SDL3 Create types ------------------------------------------------------------------------------
// SDL3_CreateWindow type
pub type SDL_CreateWindow = unsafe extern "C" fn(*const i8, u32, u32, u64) -> *mut c_void;
// SDL3_DestroyWindow type
pub type SDL_DestroyWindow = unsafe extern "C" fn(*mut c_void);
// SDL3_CreateRenderer
pub type SDL_CreateRenderer = unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void;
// SDL3_DestroyRenderer
pub type SDL_DestroyRenderer = unsafe extern "C" fn(*mut c_void);
// ------------------------------------------------------------------------------------------------

// SDL3 Get types ---------------------------------------------------------------------------------
// SDL3_GetWindowSize
pub type SDL_GetWindowSize = unsafe extern "C" fn(*mut c_void, *mut i32, *mut i32) -> bool;
// SDL3_GetWindowSurface
pub type SDL_GetWindowSurface = unsafe extern "C" fn(*mut c_void) -> *mut c_void;
// ------------------------------------------------------------------------------------------------

// SDL3 Render types ------------------------------------------------------------------------------
// SDL3_SetRenderDrawColor
pub type SDL_SetRenderDrawColor = unsafe extern "C" fn(*mut c_void, u8, u8, u8, u8) -> bool;
// SDL3_RenderClear
pub type SDL_RenderClear = unsafe extern "C" fn(*mut c_void) -> bool;
// SDL3_RenderPresent
pub type SDL_RenderPresent = unsafe extern "C" fn(*mut c_void) -> bool;
// SDL3_RenderRect
pub type SDL_RenderRect = unsafe extern "C" fn(*mut c_void, *const SDL_FRect) -> bool;
// SDL3 RenderFillRect
pub type SDL_RenderFillRect = unsafe extern "C" fn(*mut c_void, *const SDL_FRect) -> bool;
// SDL3 RenderRects
pub type SDL_RenderRects = unsafe extern "C" fn(*mut c_void, *const SDL_FRect, i32) -> bool;
// SDL3_RenderFillRects
pub type SDL_RenderFillRects = unsafe extern "C" fn(*mut c_void, *const SDL_FRect, i32) -> bool;
// SDL3_CreateTextureFromSurface
pub type SDL_CreateTextureFromSurface = unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void;
// SDL3_RenderTexture
pub type SDL_RenderTexture = unsafe extern "C" fn(*mut c_void, *mut c_void, *const SDL_FRect, *const SDL_FRect) -> bool;
// ------------------------------------------------------------------------------------------------

// SDL3 TTF types ---------------------------------------------------------------------------------
// TTF_OpenFont
pub type TTF_OpenFont = unsafe extern "C" fn(*const c_char, f32) -> *mut c_void;
// TTF_CloseFont
pub type TTF_CloseFont = unsafe extern "C" fn(*mut c_void);
// TTF_RenderText_Blended
pub type TTF_RenderText_Blended = unsafe extern "C" fn(*mut c_void, *const c_char, usize, SDL_Color) -> *mut c_void;
// TTF_CreateRenderTextEngine
pub type TTF_CreateRendererTextEngine = unsafe extern "C" fn(*mut c_void) -> *mut c_void;
// TTF_CreateText
pub type TTF_CreateText = unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_char, usize) -> *mut c_void;
// TTF_DrawRenderText
pub type TTF_DrawRendererText = unsafe extern "C" fn(*mut c_void, f32, f32) -> bool;
