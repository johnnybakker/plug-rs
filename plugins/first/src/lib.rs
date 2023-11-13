use plug_plugin::{Plugin, ffi };


struct FirstPlugin;

impl Plugin for FirstPlugin {
    fn load(&mut self) {
		println!("Load!");
    }

    fn unload(&mut self) {
        println!("Unload")
    }
}

#[no_mangle]
#[link_section = ".vtable"]
pub unsafe extern "C" fn __create_first_plugin() -> ffi::PluginVTable {
	let ptr = Box::into_raw(Box::new(FirstPlugin));
	let load_plugin = FirstPlugin::load as *const ();
	let unload_plugin = FirstPlugin::unload as *const ();

	let vtable = ffi::PluginVTable{
		ptr: std::mem::transmute(ptr),
		load: std::mem::transmute(load_plugin),
		unload: std::mem::transmute(unload_plugin)
	};
	
	return vtable;
}