pub struct Plugin {
}

impl Plugin {
    pub fn new() -> Self {
        Self { }
    }

    pub fn dispatch(&mut self) -> isize {
        debug!("dispatch()");
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