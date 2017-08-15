extern crate rpi_ws281x_sys as ffi;

pub struct WS281x {
    strip: ffi::ws2811_t,
    length: u32
}

impl WS281x {
    pub fn new() -> Result<WS281x, i32> {
    }
}