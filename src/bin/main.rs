#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![deny(clippy::large_stack_frames)]

use as5600::As5600;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    i2c::master::{Config as I2cConfig, I2c},
    main,
    time::{Duration, Instant},
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

fn delay_ms(ms: u64) {
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(ms) {}
}

#[allow(clippy::large_stack_frames)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Change GPIO pins to match your wiring
    let i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
        .unwrap()
        .with_sda(peripherals.GPIO6)
        .with_scl(peripherals.GPIO7);

    // Bridge ehal 0.2 → 1.0
    let mut sensor = As5600::new(i2c);

    let cfg = sensor.config().unwrap();
    println!("{:?}", cfg);

    delay_ms(2000);

    let status = sensor.magnet_status().unwrap();
    let agc    = sensor.automatic_gain_control().unwrap();
    let mag    = sensor.magnitude().unwrap();
    let zmco   = sensor.zmco().unwrap();

    println!("{:?}", status);
    println!("{:?}", agc);
    println!("{:?}", mag);
    println!("{:?}", zmco);

    delay_ms(2000);

    loop {
        let sensor_angle = sensor.angle().unwrap();
        let angle = (sensor_angle as f32 / 4096.0) * 360.0;
        println!("{:?}", angle);
        delay_ms(100);
    }
}
