use std::os::raw::{
    c_void,
    c_char,
};

mod opcode;
mod category;
use self::opcode::Opcode;
use self::category::Category;


pub struct Plugin {
    vendor_name: String,
    effect_name: String,
    vendor_version: isize,
    sample_rate: f32,
    block_size: isize,
}

impl Plugin {
    pub fn new() -> Self {
        // Sane defaults for now:
        Self {
            vendor_name: String::from("Vandelay Industries"),
            effect_name: String::from("VST From Scratch"),
            vendor_version: 1,
            sample_rate: 44100.0,
            block_size: 256,
        }
    }

    pub fn dispatch(&mut self, opcode: i32, index: i32, value: isize, ptr: *mut c_void, opt: f32) -> isize {
        let opcode_debug = opcode;
        let opcode = Opcode::from(opcode);

        match opcode {
            /*  0 */ Opcode::Initialize       => self.initialize(),
            /*  1 */ Opcode::Terminate        => self.terminate(),
            /* 10 */ Opcode::SetSampleRate    => self.set_sample_rate(opt),
            /* 11 */ Opcode::SetBlockSize     => self.set_block_size(value),
            /* 12 */ Opcode::StateChanged     => self.state_changed(value),
            /* 29 */ Opcode::GetPresetName    => self.get_preset_name(index, ptr),
            /* 33 */ Opcode::GetInputInfo     => self.get_input_info(index, ptr),
            /* 34 */ Opcode::GetOutputInfo    => self.get_output_info(index, ptr),
            /* 35 */ Opcode::GetCategory      => self.get_category(),
            /* 45 */ Opcode::GetEffectName    => self.get_effect_name(ptr),
            /* 47 */ Opcode::GetVendorName    => self.get_vendor_name(ptr),
            /* 49 */ Opcode::GetVendorVersion => self.get_vendor_version(),
            /* 51 */ Opcode::CanDo            => self.can_do(ptr),
            /* 58 */ Opcode::GetApiVersion    => self.get_api_version(),
            /* 71 */ Opcode::StartProcess     => self.start_process(),
            /* 72 */ Opcode::StopProcess      => self.stop_process(),

            // Quiet opcodes #52, 66, 3 for now for cleaner logs. TODO: remove
            Opcode::GetTailSize => 0,
            Opcode::GetCurrentPresetNum => 0,
            Opcode::GetMidiKeyName => 0,
            _ => {
                dispatch_debug(opcode_debug, index, value, ptr, opt);
                0
            }
        }
    }

    pub fn set_parameter(&mut self) {
        warn!("set_parameter()");
    }

    pub fn get_parameter(&mut self) -> f32 {
        warn!("get_parameter()");
        0.0
    }

    pub fn process(&mut self) {
        // TODO: re-enable this warning. Commenting it out for now for cleaner logs.
        // warn!("process()");
    }

    pub fn process_f64(&mut self) {
        warn!("process_f64");
    }

    // =======================
    // === Opcode handlers ===
    // =======================

    //  0: Initialize
    fn initialize(&mut self) -> isize {
        debug!("initialize()");
        0
    }

    //  1: Terminate
    fn terminate(&mut self) -> isize {
        debug!("terminate()");
        0
    }

    // 10: SetSampleRate
    fn set_sample_rate(&mut self, sample_rate: f32) -> isize {
        self.sample_rate = sample_rate;
        debug!("set_sample_rate(): Sample rate set to {}", sample_rate);
        0
    }

    // 11: SetBlockSize
    fn set_block_size(&mut self, block_size: isize) -> isize {
        self.block_size = block_size;
        debug!("set_block_size(): Block size set to {}", block_size);
        0
    }

    // 12: StateChanged
    fn state_changed(&mut self, state: isize) -> isize {
        // TODO: actually change state based on this function call
        if state == 0 {
            debug!("state_changed(): SUSPEND");
        }
        else {
            debug!("state_changed(): RESUME");
        }

        0
    }

    // 29: GetPresetName
    // Returns 1 if successful, 0 otherwise.
    fn get_preset_name(&mut self, _index: i32, ptr: *mut c_void) -> isize {
        // TODO: actually get this from the Plugin struct.
        // TODO: Max length of this string is 24. Set this as a const somewhere.
        let preset_name = "SomePresetName";
        copy_string(ptr, preset_name, 24);
        debug!("get_preset_name() -> {}", preset_name);
        1
    }

    // 33: GetInputInfo
    // Returns 1 if supported, 0 otherwise.
    fn get_input_info(&mut self, _index: i32, _ptr: *mut c_void) -> isize {
        // TODO: currently unsupported. Make a `VstPinProperties` struct equivalent.
        debug!("get_input_info(): [currently unsupported]");
        0
    }

    // 33: GetInputInfo
    // Returns 1 if supported, 0 otherwise.
    fn get_output_info(&mut self, _index: i32, _ptr: *mut c_void) -> isize {
        // TODO: currently unsupported. Make a `VstPinProperties` struct equivalent.
        debug!("get_output_info(): [currently unsupported]");
        0
    }

    // 35: GetCategory
    fn get_category(&mut self) -> isize {
        let category = Category::Effect;
        debug!("get_category() -> {:?}", category);
        category as isize
    }

    // 45: GetEffectName
    fn get_effect_name(&mut self, ptr: *mut c_void) -> isize {
        // TODO: Returns 1 if good, 0 if bad ???
        // TODO: Max length of this string is 64. Set this as a const somewhere.
        copy_string(ptr, self.effect_name.as_str(), 64);
        debug!("get_effect_name() -> {}", self.effect_name);
        1
    }

    // 47: GetVendorName
    fn get_vendor_name(&mut self, ptr: *mut c_void) -> isize {
        // TODO: Returns 1 if good, 0 if bad ???
        // TODO: Max length of this string is 64. Set this as a const somewhere.
        copy_string(ptr, self.vendor_name.as_str(), 64);
        debug!("get_vendor_name() -> {}", self.vendor_name);
        1
    }

    // 49: GetVendorVersion
    fn get_vendor_version(&mut self) -> isize {
        let vendor_version = self.vendor_version;
        debug!("get_vendor_version() -> {}", vendor_version);
        vendor_version
    }

    // 51: CanDo
    // TODO: Make an enum for this. (-1 = no, 0 = maybe, 1 = yes)
    // TODO: Make some sort of smart answerer to this (string -> enum)
    fn can_do(&mut self, ptr: *mut c_void) -> isize {
        let can_do_string = read_string(ptr);
        debug!("can_do(\"{}\") -> 0", can_do_string);
        0
    }

    // 58: GetApiVersion
    fn get_api_version(&mut self) -> isize {
        let api_version = 2400;
        debug!("get_api_version() -> {:?}", api_version);
        api_version
    }

    // 71: StartProcess
    fn start_process(&mut self) -> isize {
        // Steinberg SDK notes:
        // Called one time before the start of process call. This indicates that the process call
        // will be interrupted (due to Host reconfiguration or bypass state when the plug-in doesn't
        // support softBypass)
        // TODO: start_process functionality
        debug!("start_process()");
        0 // TODO: What should the return value be?
    }

    // 72: StopProcess
    fn stop_process(&mut self) -> isize {
        // Steinberg SDK notes:
        // Called one time before the start of process call. This indicates that the process call
        // will be interrupted (due to Host reconfiguration or bypass state when the plug-in doesn't
        // support softBypass)
        // TODO: start_process functionality
        debug!("stop_process()");
        0 // TODO: What should the return value be?
    }
}


fn dispatch_debug(opcode: i32, index: i32, value: isize, ptr: *mut c_void, opt: f32) {
    let opcode_debug = opcode;
    let opcode = Opcode::from(opcode);

    warn!("dispatch() | Opcode: {:2} ({:?}) | index: {}, value: {}, ptr: {:?}, opt: {}",
           opcode_debug, opcode, index, value, ptr, opt
    );
}

// TODO: move these two functions to a new FFI-related module.

fn copy_string(dst: *mut c_void, src: &str, max: usize) -> isize {
    unsafe {
        use std::cmp::min;
        use libc::{c_void, memset, memcpy};

        let dst = dst as *mut c_void;
        memset(dst, 0, max);
        memcpy(dst, src.as_ptr() as *const c_void, min(max, src.as_bytes().len()));
    }

    1 // Success
}

// Read a string from the `ptr` buffer
fn read_string(ptr: *mut c_void) -> String {
    use std::ffi::CStr;

    String::from_utf8_lossy(unsafe { CStr::from_ptr(ptr as *mut c_char).to_bytes() }).into_owned()
}