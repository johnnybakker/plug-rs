use std::{ffi::{CString}, os::raw::c_char};
use plug_plugin::ffi::{LibraryVTable, RuntimeVTable};

extern "C" {

	#[link_name = "load_plugin_library"]
	fn _load_plugin_library(runtime: RuntimeVTable, path: *const c_char) -> *mut LibraryVTable;

}

pub fn load_plugin_library<T: AsRef<str>>(path: T) -> Option<LibraryVTable> {
	let path = path.as_ref().to_owned();
	let path = CString::new(path)
		.expect("Expects a string");

	let ptr = unsafe {
		_load_plugin_library(RuntimeVTable { ptr: std::ptr::null_mut() }, path.as_ptr())
	};

	if ptr.is_null() {
		return None;
	}

	let table = *unsafe {
		Box::from_raw(ptr)
	};

	Some(table)
}