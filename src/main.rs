#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::ffi::c_void;

#[allow(unused_imports)]
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{dma::NoDma, peripherals::I2C1, time::Hertz};
#[allow(unused_imports)]
use embassy_stm32::{
    gpio::{Flex, Input, Level, Output, Pin, Pull, Speed},
    i2c::I2c,
    interrupt,
    peripherals::PB14,
};
use embassy_time::{block_for, Duration, Instant, Timer};
use spin::Mutex;
use stm_tof::{ffi::VL53LX_DEV, platform::Platform, Result, VL53L4CX_Device};
use {defmt_rtt as _, panic_probe as _};

static STOP_CM: f32 = 40.; // When red light turns  on
static WARN_CM: f32 = 90.; // When yellow light turns on, green if greater
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

    let mut data = [0u8; 1];
    unwrap!(i2c.blocking_write_read(ADDRESS, &[0x0F], &mut data));
    info!("Whoami: {}", data[0]);

    let mut led_green = Output::new(peripherals.PB14, Level::High, Speed::Low);

    let ctx = &mut i2c as *mut _ as *mut c_void;

    let mut platform = Platform::new(ctx);
    platform.set_read_byte(|ctx| {
        let i2c = unsafe { &mut *(ctx.0 as *mut I2c<I2C1>) };
        let mut data = [0u8; 1];
        unwrap!(i2c.blocking_read(ADDRESS, &mut data));
        Ok(data[0])
    });
    platform.set_read_word(|ctx| {
        let i2c = unsafe { &mut *(ctx.0 as *mut I2c<I2C1>) };
        let mut data = [0u8; 2];
        unwrap!(i2c.blocking_read(ADDRESS, &mut data));
        Ok(u16::from_le_bytes(data))
    });
    platform.set_write_byte(|ctx, byte| {
        let i2c = unsafe { &mut *(ctx.0 as *mut I2c<I2C1>) };
        let data = [byte];
        unwrap!(i2c.blocking_write(ADDRESS, &data));
        Ok(())
    });
    platform.set_write_word(|ctx, word| {
        let i2c = unsafe { &mut *(ctx.0 as *mut I2c<I2C1>) };
        let data = word.to_le_bytes();
        unwrap!(i2c.blocking_write(ADDRESS, &data));
        Ok(())
    });
    platform.set_wait_millis(|_ctx, n| {
        block_for(Duration::from_millis(n as _));
        Ok(())
    });
    platform.set_wait_value_mask_ex(|ctx, timeout_ms, value, mask, poll_delay_ms| {
        let i2c = unsafe { &mut *(ctx.0 as *mut I2c<I2C1>) };

        let mut buf = [0u8; 1];
        loop {
            match i2c.blocking_read(ADDRESS, &mut buf) {
                Ok(_) => {
                    info!(
                        "waiting for value: {}, mask: {}, recv: {}",
                        value, mask, buf[0]
                    );
                    buf[0] &= mask;
                    if buf[0] == value {
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
    loop {
        led_green.set_high();
        Timer::after(Duration::from_millis(250)).await;
        led_green.set_low();
        Timer::after(Duration::from_millis(250)).await;
    }
}
