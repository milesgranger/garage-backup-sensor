#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

#[allow(unused_imports)]
use defmt::*;

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Flex, Input, Level, Output, Pull, Speed};
use embassy_time::{block_for, Duration, Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

static STOP_CM: f32 = 40.; // When red light turns on
static WARN_CM: f32 = 90.; // When yellow light turns on, green if greater

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());

    let mut led_red = Output::new(peripherals.PA2, Level::High, Speed::Low);
    let mut led_yellow = Output::new(peripherals.PB0, Level::High, Speed::Low);
    let mut led_green = Output::new(peripherals.PC0, Level::High, Speed::Low);
    let button = Input::new(peripherals.PC13, Pull::None);

    let mut laser = Flex::new(peripherals.PA3);

    Timer::after(Duration::from_millis(100)).await;

    led_green.set_low();
    led_yellow.set_low();
    led_red.set_low();

    let interval = Duration::from_millis(250);
    let mut distance_prev = 0f32;
    let mut last_significant_change = Instant::now();
    let mut stop_cm = STOP_CM;
    let mut warn_cm = WARN_CM;

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

        // Invalid/Inacurate measurements: Ref pg 3 of datasheet.
        // We need an valid measurement.
        if duration.as_millis() > 12 {
            led_green.set_low();
            led_yellow.set_low();
            led_red.set_low();
            Timer::after(Duration::from_secs(1)).await;
            continue;
        }

        // Ref Ping Laser datasheet pg. 4
        let distance_curr = (duration.as_micros() as f32 * 171.5) / 10000.;

        // Check if we're setting the distance
        let button_pressed_start = Instant::now();
        while button.is_high() {
            if button_pressed_start.elapsed() >= Duration::from_secs(5) {
                stop_cm = distance_curr;
                warn_cm = stop_cm + 50.;
                for _ in 0..3 {
                    led_green.set_high();
                    led_yellow.set_high();
                    led_red.set_high();
                    Timer::after(Duration::from_millis(250)).await;
                    led_green.set_low();
                    led_yellow.set_low();
                    led_red.set_low();
                    Timer::after(Duration::from_millis(250)).await;
                }
                break;
            }
            Timer::after(Duration::from_millis(250)).await;
        }

        // Determine if distance is actively changing since last, more than 20cm
        let mut active = true;
        if num::abs(distance_prev - distance_curr) < 20. {
            // Last change over 30s ago, turn off all lights.
            if last_significant_change.elapsed().as_secs() > 30 {
                led_green.set_low();
                led_yellow.set_low();
                led_red.set_low();
                // Extra sleep until next check since nothing is happening.
                Timer::after(interval).await;
                active = false;
            }
        } else {
            last_significant_change = Instant::now();
        };

        if active {
            if distance_curr <= stop_cm {
                led_green.set_low();
                led_yellow.set_low();
                led_red.set_high();
            } else if distance_curr <= warn_cm {
                led_green.set_low();
                led_yellow.set_high();
                led_red.set_low();
            } else {
                led_green.set_high();
                led_yellow.set_low();
                led_red.set_low();
            }
            distance_prev = distance_curr;
        }
        Timer::after(interval).await;
    }
}
