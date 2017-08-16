extern crate rpi_ws281x_sys as ffi;

use std::ptr;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StripType {
    // 4 color R, G, B and W ordering,
    SK6812StripRGBW = 0x18100800,
    SK6812StripRBGW = 0x18100008,
    SK6812StripGRBW = 0x18081000,
    SK6812StripGBRW = 0x18080010,
    SK6812StripBRGW = 0x18001008,
    SK6812StripBGRW = 0x18000810,
    // 3 color R, G and B ordering,
    WS2811StripRGB = 0x00100800,
    WS2811StripRBG = 0x00100008,
    WS2811StripGRB = 0x00081000,
    WS2811StripGBR = 0x00080010,
    WS2811StripBRG = 0x00001008,
    WS2811StripBGR = 0x00000810
}

pub struct WS281x {
    strip: ffi::ws2811_t,
    length: u32
}

impl WS281x {
    pub fn new(port: u8, count: u32, stype: StripType) -> Result<WS281x, i32> {
        unsafe {
            let mut dev: ffi::ws2811_t = ffi::ws2811_t {
                freq: 800000,
                dmanum: 5,
                channel:
                [ffi::ws2811_channel_t { gpionum: port as i32, count: count as i32, invert: 0, brightness: 255, strip_type: stype as u32, leds: ptr::null_mut(), wshift: 0, rshift: 0, gshift: 0, bshift: 0 },
                 ffi::ws2811_channel_t { gpionum: 0,  count: 0, invert: 0, brightness: 0,   strip_type: 0, leds: ptr::null_mut(), wshift: 0, rshift: 0, gshift: 0, bshift: 0 }],
                device: ptr::null_mut(),
                rpi_hw: ptr::null_mut(),
                render_wait_time: 0
            };

            let ret = ffi::ws2811_init(&mut dev as *mut ffi::ws2811_t);

            match ret {
                ffi::ws2811_return_t::WS2811_SUCCESS => Ok(WS281x {strip: dev, length: count }),
                e => Err(e as i32)
            }
        }
    }

    pub fn display_color(&mut self, r: u8, g: u8, b: u8) {
        let c: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
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

    pub fn set_pixel(&mut self, p: u32, r: u8, g: u8, b: u8) {
        let c: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        unsafe {
            std::ptr::write(self.strip.channel[0].leds.offset(p as isize), c);
        }
    }

    pub fn render(&mut self) {
        unsafe { ffi::ws2811_render(&mut self.strip as *mut ffi::ws2811_t); }
    }

    pub fn wait(&mut self) {
        unsafe { ffi::ws2811_wait(&mut self.strip as *mut ffi::ws2811_t); }
    }
}

impl Drop for WS281x {
    fn drop(&mut self) {
        unsafe {
            ffi::ws2811_fini(&mut self.strip as *mut ffi::ws2811_t);
        }
    }
}

unsafe impl Sync for rpi_ws281x::WS281x { }