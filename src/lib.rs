extern crate rpi_ws281x_sys as ffi;

use std::ptr;

pub struct WS281x {
    strip: ffi::ws2811_t,
    length: u32
}

impl WS281x {
    pub fn new() -> Result<WS281x, i32> {
        unsafe {
            let mut dev: ffi::ws2811_t = ffi::ws2811_t {
                freq: 800000,
                dmanum: 5,
                channel:
                [ffi::ws2811_channel_t { gpionum: 21, count: 20, invert: 0, brightness: 255, strip_type: ffi::WS2811_STRIP_GRB, leds: ptr::null_mut(), wshift: 0, rshift: 0, gshift: 0, bshift: 0 },
                 ffi::ws2811_channel_t { gpionum: 0,  count: 0, invert: 0, brightness: 0,   strip_type: 0, leds: ptr::null_mut(), wshift: 0, rshift: 0, gshift: 0, bshift: 0 }],
                device: ptr::null_mut(),
                rpi_hw: ptr::null_mut(),
                render_wait_time: 0
            };

            let ret = ffi::ws2811_init(&mut dev as *mut ffi::ws2811_t);

            match ret {
                ffi::ws2811_return_t::WS2811_SUCCESS => Ok(WS281x {strip: dev, length: 20 }),
                e => Err(e as i32)
            }
        }
    }

    pub fn set_color(&mut self, c: u32) {
        for i in 0..self.length {
            unsafe {
                std::ptr::write(self.strip.channel[0].leds.offset(i as isize), c);
            }
        }

        unsafe {
            ffi::ws2811_render(&mut self.strip as *mut ffi::ws2811_t);
            ffi::ws2811_wait(&mut self.strip as *mut ffi::ws2811_t);
        }
    }
}

impl Drop for WS281x {
    fn drop(&mut self) {
        unsafe {
            ffi::ws2811_fini(&mut self.strip as *mut ffi::ws2811_t);
        }
    }
}