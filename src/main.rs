#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

#[allow(unused_imports)]
use defmt::*;

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Flex, Input, Level, Output, Pull, Speed};
use embassy_time::{block_for, Duration, Instant, Timer};
use stm_tof::{ffi::VL53LX_DEV, Platform, Result, VL53L4CX_Device};
use {defmt_rtt as _, panic_probe as _};

static STOP_CM: f32 = 40.; // When red light turns  on
static WARN_CM: f32 = 90.; // When yellow light turns on, green if greater

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());

    let mut led_green = Output::new(peripherals.PB14, Level::High, Speed::Low);
    let mut platform = Platform::new();
    platform.set_read_byte(|| crate::unimplemented!("read_byte not implemented"));
    platform.set_wait_millis(|n| block_for(Duration::from_millis(n as _)));

    let mut distance_sensor = VL53L4CX_Device::new(0x11, platform);

    distance_sensor.init().unwrap();
    loop {
        led_green.set_high();
        Timer::after(Duration::from_millis(250)).await;
        led_green.set_low();
        Timer::after(Duration::from_millis(250)).await;
    }
}
