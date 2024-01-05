#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

extern crate alloc;

use alloc::vec::Vec;
use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
};

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use embedded_alloc::Heap;

use garage_backup_sensor::a121;

#[global_allocator]
static HEAP: Heap = Heap::empty();

type LayoutStorageSize = u16;
const STORAGE_SIZE_BYTES: usize = (LayoutStorageSize::BITS / 8) as usize;

// struct DistanceDetectorResources {
//     sensor: Sensor,
// }
fn get_layout(size: usize) -> Layout {
    // TODO: I'm not sure if align is correct?
    Layout::from_size_align(size, core::mem::align_of::<u8>()).unwrap()
}
unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
    info!("Allocating {} bytes", size);

    // size less than types we're about to cast to
    if size <= LayoutStorageSize::MAX as _ {
        defmt::unimplemented!("`size` to allocate larger than LayoutStorageSize");
    }

    // allocate size + value of `size` in bytes
    let size_bytes = (size as LayoutStorageSize).to_le_bytes();
    let ptr = HEAP.alloc(get_layout(size + size_bytes.len()));

    // need to store size for 'free'.
    // Store size at start then return offset from there
    let slice = unsafe { core::slice::from_raw_parts_mut(ptr, size) };
    slice[..size_bytes.len()].copy_from_slice(&size_bytes);
    ptr.add(size_bytes.len()) as *mut c_void
}

unsafe extern "C" fn free(ptr: *mut c_void) {
    info!("Deallocating");

    // ptr is at the offset from size storage done in malloc, move to the start
    let ptr = (ptr as *mut u8).sub(STORAGE_SIZE_BYTES);

    // Get the size stored at the beginning of this memory chunk
    let size_bytes_slice = unsafe { core::slice::from_raw_parts(ptr, STORAGE_SIZE_BYTES) };
    let size_bytes_array: [u8; STORAGE_SIZE_BYTES] = size_bytes_slice.try_into().unwrap();
    let size = LayoutStorageSize::from_le_bytes(size_bytes_array) as usize;

    // Deallocate
    HEAP.dealloc(ptr as *mut u8, get_layout(size))
}

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

    // version
    a121::version::version_get(|v| info!("Version: {}", v.to_str().unwrap()));

    // HAL for radar
    let hal = a121::rss::AccHAL::new(None, None, None, malloc, free);

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
