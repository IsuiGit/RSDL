// SDL_INIT CONSTANTS START------------------------------------------------------------------------
pub const SDL_INIT_AUDIO: u32 = 0x00000010;
pub const SDL_INIT_VIDEO: u32 = 0x00000020;
pub const SDL_INIT_JOYSTICK: u32 = 0x00000200;
pub const SDL_INIT_HAPTIC: u32 = 0x00001000;
pub const SDL_INIT_GAMEPAD: u32 = 0x00002000;
pub const SDL_INIT_EVENTS: u32 = 0x00004000;
pub const SDL_INIT_SENSOR: u32 = 0x00008000;
pub const SDL_INIT_CAMERA: u32 = 0x00010000;
// SDL_INIT CONSTANTS END--------------------------------------------------------------------------

// SDL_EVENT CONSTANTS START-----------------------------------------------------------------------
pub const SDL_EVENT_QUIT: u32 = 0x100;
pub const SDL_EVENT_WINDOW_RESIZED: u32 = 0x206;
pub const SDL_EVENT_KEY_DOWN: u32 = 0x300;
pub const SDL_EVENT_KEY_UP: u32 = 0x301;
// SDL_EVENT CONSTANTS END-------------------------------------------------------------------------

// SDL_WINDOW INIT CONSTANTS----------------------------------------------------------------------------
pub const SDL_WINDOW_FULLSCREEN: u64 = 0x0000000000000001;
pub const SDL_WINDOW_OPENGL: u64 = 0x0000000000000002;
pub const SDL_WINDOW_OCCLUDED: u64 = 0x0000000000000004;
pub const SDL_WINDOW_HIDDEN: u64 = 0x0000000000000008;
pub const SDL_WINDOW_BORDERLESS: u64 = 0x0000000000000010;
pub const SDL_WINDOW_RESIZABLE: u64 = 0x0000000000000020;
pub const SDL_WINDOW_MINIMIZED: u64 = 0x0000000000000040;
pub const SDL_WINDOW_MAXIMIZED: u64 = 0x0000000000000080;
pub const SDL_WINDOW_MOUSE_GRABBED: u64 = 0x0000000000000100;
pub const SDL_WINDOW_INPUT_FOCUS: u64 = 0x0000000000000200;
pub const SDL_WINDOW_MOUSE_FOCUS: u64 = 0x0000000000000400;
pub const SDL_WINDOW_EXTERNAL: u64 = 0x0000000000000800;
pub const SDL_WINDOW_MODAL: u64 = 0x0000000000001000;
pub const SDL_WINDOW_HIGH_PIXEL_DENSITY: u64 = 0x0000000000002000;
pub const SDL_WINDOW_MOUSE_CAPTURE: u64 = 0x0000000000004000;
pub const SDL_WINDOW_MOUSE_RELATIVE_MODE: u64 = 0x0000000000008000;
pub const SDL_WINDOW_ALWAYS_ON_TOP: u64 = 0x0000000000010000;
pub const SDL_WINDOW_UTILITY: u64 = 0x0000000000020000;
pub const SDL_WINDOW_TOOLTIP: u64 = 0x0000000000040000;
pub const SDL_WINDOW_POPUP_MENU: u64 = 0x0000000000080000;
pub const SDL_WINDOW_KEYBOARD_GRABBED: u64 = 0x0000000000100000;
pub const SDL_WINDOW_VULKAN: u64 = 0x0000000010000000;
pub const SDL_WINDOW_METAL: u64 = 0x0000000020000000;
pub const SDL_WINDOW_TRANSPARENT: u64 = 0x0000000040000000;
pub const SDL_WINDOW_NOT_FOCUSABLE: u64 = 0x0000000080000000;
// ------------------------------------------------------------------------------------------------

// SDL_KEY CONSTANTS START-------------------------------------------------------------------------
pub const SDLK_ESCAPE: u32 = 0x0000001b;
pub const SDLK_SPACE: u32 = 0x00000020;
pub const SDLK_A: u32 = 0x00000061;
pub const SDLK_B: u32 = 0x00000062;
pub const SDLK_C: u32 = 0x00000063;
pub const SDLK_D: u32 = 0x00000064;
pub const SDLK_E: u32 = 0x00000065;
pub const SDLK_F: u32 = 0x00000066;
pub const SDLK_G: u32 = 0x00000067;
pub const SDLK_H: u32 = 0x00000068;
pub const SDLK_I: u32 = 0x00000069;
pub const SDLK_J: u32 = 0x0000006a;
pub const SDLK_K: u32 = 0x0000006b;
pub const SDLK_L: u32 = 0x0000006c;
pub const SDLK_M: u32 = 0x0000006d;
pub const SDLK_N: u32 = 0x0000006e;
pub const SDLK_O: u32 = 0x0000006f;
pub const SDLK_P: u32 = 0x00000070;
pub const SDLK_Q: u32 = 0x00000071;
pub const SDLK_R: u32 = 0x00000072;
pub const SDLK_S: u32 = 0x00000073;
pub const SDLK_T: u32 = 0x00000074;
pub const SDLK_U: u32 = 0x00000075;
pub const SDLK_V: u32 = 0x00000076;
pub const SDLK_W: u32 = 0x00000077;
pub const SDLK_X: u32 = 0x00000078;
pub const SDLK_Y: u32 = 0x00000079;
pub const SDLK_Z: u32 = 0x0000007a;
pub const SDLK_F1: u32 = 0x4000003a;
pub const SDLK_F2: u32 = 0x4000003b;
// ------------------------------------------------------------------------------------------------

// SDL_KEYMOD CONSTANTS START----------------------------------------------------------------------
const SDL_KMOD_PADDING: u16 = 0x1000;
pub const SDL_KMOD_LSHIFT: u16 = 0x0001 | SDL_KMOD_PADDING;
pub const SDL_KMOD_RSHIFT: u16 =  0x0002 | SDL_KMOD_PADDING;
pub const SDL_KMOD_LEVEL5: u16 =  0x0004 | SDL_KMOD_PADDING;
pub const SDL_KMOD_LCTRL: u16 = 0x0040 | SDL_KMOD_PADDING;
pub const SDL_KMOD_RCTRL: u16 = 0x0080 | SDL_KMOD_PADDING;
pub const SDL_KMOD_LALT: u16 = 0x0100 | SDL_KMOD_PADDING;
pub const SDL_KMOD_RALT: u16 = 0x0200 | SDL_KMOD_PADDING;
pub const SDL_KMOD_LGUI: u16 = 0x0400 | SDL_KMOD_PADDING;
pub const SDL_KMOD_RGUI: u16 = 0x0800 | SDL_KMOD_PADDING;
pub const SDL_KMOD_NUM: u16 = 0x1000 | SDL_KMOD_PADDING;
pub const SDL_KMOD_CAPS: u16 = 0x2000 | SDL_KMOD_PADDING;
pub const SDL_KMOD_MODE: u16 = 0x4000 | SDL_KMOD_PADDING;
pub const SDL_KMOD_SCROLL: u16 = 0x8000 | SDL_KMOD_PADDING;
// ------------------------------------------------------------------------------------------------

// SDL3 SURFACE CONSTANTS START--------------------------------------------------------------------
pub const SDL_SURFACE_PREALLOCATED: u32 = 0x00000001;
pub const SDL_SURFACE_LOCK_NEEDED: u32 = 0x00000002;
pub const SDL_SURFACE_LOCKED: u32 = 0x00000004;
pub const SDL_SURFACE_SIMD_ALIGNED: u32 = 0x00000008;
// ------------------------------------------------------------------------------------------------

// SDL3 PIXEL FORMATS START------------------------------------------------------------------------
pub const SDL_PIXELFORMAT_RGBA8888: u32 = 0x16462004;
// ------------------------------------------------------------------------------------------------

// SDL3 TEXTURE ACCESS START-----------------------------------------------------------------------
pub const SDL_TEXTUREACCESS_STATIC: u32 = 0;
pub const SDL_TEXTUREACCESS_STREAMING: u32 = 1;
pub const SDL_TEXTUREACCESS_TARGET: u32 = 2;
// ------------------------------------------------------------------------------------------------
