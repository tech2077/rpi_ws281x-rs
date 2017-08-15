extern crate gcc;

use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap().to_string();
    let _ = Command::new("git").args(&["clone", "https://github.com/jgarff/rpi_ws281x", &format!("{}{}", out_dir, "/rpi_ws281x")]).status();
    let mut cfg = gcc::Config::new();


    cfg.include(format!("{}{}", out_dir, "/rpi_ws281x"))
        .file(format!("{}{}", out_dir, "/rpi_ws281x/mailbox.c"))
        .file(format!("{}{}", out_dir, "/rpi_ws281x/ws2811.c"))
        .file(format!("{}{}", out_dir, "/rpi_ws281x/pwm.c"))
        .file(format!("{}{}", out_dir, "/rpi_ws281x/pcm.c"))
        .file(format!("{}{}", out_dir, "/rpi_ws281x/dma.c"))
        .file(format!("{}{}", out_dir, "/rpi_ws281x/rpihw.c"))
        .compile("libws2811.a");
}