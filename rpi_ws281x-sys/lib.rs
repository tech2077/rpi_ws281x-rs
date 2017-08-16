use std::os::raw::*;

pub const RPI_PWM_CHANNELS: c_uint = 2;
pub const WS2811_TARGET_FREQ: c_uint = 800000;
pub const SK6812_STRIP_RGBW: c_uint = 403703808;
pub const SK6812_STRIP_RBGW: c_uint = 403701768;
pub const SK6812_STRIP_GRBW: c_uint = 403181568;
pub const SK6812_STRIP_GBRW: c_uint = 403177488;
pub const SK6812_STRIP_BRGW: c_uint = 402657288;
pub const SK6812_STRIP_BGRW: c_uint = 402655248;
pub const SK6812_SHIFT_WMASK: c_uint = 4026531840;
pub const WS2811_STRIP_RGB: c_uint = 1050624;
pub const WS2811_STRIP_RBG: c_uint = 1048584;
pub const WS2811_STRIP_GRB: c_uint = 528384;
pub const WS2811_STRIP_GBR: c_uint = 524304;
pub const WS2811_STRIP_BRG: c_uint = 4104;
pub const WS2811_STRIP_BGR: c_uint = 2064;
pub const WS2812_STRIP: c_uint = 528384;
pub const SK6812_STRIP: c_uint = 528384;
pub const SK6812W_STRIP: c_uint = 403181568;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ws2811_return_t {
    WS2811_SUCCESS = 0,
    WS2811_ERROR_GENERIC = -1,
    WS2811_ERROR_OUT_OF_MEMORY = -2,
    WS2811_ERROR_HW_NOT_SUPPORTED = -3,
    WS2811_ERROR_MEM_LOCK = -4,
    WS2811_ERROR_MMAP = -5,
    WS2811_ERROR_MAP_REGISTERS = -6,
    WS2811_ERROR_GPIO_INIT = -7,
    WS2811_ERROR_PWM_SETUP = -8,
    WS2811_ERROR_MAILBOX_DEVICE = -9,
    WS2811_ERROR_DMA = -10,
    WS2811_ERROR_ILLEGAL_GPIO = -11,
    WS2811_ERROR_PCM_SETUP = -12,
    WS2811_ERROR_SPI_SETUP = -13,
    WS2811_ERROR_SPI_TRANSFER = -14,
}

pub enum ws2811_device {}

pub type ws2811_led_t = u32;


#[repr(C)]
pub struct rpi_hw_t {
    pub type_: u32,
    pub hwver: u32,
    pub periph_base: u32,
    pub videocore_base: u32,
    pub desc: *mut c_char,
}


#[repr(C)]
pub struct ws2811_channel_t {
    pub gpionum: c_int,
    pub invert: c_int,
    pub count: c_int,
    pub strip_type: c_uint,
    pub leds: *mut ws2811_led_t,
    pub brightness: u8,
    pub wshift: u8,
    pub rshift: u8,
    pub gshift: u8,
    pub bshift: u8,
}

#[repr(C)]
pub struct ws2811_t {
    pub render_wait_time: u64,
    pub device: *mut ws2811_device,
    pub rpi_hw: *const rpi_hw_t,
    pub freq: u32,
    pub dmanum: c_int,
    pub channel: [ws2811_channel_t; 2usize],
}

unsafe impl Sync for rpi_hw_t { }
unsafe impl Sync for ws2811_device { }
unsafe impl Sync for *mut u32 { }

extern "C" {
    pub fn ws2811_init(ws2811: *mut ws2811_t) -> ws2811_return_t;
    pub fn ws2811_fini(ws2811: *mut ws2811_t);
    pub fn ws2811_render(ws2811: *mut ws2811_t) -> ws2811_return_t;
    pub fn ws2811_wait(ws2811: *mut ws2811_t) -> ws2811_return_t;
    pub fn ws2811_get_return_t_str(state: ws2811_return_t) -> *const c_char;
}