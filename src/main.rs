#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(unused_imports)]

use core::ffi::c_void;

#[allow(unused_imports)]
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{dma::NoDma, peripherals::I2C1, time::Hertz};
#[allow(unused_imports)]
use embassy_stm32::{
    gpio::{Flex, Input, Level, Output, Pin, Pull, Speed},
    i2c::{Config as I2cConfig, I2c},
    interrupt,
    peripherals::PB14,
};
use embassy_time::{block_for, Duration, Instant, Timer};
use spin::Mutex;
use stm_tof::{ffi::VL53LX_DEV, platform::Platform, DistanceMode, Result, VL53L4CX_Device};
use {defmt_rtt as _, panic_probe as _};

static ADDRESS: u8 = 0x29;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());
    Timer::after(Duration::from_millis(500)).await;

    let irq = interrupt::take!(I2C1_EV);
    let mut i2c = I2c::new(
        peripherals.I2C1,
        peripherals.PB8,
        peripherals.PB9,
        irq,
        NoDma,
        NoDma,
        Hertz(100_000),
        Default::default(),
    );

    let mut distance_sensor_xshut = Output::new(peripherals.PA4, Level::Low, Speed::Low);
    distance_sensor_xshut.set_high();
    Timer::after(Duration::from_millis(500)).await;

    // let mut data = [0u8; 1];
    // unwrap!(i2c.blocking_write_read(ADDRESS, &[0x0F], &mut data));
    // info!("Whoami: {}", data[0]);

    let mut led_green = Output::new(peripherals.PB14, Level::High, Speed::Low);

    let ctx = &mut i2c as *mut _ as *mut c_void;

    let mut platform = Platform::new(ctx);
    platform.set_read_byte(|ctx, index| {
        let i2c = unsafe { &mut *(ctx.0 as *mut I2c<I2C1>) };
        let mut data = [0u8; 1];
        unwrap!(i2c.blocking_write_read(ADDRESS, &index.to_be_bytes(), &mut data));
        Ok(data[0])
    });
    platform.set_read_word(|ctx, index| {
        let i2c = unsafe { &mut *(ctx.0 as *mut I2c<I2C1>) };
        let mut data = [0u8; 2];
        unwrap!(i2c.blocking_write_read(ADDRESS, &index.to_be_bytes(), &mut data));
        let word = u16::from_le_bytes(data);
        Ok(word)
    });
    platform.set_write_byte(|ctx, index, byte| {
        let i2c = unsafe { &mut *(ctx.0 as *mut I2c<I2C1>) };
        let mut data = [0u8; 3];
        data[0..2].copy_from_slice(&index.to_be_bytes());
        data[2] = byte;
        unwrap!(i2c.blocking_write(ADDRESS, &data));
        Ok(())
    });
    platform.set_write_word(|ctx, index, word| {
        let i2c = unsafe { &mut *(ctx.0 as *mut I2c<I2C1>) };
        let mut bytes = [0u8; 4];
        bytes[0..2].copy_from_slice(&index.to_be_bytes());
        bytes[2..].copy_from_slice(&word.to_be_bytes());
        info!("Writting word");
        unwrap!(i2c.blocking_write(ADDRESS, &bytes));
        Ok(())
    });
    platform.set_wait_millis(|_ctx, n| {
        block_for(Duration::from_millis(n as _));
        Ok(())
    });
    platform.set_wait_value_mask_ex(|ctx, index, timeout_ms, value, mask, poll_delay_ms| {
        let i2c = unsafe { &mut *(ctx.0 as *mut I2c<I2C1>) };

        let mut buf = [0u8; 1];
        // TODO: consider the timeout limit
        loop {
            match i2c.blocking_write_read(ADDRESS, &index.to_be_bytes(), &mut buf) {
                Ok(_) => {
                    if (buf[0] & mask) == value {
                        return Ok(());
                    }
                }
                Err(err) => info!("Failed to read: {:?}", err),
            };
            block_for(Duration::from_millis(poll_delay_ms as _));
        }
    });

    let mut distance_sensor = VL53L4CX_Device::new(ADDRESS as _, platform);

    distance_sensor.init().unwrap();
    info!("Init finished");
    distance_sensor
        .set_distance_mode(DistanceMode::Medium)
        .unwrap();
    info!("Finished setting distance mode!");
    distance_sensor.start_measurement().unwrap();
    info!("Started measurement");
    loop {
        led_green.set_high();
        Timer::after(Duration::from_millis(250)).await;
        led_green.set_low();
        Timer::after(Duration::from_millis(250)).await;

        // let dist = distance_sensor.get_distance().unwrap();
        // info!("Distance: {}", dist);

        distance_sensor.wait_measurement_data_ready().unwrap();
        let n = distance_sensor.get_multi_ranging_data().unwrap();
        info!("Found {} objects!", n);
        distance_sensor
            .clear_interrupt_and_start_measurement()
            .unwrap();
    }
}
