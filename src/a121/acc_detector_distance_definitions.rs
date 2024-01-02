/* automatically generated by rust-bindgen 0.69.1 */

#[doc = " Return peaks with the closest detection first."]
pub const acc_detector_distance_peak_sorting_t_ACC_DETECTOR_DISTANCE_PEAK_SORTING_CLOSEST:
    acc_detector_distance_peak_sorting_t = 0;
#[doc = " Return peaks with the peak with the highest RCS first."]
pub const acc_detector_distance_peak_sorting_t_ACC_DETECTOR_DISTANCE_PEAK_SORTING_STRONGEST:
    acc_detector_distance_peak_sorting_t = 1;
#[doc = " @brief Enum for peak sorting algorithms"]
pub type acc_detector_distance_peak_sorting_t = cty::c_uint;
#[doc = " Compares processed data against a fixed amplitude value"]
pub const acc_detector_distance_threshold_method_t_ACC_DETECTOR_DISTANCE_THRESHOLD_METHOD_FIXED_AMPLITUDE : acc_detector_distance_threshold_method_t = 0 ;
#[doc = " Compares processed data against a fixed strength value"]
pub const acc_detector_distance_threshold_method_t_ACC_DETECTOR_DISTANCE_THRESHOLD_METHOD_FIXED_STRENGTH : acc_detector_distance_threshold_method_t = 1 ;
#[doc = " Compares processed data against a recorded threshold"]
pub const acc_detector_distance_threshold_method_t_ACC_DETECTOR_DISTANCE_THRESHOLD_METHOD_RECORDED : acc_detector_distance_threshold_method_t = 2 ;
#[doc = " Uses the CFAR algorithm as a threshold"]
pub const acc_detector_distance_threshold_method_t_ACC_DETECTOR_DISTANCE_THRESHOLD_METHOD_CFAR:
    acc_detector_distance_threshold_method_t = 3;
#[doc = " @brief Enum for threshold methods"]
pub type acc_detector_distance_threshold_method_t = cty::c_uint;
#[doc = " Use a generic reflector shape for RCS calculation"]
pub const acc_detector_distance_reflector_shape_t_ACC_DETECTOR_DISTANCE_REFLECTOR_SHAPE_GENERIC:
    acc_detector_distance_reflector_shape_t = 0;
#[doc = " Use a planar reflector shape for RCS calculation"]
pub const acc_detector_distance_reflector_shape_t_ACC_DETECTOR_DISTANCE_REFLECTOR_SHAPE_PLANAR:
    acc_detector_distance_reflector_shape_t = 1;
#[doc = " @brief Enum for reflector shapes"]
pub type acc_detector_distance_reflector_shape_t = cty::c_uint;
