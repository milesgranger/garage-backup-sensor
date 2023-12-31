#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::ffi::c_char;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

extern "C" {
    // fn acc_hal_rss_integration_get_implementation();
    fn acc_version_get() -> *const c_char;
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World, calling sensor count!");
    let version_ptr = unsafe { acc_version_get() };

    let mut led = Output::new(p.PB14, Level::High, Speed::Low);

    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}
