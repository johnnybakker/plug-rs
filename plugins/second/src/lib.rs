use plug_plugin::{Plugin, ffi::PluginVTable};

struct SecondPlugin;

impl SecondPlugin {
	
}

impl Default for SecondPlugin {
    fn default() -> Self {
        Self
    }
}

impl Plugin for SecondPlugin {
    fn load(&mut self) {
        todo!()
    }

    fn unload(&mut self) {
        todo!()
    }
}

#[no_mangle]
#[link_section = ".plugins"]
pub unsafe extern "C" fn __create_second_plugin() -> PluginVTable {
	let ptr = Box::into_raw(Box::new(SecondPlugin::default()));
	let load = SecondPlugin::load as *const ();
	let unload = SecondPlugin::unload as *const ();

	let vtable = PluginVTable{
		ptr: std::mem::transmute(ptr),
		load: std::mem::transmute(load),
		unload: std::mem::transmute(unload)
	};
	
	vtable
}