use std::{
    fmt::{Debug,Formatter,Result},
    mem::ManuallyDrop,
    ptr::drop_in_place,
    os::raw::c_char,
    ffi::c_void
};
// SDL3 EVENT UNION--------------------------------------------------------------------------------
#[repr(C)]
pub union SDL_Event {
    pub type_: u32,
    pub common: ManuallyDrop<SDL_CommonEvent>,
    pub display: ManuallyDrop<SDL_DisplayEvent>,
    pub window: ManuallyDrop<SDL_WindowEvent>,
    pub kdevice: ManuallyDrop<SDL_KeyboardDeviceEvent>,
    pub key: ManuallyDrop<SDL_KeyboardEvent>,
    pub edit: ManuallyDrop<SDL_TextEditingEvent>,
    pub edit_candidates: ManuallyDrop<SDL_TextEditingCandidatesEvent>,
    pub text: ManuallyDrop<SDL_TextInputEvent>,
    pub mdevice: ManuallyDrop<SDL_MouseDeviceEvent>,
    pub motion: ManuallyDrop<SDL_MouseMotionEvent>,
    pub button: ManuallyDrop<SDL_MouseButtonEvent>,
    pub wheel: ManuallyDrop<SDL_MouseWheelEvent>,
    pub jdevice: ManuallyDrop<SDL_JoyDeviceEvent>,
    pub jaxis: ManuallyDrop<SDL_JoyAxisEvent>,
    pub jball: ManuallyDrop<SDL_JoyBallEvent>,
    pub jhat: ManuallyDrop<SDL_JoyHatEvent>,
    pub jbutton: ManuallyDrop<SDL_JoyButtonEvent>,
    pub jbattery: ManuallyDrop<SDL_JoyBatteryEvent>,
    pub gdevice: ManuallyDrop<SDL_GamepadDeviceEvent>,
    pub gaxis: ManuallyDrop<SDL_GamepadAxisEvent>,
    pub gbutton: ManuallyDrop<SDL_GamepadButtonEvent>,
    pub gtouchpad: ManuallyDrop<SDL_GamepadTouchpadEvent>,
    pub gsensor: ManuallyDrop<SDL_GamepadSensorEvent>,
    pub adevice: ManuallyDrop<SDL_AudioDeviceEvent>,
    pub cdevice: ManuallyDrop<SDL_CameraDeviceEvent>,
    pub sensor: ManuallyDrop<SDL_SensorEvent>,
    pub quit: ManuallyDrop<SDL_QuitEvent>,
    pub user: ManuallyDrop<SDL_UserEvent>,
    pub tfinger: ManuallyDrop<SDL_TouchFingerEvent>,
    pub pproximity: ManuallyDrop<SDL_PenProximityEvent>,
    pub ptouch: ManuallyDrop<SDL_PenTouchEvent>,
    pub pmotion: ManuallyDrop<SDL_PenMotionEvent>,
    pub pbutton: ManuallyDrop<SDL_PenButtonEvent>,
    pub paxis: ManuallyDrop<SDL_PenAxisEvent>,
    pub render: ManuallyDrop<SDL_RenderEvent>,
    pub drop: ManuallyDrop<SDL_DropEvent>,
    pub clipboard: ManuallyDrop<SDL_ClipboardEvent>,
    pub padding: [u8; 128],
}

impl Debug for SDL_Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        unsafe {
            let common = &self.common.type_;
            let display = &self.display.displayID;
            let window = &self.window.windowID;
            let kdevice = &self.kdevice.which;
            let key = &self.key.key;
            let mod_ = &self.key.mod_;
            let mdevice = &self.mdevice.which;
            let motion = &self.motion.state;
            let button = &self.button.button;
            let wheel = &self.wheel.direction;
            let quit = &self.quit.type_;
            let user = &self.user.code;
            write!(
                f,
                "SDL_Event {{ \ntype_: {},\ncommon: {},\ndisplay: {},\nwindow: {},\nkdevice: {},\nkey: {}+{},\nmdevice: {}\nmotion: {},\nbutton: {},\nwheel: {}, \nquit: {},\nuser: {} }}\n",
                self.type_, *common, *display, *window, *kdevice, *key, *mod_, *mdevice, *motion, *button, *wheel, *quit, *user
            )
        }
    }
}

impl SDL_Event{
    pub fn drop_fields(&mut self) {
        unsafe {
            drop_in_place(&mut self.common);
            drop_in_place(&mut self.display);
            drop_in_place(&mut self.window);
            drop_in_place(&mut self.kdevice);
            drop_in_place(&mut self.key);
            drop_in_place(&mut self.edit);
            drop_in_place(&mut self.edit_candidates);
            drop_in_place(&mut self.text);
            drop_in_place(&mut self.mdevice);
            drop_in_place(&mut self.motion);
            drop_in_place(&mut self.button);
            drop_in_place(&mut self.wheel);
            drop_in_place(&mut self.jdevice);
            drop_in_place(&mut self.jaxis);
            drop_in_place(&mut self.jball);
            drop_in_place(&mut self.jhat);
            drop_in_place(&mut self.jbutton);
            drop_in_place(&mut self.jbattery);
            drop_in_place(&mut self.gdevice);
            drop_in_place(&mut self.gaxis);
            drop_in_place(&mut self.gbutton);
            drop_in_place(&mut self.gtouchpad);
            drop_in_place(&mut self.gsensor);
            drop_in_place(&mut self.adevice);
            drop_in_place(&mut self.cdevice);
            drop_in_place(&mut self.sensor);
            drop_in_place(&mut self.quit);
            drop_in_place(&mut self.user);
            drop_in_place(&mut self.tfinger);
            drop_in_place(&mut self.pproximity);
            drop_in_place(&mut self.ptouch);
            drop_in_place(&mut self.pmotion);
            drop_in_place(&mut self.pbutton);
            drop_in_place(&mut self.paxis);
            drop_in_place(&mut self.render);
            drop_in_place(&mut self.drop);
            drop_in_place(&mut self.clipboard);
        }
    }
}
// ------------------------------------------------------------------------------------------------

// SDL3 CommonEvent STRUCT-------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_CommonEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
}
// ------------------------------------------------------------------------------------------------

// SDL3 DisplayEvent STRUCT------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_DisplayEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub displayID: u32,
    pub data1: i32,
    pub data2: i32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 WindowEvent STRUCT-------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_WindowEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub data1: i32,
    pub data2: i32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_KeyboardDeviceEvent STRUCT-------------------------------------------------------------
#[repr(C)]
pub struct SDL_KeyboardDeviceEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_KeyboardEvent STRUCT-------------------------------------------------------------------
#[repr(C)]
pub struct SDL_KeyboardEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub which: u32,
    pub scancode: u32,
    pub key: u32,
    pub mod_: u16,
    pub raw: u16,
    pub down: bool,
    pub repeat: bool,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_TextEditingEvent STRUCT----------------------------------------------------------------
#[repr(C)]
pub struct SDL_TextEditingEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub text: *const c_char,
    pub start: i32,
    pub length: i32,
}
//-------------------------------------------------------------------------------------------------

// SDL3 SDL_TextEditingCandidatesEvent
#[repr(C)]
pub struct SDL_TextEditingCandidatesEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub candidates: *const c_char,
    pub num_candidates: i32,
    pub selected_candidate: i32,
    pub horizontal: bool,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_TextInputEvent-------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_TextInputEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub text: *const c_char,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_MouseDeviceEvent-----------------------------------------------------------------------
#[repr(C)]
pub struct SDL_MouseDeviceEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_MouseMotionEvent-----------------------------------------------------------------------
#[repr(C)]
pub struct SDL_MouseMotionEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub which: u32,
    pub state: u32,
    pub x: f32,
    pub y: f32,
    pub xrel: f32,
    pub yrel: f32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_MouseButtonEvent-----------------------------------------------------------------------
#[repr(C)]
pub struct SDL_MouseButtonEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub which: u32,
    pub button: u8,
    pub down: bool,
    pub clicks: u8,
    pub padding: u8,
    pub x: f32,
    pub y: f32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_MouseWheelEvent------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_MouseWheelEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub which: u32,
    pub x: f32,
    pub y: f32,
    pub direction: u32,
    pub mouse_x: f32,
    pub mouse_y: f32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_JoyDeviceEvent-------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_JoyDeviceEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_JoyAxisEvent---------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_JoyAxisEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
    pub axis: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub value: i16,
    pub padding4: u16,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_JoyBallEvent---------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_JoyBallEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
    pub ball: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub xrel: i16,
    pub yrel: i16,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_JoyHatEvent----------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_JoyHatEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
    pub hat: u8,
    pub value: u8,
    pub padding1: u8,
    pub padding2: u8,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_JoyButtonEvent-------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_JoyButtonEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
    pub button: u8,
    pub down: bool,
    pub padding1: u8,
    pub padding2: u8,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_JoyBatteryEvent------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_JoyBatteryEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
    pub state: u32,
    pub percent: i32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_GamepadDeviceEvent---------------------------------------------------------------------
#[repr(C)]
pub struct SDL_GamepadDeviceEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_GamepadAxisEvent-----------------------------------------------------------------------
#[repr(C)]
pub struct SDL_GamepadAxisEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
    pub axis: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub value: i16,
    pub padding4: u16,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_GamepadButtonEvent---------------------------------------------------------------------
#[repr(C)]
pub struct SDL_GamepadButtonEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u32,
    pub which: u32,
    pub button: u8,
    pub down: bool,
    pub padding1: u8,
    pub padding2: u8,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_GamepadTouchpadEvent-------------------------------------------------------------------
#[repr(C)]
pub struct SDL_GamepadTouchpadEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
    pub touchpad: i32,
    pub finger: i32,
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_GamepadSensorEvent---------------------------------------------------------------------
#[repr(C)]
pub struct SDL_GamepadSensorEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
    pub sensor: i32,
    pub data: [f32; 3],
    pub sensor_timestamp: u64,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_AudioDeviceEvent-----------------------------------------------------------------------
#[repr(C)]
pub struct SDL_AudioDeviceEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
    pub recording: bool,
    pub padding1: u32,
    pub padding2: u32,
    pub padding3: u32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_CameraDeviceEvent----------------------------------------------------------------------
#[repr(C)]
pub struct SDL_CameraDeviceEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_SensorEvent----------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_SensorEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: u32,
    pub data: [f32; 6],
    pub sensor_timestamp: u64,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_QuitEvent------------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_QuitEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_UserEvent------------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_UserEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub code: i32,
    pub data1: *mut c_void,
    pub data2: *mut c_void,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_TouchFingerEvent-----------------------------------------------------------------------
#[repr(C)]
pub struct SDL_TouchFingerEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub touchID: u32,
    pub fingerID: u32,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub pressure: f32,
    pub windowID: u32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_PenProximityEvent----------------------------------------------------------------------
#[repr(C)]
pub struct SDL_PenProximityEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub which: u32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_PenTouchEvent--------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_PenTouchEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub which: u32,
    pub pen_state: u32,
    pub x: f32,
    pub y: f32,
    pub eraser: bool,
    pub down: bool,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_PenMotionEvent-------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_PenMotionEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub which: u32,
    pub pen_state: u32,
    pub x: f32,
    pub y: f32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_PenButtonEvent-------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_PenButtonEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub which: u32,
    pub pen_state: u32,
    pub x: f32,
    pub y: f32,
    pub button: u8,
    pub down: bool,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_PenAxisEvent---------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_PenAxisEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub which: u32,
    pub pen_state: u32,
    pub x: f32,
    pub y: f32,
    pub axis: u32,
    pub value: f32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_RenderEvent----------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_RenderEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_DropEvent------------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_DropEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: u32,
    pub x: f32,
    pub y: f32,
    pub source: *const c_char,
    pub data: *const c_char,
}
// ------------------------------------------------------------------------------------------------

// SDL3 SDL_ClipboardEvent-------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_ClipboardEvent{
    pub type_: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub owner: bool,
    pub num_mime_types: i32,
    pub mime_types: *const *const c_char,
}
// ------------------------------------------------------------------------------------------------

// SDL3 RECT STRUCT--------------------------------------------------------------------------------
#[repr(C)]
pub struct SDL_FRect{
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
// ------------------------------------------------------------------------------------------------
