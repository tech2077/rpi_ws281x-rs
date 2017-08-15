extern crate gcc;

use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    let _ = Command::new("git").args(&["clone", "https://github.com/jgarff/rpi_ws281x", env!("OUT_DIR")]).status();
    gcc:Config::new()
                .file(concat!(env!("OUT_DIR"), "/rpi_ws281x/mailbox.c"))
                .file(concat!(env!("OUT_DIR"), "/rpi_ws281x/ws2811.c")),
                .file(concat!(env!("OUT_DIR"), "/rpi_ws281x/pwm.c")),
                .file(concat!(env!("OUT_DIR"), "/rpi_ws281x/pcm.c")),
                .file(concat!(env!("OUT_DIR"), "/rpi_ws281x/dma.c")),
                .file(concat!(env!("OUT_DIR"), "/rpi_ws281x/rpihw.c"))
                .include(concat!(env!("OUT_DIR"), "/rpi_ws281x"))
                .compile("libws2811.a");
}