use std::os::raw::c_void;

mod function;

/// Used with the VST API to pass around plugin information.
#[allow(non_snake_case)]
#[repr(C)]
pub struct AEffect {
    pub magic: i32,                                // Magic number. Must be 0x56737450 ("VstP")
    pub dispatcher: function::PluginDispatch,      // Host to plug-in dispatcher.
    pub _process: function::Process,               // DEPRECATED. (Accumulating process mode process() function)
    pub setParameter: function::SetParameter,      // Set the value of an automatable parameter.
    pub getParameter: function::GetParameter,      // Get the value of an automatable parameter.
    pub numPrograms: i32,                          // Number of "programs" (presets)
    pub numParams: i32,                            // Number of parameters. All presets are assumed to have this many parameters.
    pub numInputs: i32,                            // Number of audio inputs.
    pub numOutputs: i32,                           // Number of audio outputs.
    pub flags: i32,                                // Bitmask. TODO: make flags to put here
    pub reserved1: isize,                          // Reserved for host. Must be set to 0.
    pub reserved2: isize,                          // Reserved for host. Must be set to 0.
    pub initialDelay: i32,                         // Initial sample delay. TODO: document better
    pub _realQualities: i32,                       // DEPRECATED.
    pub _offQualities: i32,                        // DEPRECATED.
    pub _ioRatio: f32,                             // DEPRECATED.
    pub object: *mut c_void,                       // Void pointer usable by the API to store object data. (???)
    pub user: *mut c_void,                         // User-defined pointer. (???)
    pub uniqueId: i32,                             // Unique identifier for the VST. Used during save/load of preset and project.
    pub version: i32,                              // Plugin version.
    pub processReplacing: function::Process,       // process() function (f32)
    pub processReplacingF64: function::ProcessF64, // process() function (f64)
    pub future: [u8; 56],                          // Reserved for future use (should be set to 0).
}

#[no_mangle]
pub extern "C" fn VSTPluginMain(_callback: function::HostDispatch) -> *mut AEffect {
    // TODO: save the HostCallbackProc for later so that the plugin can call it.
    // Need an actual Plugin struct for this though.

    let effect = AEffect {
        magic: 0x56737450, // shia_labeouf_magic.gif
        dispatcher: function::plugin_dispatch,
        _process: function::process_deprecated,
        setParameter: function::set_parameter,
        getParameter: function::get_parameter,
        numPrograms: 1,
        numParams: 0,
        numInputs: 2, // Stereo input
        numOutputs: 2, // Stereo output
        flags: (1 << 4), // Can only handle f32 samples
        reserved1: 0,
        reserved2: 0,
        initialDelay: 0,
        _realQualities: 0,
        _offQualities: 0,
        _ioRatio: 0.0,
        object: Box::into_raw(Box::new(())) as *mut c_void, // i_have_do_idea_what_im_doing.jpg
        user: Box::into_raw(Box::new(())) as *mut c_void,
        uniqueId: 13371337, // Some random VST ID
        version: 1, // Version 0.0.0.1
        processReplacing: function::process,
        processReplacingF64: function::process_f64,
        future: [0u8; 56],
    };

    Box::into_raw(Box::new(effect))
}