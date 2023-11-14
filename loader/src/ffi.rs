use std::ffi::c_char;
use plug_plugin::ffi::PluginVTableArray;

extern "C" {

	#[link_name = "plugin_library_load"]
	pub fn _plugin_library_load(path: *const c_char) -> *const ();
	
	#[link_name = "plugin_library_unload"]
	pub fn _plugin_library_unload(ptr: *const ());

	#[link_name = "plugin_library_vtables"]
	pub fn _plugin_library_vtables(ptr: *const ()) -> PluginVTableArray;
	
}