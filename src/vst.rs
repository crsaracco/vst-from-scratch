use crate::function;
use crate::plugin::Plugin;

use std::os::raw::c_void;

#[repr(C)]
pub struct Vst {
    pub magic: i32,                                // Magic number. Must be 0x56737450 ("VstP")
    pub plugin_dispatch: function::PluginDispatch, // Host to plug-in dispatcher.
    pub _process_deprecated: function::Process,    // DEPRECATED. (process samples - accumulating process mode)
    pub set_parameter: function::SetParameter,     // Set the value of an automatable parameter.
    pub get_parameter: function::GetParameter,     // Get the value of an automatable parameter.
    pub num_presets: i32,                          // Number of presets this plugin has
    pub num_parameters: i32,                       // Number of parameters. All presets are assumed to have this many parameters.
    pub num_inputs: i32,                           // Number of audio inputs.
    pub num_outputs: i32,                          // Number of audio outputs.
    pub flags: i32,                                // Bitmask. TODO: make flags to put here
    pub reserved_1: isize,                         // Reserved for host. Must be set to 0.
    pub reserved_2: isize,                         // Reserved for host. Must be set to 0.
    pub initial_delay: i32,                        // Initial sample delay. TODO: document better
    pub _real_qualities: i32,                      // DEPRECATED.
    pub _off_qualities: i32,                       // DEPRECATED.
    pub _io_ratio: f32,                            // DEPRECATED.
    pub plugin: *mut c_void,                       // Store a pointer to the plugin. We can use this to call plugin member functions.
    pub user: *mut c_void,                         // User-defined pointer. (???)
    pub unique_id: i32,                            // Unique identifier for the VST. Used during save/load of preset and project.
    pub version: i32,                              // Plugin version.
    pub process: function::Process,                // process samples (f32)
    pub process_f64: function::ProcessF64,         // process samples (f64) -- note: might never actually be called by the host.
    pub future: [u8; 56],                          // Reserved for future use (should be set to 0).
}

impl Vst {
    pub fn new(plugin: Plugin) -> Self {
        Self {
            magic: 0x56737450, // shia_labeouf_magic.gif
            plugin_dispatch: function::plugin_dispatch,
            _process_deprecated: function::process_deprecated,
            set_parameter: function::set_parameter,
            get_parameter: function::get_parameter,
            num_presets: 1,
            num_parameters: 0,
            num_inputs: 2, // Stereo input
            num_outputs: 2, // Stereo output
            flags: (1 << 4), // Can only handle f32 samples
            reserved_1: 0,
            reserved_2: 0,
            initial_delay: 0,
            _real_qualities: 0,
            _off_qualities: 0,
            _io_ratio: 0.0,
            plugin: Box::into_raw(Box::new(plugin)) as *mut c_void,
            user: Box::into_raw(Box::new(())) as *mut c_void,
            unique_id: 13371337, // Some random VST ID
            version: 1, // Version 0.0.0.1
            process: function::process,
            process_f64: function::process_f64,
            future: [0u8; 56],
        }
    }

    pub unsafe fn get_plugin(&mut self) -> &mut Plugin {
        let plugin = &mut *(self.plugin as *mut Plugin);
        plugin
    }
}