#![allow(dead_code, non_camel_case_types, non_upper_case_globals)]

use core::ffi::CStr;
pub mod acc_alg_basic_utils;
pub mod acc_config;
pub mod acc_config_subsweep;
pub mod acc_definitions_a121;
pub mod acc_definitions_common;
pub mod acc_detector_distance;
pub mod acc_detector_distance_definitions;
pub mod acc_detector_presence;
pub mod acc_hal_definitions_a121;
pub mod acc_processing;
pub mod acc_rss_a121;
pub mod acc_sensor;
pub mod acc_version;

pub mod detector_distance {
    use super::*;

    use acc_detector_distance::*;

    pub struct Handle(*mut acc_detector_distance_handle_t);

    impl Handle {
        pub fn new(conf: &Config) -> Self {
            let hdl = unsafe { acc_detector_distance_create(conf.0) };
            if hdl.is_null() {
                unimplemented!("acc_detector_distance_create() failed");
            }
            Self(hdl)
        }
        pub fn get_buffer_size(&self) -> u32 {
            Self::get_buffer_sizes(self).0
        }
        pub fn get_calibration_buffer_size(&self) -> u32 {
            Self::get_buffer_sizes(self).1
        }
        fn get_buffer_sizes(handle: &Handle) -> (u32, u32) {
            let mut buffer_size = 0;
            let mut calibration_buffer_size = 0;
            let ret = unsafe {
                acc_detector_distance_get_buffer_sizes(
                    handle.0,
                    &mut buffer_size,
                    &mut calibration_buffer_size,
                )
            };
            if !ret {
                unimplemented!("acc_detector_distance_get_buffer_sizes() failed!");
            }
            (buffer_size, calibration_buffer_size)
        }
    }

    pub enum DistancePresetConfig {
        None = 0,
        Balanced = 1,
        HighAccuracy = 2,
    }

    /// RSS Sensor config
    pub struct Config(*mut acc_detector_distance::acc_detector_distance_config_t);

    impl Config {
        pub fn new(preset: DistancePresetConfig) -> Self {
            let mut conf = Self::default();
            conf.set_config(preset);
            conf
        }
        pub fn set_config(&mut self, preset: DistancePresetConfig) {
            match preset {
                DistancePresetConfig::None => {}
                DistancePresetConfig::Balanced => unsafe {
                    acc_detector_distance_config_start_set(self.0, 0.25);
                    acc_detector_distance_config_end_set(self.0, 3.0);
                    acc_detector_distance_config_max_step_length_set(self.0, 0);
                    acc_detector_distance_config_max_profile_set(
                        self.0,
                        acc_config_profile_t_ACC_CONFIG_PROFILE_5,
                    );
                    acc_detector_distance_config_reflector_shape_set(
                        self.0,
                        acc_detector_distance_reflector_shape_t_ACC_DETECTOR_DISTANCE_REFLECTOR_SHAPE_GENERIC,
                    );
                    acc_detector_distance_config_peak_sorting_set(
                        self.0,
                        acc_detector_distance_peak_sorting_t_ACC_DETECTOR_DISTANCE_PEAK_SORTING_STRONGEST,
                    );
                    acc_detector_distance_config_threshold_method_set(
                        self.0,
                        acc_detector_distance_threshold_method_t_ACC_DETECTOR_DISTANCE_THRESHOLD_METHOD_CFAR,
                    );
                    acc_detector_distance_config_num_frames_recorded_threshold_set(self.0, 100);
                    acc_detector_distance_config_threshold_sensitivity_set(self.0, 0.5);
                    acc_detector_distance_config_signal_quality_set(self.0, 15.0);
                    acc_detector_distance_config_close_range_leakage_cancellation_set(
                        self.0, false,
                    );
                },
                DistancePresetConfig::HighAccuracy => unsafe {
                    acc_detector_distance_config_start_set(self.0, 0.25);
                    acc_detector_distance_config_end_set(self.0, 3.0);
                    acc_detector_distance_config_max_step_length_set(self.0, 2);
                    acc_detector_distance_config_max_profile_set(
                        self.0,
                        acc_config_profile_t_ACC_CONFIG_PROFILE_3,
                    );
                    acc_detector_distance_config_reflector_shape_set(
                        self.0,
                        acc_detector_distance_reflector_shape_t_ACC_DETECTOR_DISTANCE_REFLECTOR_SHAPE_GENERIC,
                    );
                    acc_detector_distance_config_peak_sorting_set(
                        self.0,
                        acc_detector_distance_peak_sorting_t_ACC_DETECTOR_DISTANCE_PEAK_SORTING_STRONGEST,
                    );
                    acc_detector_distance_config_threshold_method_set(
                        self.0,
                        acc_detector_distance_threshold_method_t_ACC_DETECTOR_DISTANCE_THRESHOLD_METHOD_CFAR,
                    );
                    acc_detector_distance_config_num_frames_recorded_threshold_set(self.0, 100);
                    acc_detector_distance_config_threshold_sensitivity_set(self.0, 0.5);
                    acc_detector_distance_config_signal_quality_set(self.0, 20.0);
                    acc_detector_distance_config_close_range_leakage_cancellation_set(
                        self.0, false,
                    );
                },
            }
        }
    }

    impl Default for Config {
        fn default() -> Self {
            let conf = unsafe { acc_detector_distance::acc_detector_distance_config_create() };
            if conf.is_null() {
                unimplemented!("Config creation failed!");
            }
            Self(conf)
        }
    }

    impl Drop for Config {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { acc_detector_distance::acc_detector_distance_config_destroy(self.0) };
            }
        }
    }
}
pub mod rss {
    use super::*;

    use acc_rss_a121::*;
    use core::ffi::c_void;

    pub fn hal_register(hal: &AccHAL) {
        if !unsafe { acc_rss_hal_register(&hal.0 as *const _) } {
            unimplemented!("Failed to register HAL");
        }
    }

    pub type MemAllocFn = unsafe extern "C" fn(usize) -> *mut c_void;
    pub type MemFreeFn = unsafe extern "C" fn(*mut c_void);
    pub type TransferFn = acc_hal_sensor_transfer8_function_t;
    pub type Transfer16Fn = acc_hal_sensor_transfer16_function_t;
    pub type LogFn = acc_hal_log_function_t;

    pub struct AccHAL(pub(crate) acc_hal_a121_t);

    impl AccHAL {
        pub fn new(
            transfer: TransferFn,
            transfer16: Transfer16Fn,
            log: LogFn,
            mem_alloc: MemAllocFn,
            mem_free: MemFreeFn,
        ) -> Self {
            if transfer.is_none() {
                unimplemented!("Must define a transfer function");
            }
            let inner = acc_hal_a121_t {
                max_spi_transfer_size: 65535,
                mem_alloc: Some(mem_alloc),
                mem_free: Some(mem_free),
                transfer,
                log,
                optimization: acc_hal_optimization_t { transfer16 },
            };
            let slf = Self(inner);
            slf.register();
            slf
        }
        fn register(&self) {
            hal_register(self)
        }
    }

    /// RSS Sensor config
    pub struct Config(*mut acc_config_t);

    impl Default for Config {
        fn default() -> Self {
            let conf = unsafe { acc_config_create() };
            Self(conf)
        }
    }

    impl Drop for Config {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { acc_config_destroy(self.0) };
            }
        }
    }
}

pub mod version {

    use super::*;

    pub fn version_get<F: FnOnce(&CStr)>(cb: F) {
        let version = unsafe { CStr::from_ptr(acc_version::acc_version_get() as _) };
        cb(version);
    }

    pub fn version_get_hex() -> u32 {
        unsafe { acc_version::acc_version_get_hex() }
    }
}
