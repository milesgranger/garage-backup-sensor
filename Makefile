
bindgen:
	bindgen --use-core --ctypes-prefix cty vendored/cortex_m4_gcc/rss/include/acc_alg_basic_utils.h > src/a121/acc_alg_basic_utils.rs
	bindgen --use-core --ctypes-prefix cty vendored/cortex_m4_gcc/rss/include/acc_config.h > src/a121/acc_config.rs
	bindgen --use-core --ctypes-prefix cty vendored/cortex_m4_gcc/rss/include/acc_config_subsweep.h > src/a121/acc_config_subsweep.rs
	bindgen --use-core --ctypes-prefix cty vendored/cortex_m4_gcc/rss/include/acc_definitions_a121.h > src/a121/acc_definitions_a121.rs
	bindgen --use-core --ctypes-prefix cty vendored/cortex_m4_gcc/rss/include/acc_definitions_common.h > src/a121/acc_definitions_common.rs
	bindgen --use-core --ctypes-prefix cty vendored/cortex_m4_gcc/rss/include/acc_detector_distance_definitions.h > src/a121/acc_detector_distance_definitions.rs
	bindgen --use-core --ctypes-prefix cty vendored/cortex_m4_gcc/rss/include/acc_detector_distance.h > src/a121/acc_detector_distance.rs
	bindgen --use-core --ctypes-prefix cty vendored/cortex_m4_gcc/rss/include/acc_detector_presence.h > src/a121/acc_detector_presence.rs
	bindgen --use-core --ctypes-prefix cty vendored/cortex_m4_gcc/rss/include/acc_hal_definitions_a121.h > src/a121/acc_hal_definitions_a121.rs
	bindgen --use-core --ctypes-prefix cty vendored/cortex_m4_gcc/rss/include/acc_processing.h > src/a121/acc_processing.rs
	bindgen --use-core --ctypes-prefix cty vendored/cortex_m4_gcc/rss/include/acc_rss_a121.h > src/a121/acc_rss_a121.rs
	bindgen --use-core --ctypes-prefix cty vendored/cortex_m4_gcc/rss/include/acc_sensor.h > src/a121/acc_sensor.rs
	bindgen --use-core --ctypes-prefix cty vendored/cortex_m4_gcc/rss/include/acc_version.h > src/a121/acc_version.rs
