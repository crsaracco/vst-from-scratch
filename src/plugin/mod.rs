use std::os::raw::c_void;

mod opcode;
use self::opcode::Opcode;

pub struct Plugin {
}

impl Plugin {
    pub fn new() -> Self {
        Self { }
    }

    pub fn dispatch(&mut self, opcode: i32, _index: i32, _value: isize, _ptr: *mut c_void, _opt: f32) -> isize {
        let opcode_enum = Opcode::from(opcode);
        debug!("dispatch() | Opcode: {:2} ({:?})", opcode, opcode_enum);
        0
    }

    pub fn set_parameter(&mut self) {
        debug!("set_parameter()");
    }

    pub fn get_parameter(&mut self) -> f32 {
        debug!("get_parameter()");
        0.0
    }

    pub fn process(&mut self) {
        debug!("process()");
    }

    pub fn process_f64(&mut self) {
        debug!("process_f64");
    }
}