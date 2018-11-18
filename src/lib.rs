#[macro_use] extern crate log;
extern crate simplelog;

use std::fs::File;
use simplelog::*;

mod vst;
mod function;
mod plugin;

#[no_mangle]
pub extern "C" fn VSTPluginMain(_callback: function::HostDispatch) -> *mut vst::Vst {
    // Set up a logger so we can see what's going on in the VST
    let mut logger_config = Config::default();
    logger_config.time_format = Some("%H:%M:%S%.6f");
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::max(), logger_config, File::create("/tmp/plugin.log").unwrap()),
        ]
    ).unwrap();

    // TODO: save _callback for later so that the plugin can call it.
    let plugin = plugin::Plugin::new();
    let vst = vst::Vst::new(plugin);

    Box::into_raw(Box::new(vst))
}