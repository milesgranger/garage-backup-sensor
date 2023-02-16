#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::ffi::c_void;

#[allow(unused_imports)]
use defmt::*;
use embassy_executor::Spawner;
#[allow(unused_imports)]
use embassy_stm32::{
    gpio::{Flex, Input, Level, Output, Pin, Pull, Speed},
    peripherals::PB14,
};
use embassy_time::{block_for, Duration, Instant, Timer};
use spin::Mutex;
use stm_tof::{ffi::VL53LX_DEV, Platform, Result, VL53L4CX_Device};
use {defmt_rtt as _, panic_probe as _};

static STOP_CM: f32 = 40.; // When red light turns  on
static WARN_CM: f32 = 90.; // When yellow light turns on, green if greater

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());

    let mut led_green = Mutex::new(Output::new(peripherals.PB14, Level::High, Speed::Low));

    let ctx = &mut led_green as *mut _ as *mut c_void;

    let mut platform = Platform::new(ctx);
    platform.set_read_byte(|ctx| crate::unimplemented!("read_byte not implemented"));
    platform.set_wait_millis(|ctx, n| {
        let led_green = unsafe { &mut *(ctx.0 as *mut Mutex<Output<PB14>>) };
        led_green.lock().set_high();
        block_for(Duration::from_millis(n as _));
    });

    let mut distance_sensor = VL53L4CX_Device::new(0x11, platform);

    distance_sensor.init().unwrap();
    loop {
        led_green.lock().set_high();
        Timer::after(Duration::from_millis(250)).await;
        led_green.lock().set_low();
        Timer::after(Duration::from_millis(250)).await;
    }
}
