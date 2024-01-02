#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::ffi::CStr;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use garage_backup_sensor::a121;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");
    print_version();

    let mut led = Output::new(p.PB14, Level::High, Speed::Low);

    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}

fn print_version() {
    let version = unsafe { CStr::from_ptr(a121::acc_version::acc_version_get() as _) };
    info!("Version: {}", version.to_str().unwrap());
}
