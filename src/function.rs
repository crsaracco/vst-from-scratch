use std::os::raw::c_void;


/// Host dispatch function:
/// Called by the plugin to request something from the host.
pub type HostDispatch = fn(
    effect: *mut crate::AEffect,
    opcode: i32,
    index: i32,
    value: isize,
    ptr: *mut c_void,
    opt: f32,
) -> isize;


/// Plugin dispatch function:
/// Called by the host to request something from the plugin.
pub type PluginDispatch = fn(
    effect: *mut crate::AEffect,
    opcode: i32,
    index: i32,
    value: isize,
    ptr: *mut c_void,
    opt: f32,
) -> isize;

pub fn plugin_dispatch(_effect: *mut crate::AEffect, _opcode: i32, _index: i32, _value: isize, _ptr: *mut c_void, _opt: f32) -> isize {
    debug!("plugin_dispatch");
    return 0;
}


/// Set parameter function:
/// Called by the host to request the plugin to set a parameter to a given value.
pub type SetParameter = fn(
    effect: *mut crate::AEffect,
    index: i32,
    value: f32,
);

pub fn set_parameter(_effect: *mut crate::AEffect, _index: i32, _value: f32) {
    debug!("set_parameter");
}


/// Get parameter function:
/// Called by the host to get the value of a parameter from the plugin.
pub type GetParameter = fn(
    effect: *mut crate::AEffect,
    index: i32,
) -> f32;

pub fn get_parameter(_effect: *mut crate::AEffect, _index: i32) -> f32 {
    debug!("get_parameter");
    return 0.0;
}


/// Process function:
/// Called by the host to get the plugin to process a chunk of samples.
/// (f32 version)
pub type Process = fn(
    effect: *mut crate::AEffect,
    inputs: *const *const f32,
    outputs: *mut *mut f32,
    sample_frames: i32,
);

pub fn process(_effect: *mut crate::AEffect, _raw_inputs: *const *const f32, _raw_outputs: *mut *mut f32, _samples: i32) {
    debug!("process");
}

pub fn process_deprecated(_effect: *mut crate::AEffect, _raw_inputs: *const *const f32, _raw_outputs: *mut *mut f32, _samples: i32) {
    debug!("process_deprecated");
    // This function used to take samples, calculate something based on those samples, and
    // *add* the result to the original sample (for some reason...?)
    // It is now DEPRECATED, so it shouldn't ever be called.
    // Hosts should always call `process()` or `process_f64()` instead.
}


/// Process function:
/// Called by the host to get the plugin to process a chunk of samples.
/// (f64 version)
pub type ProcessF64 = fn(
    effect: *mut crate::AEffect,
    inputs: *const *const f64,
    outputs: *mut *mut f64,
    sample_frames: i32
);

pub fn process_f64(_effect: *mut crate::AEffect, _raw_inputs: *const *const f64, _raw_outputs: *mut *mut f64, _samples: i32) {
    debug!("process_f64");
}