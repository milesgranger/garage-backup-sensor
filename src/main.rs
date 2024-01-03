#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

extern crate alloc;

use alloc::vec::Vec;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use embedded_alloc::Heap;

use garage_backup_sensor::a121;

#[global_allocator]
static HEAP: Heap = Heap::empty();

// struct DistanceDetectorResources {
//     sensor: Sensor,
// }

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World, initializing allocator!");

    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    a121::version::version_get(|v| info!("Version: {}", v.to_str().unwrap()));
    let config = a121::detector_distance::Config::new(
        a121::detector_distance::DistancePresetConfig::Balanced,
    );
    let handle = a121::detector_distance::Handle::new(&config);
    let mut buffer = Vec::with_capacity(handle.get_buffer_size() as usize);
    for _ in 0..buffer.capacity() {
        buffer.push(0u8);
    }
    let mut capacity_buffer = Vec::with_capacity(handle.get_calibration_buffer_size() as usize);
    for _ in 0..capacity_buffer.capacity() {
        capacity_buffer.push(0u8);
    }

    let mut led = Output::new(p.PB14, Level::High, Speed::Low);

    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}
