use plug_plugin::{Plugin, ffi };

struct FirstPlugin;

impl Plugin for FirstPlugin {
    fn load(&mut self) {
		println!("Load!!!!!!!!!");
    }

    fn unload(&mut self) {
        println!("Unload!!!!!!!!!!!!!!")
    }
}

impl Default for FirstPlugin {
    fn default() -> Self {
        Self {  }
    }
}

#[no_mangle]
#[link_section = ".vtable"]
pub unsafe extern "C" fn __FIRST_PLUGIN_VTABLE() -> ffi::PluginVTable {
	ffi::PluginVTable::new::<FirstPlugin>()
}