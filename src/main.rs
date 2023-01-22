#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Flex, Level, Output, Pull, Speed};
use embassy_time::{block_for, Duration, Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

static STOP_CM: u8 = 40; // When red light turns on
static WARN_CM: u8 = 50; // When yellow light turns on, green if greater

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());

    let mut led_red = Output::new(peripherals.PA2, Level::High, Speed::Low);
    let mut led_yellow = Output::new(peripherals.PB0, Level::High, Speed::Low);
    let mut led_green = Output::new(peripherals.PC0, Level::High, Speed::Low);

    let mut laser = Flex::new(peripherals.PA3);

    Timer::after(Duration::from_millis(100)).await;

    led_green.set_low();
    led_yellow.set_low();
    led_red.set_low();

    let loop_duration_millis = 100;
    let mut distance = 0_f32;
    let mut last_change_millis = 0;

    loop {
        // Query laser for measurement, ref pg 3 of datasheet.
        // 1. Ensure set low, followed by high pulse of 2micros (min), the low.
        // 2. Wait for laser pulse, then measure the width.
        laser.set_as_output(Speed::Medium);
        laser.set_low();
        block_for(Duration::from_micros(5));

        laser.set_high();
        block_for(Duration::from_micros(10));
        laser.set_low();

        laser.set_as_input(Pull::None);

        while laser.is_low() {}

        let inst = Instant::now();
        while laser.is_high() {}
        let duration = inst.elapsed();

        let dist = ((duration.as_micros() as f32 * 171.5) / 10_f32 / 100_f32 / 10_f32) as f32; // Ref Ping Laser datasheet pg. 4
        info!("Value: {}", dist);

        // Determine if distance is actively changing since last, more than 15cm
        let active = if num::abs(dist - distance) < 15_f32 {
            // Last change over 30seconds ago, turn off all lights.
            if last_change_millis > 30_000 {
                led_green.set_low();
                led_yellow.set_low();
                led_red.set_low();
                distance = dist;
                // Extra sleep until next check since nothing is happening.
                Timer::after(Duration::from_secs(5)).await;
                false

            // Last change less than 10s ago, increment no change counter
            } else {
                last_change_millis += loop_duration_millis;
                true
            }
        } else {
            last_change_millis = 0;
            distance = dist;
            true
        };

        if active {
            if distance <= STOP_CM as _ {
                led_green.set_low();
                led_yellow.set_low();
                led_red.set_high();
            } else if distance <= WARN_CM as _ {
                led_green.set_low();
                led_yellow.set_high();
                led_red.set_low();
            } else {
                led_green.set_high();
                led_yellow.set_low();
                led_red.set_low();
            }
        }

        Timer::after(Duration::from_millis(loop_duration_millis)).await;
    }
}
