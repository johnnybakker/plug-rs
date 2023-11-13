use std::ffi::c_char;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LibraryVTable {
	pub path: *mut c_char,
	pub ptr: *mut (),
	pub plugins: *mut PluginVTable,
	pub amount: usize 
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PluginVTable {
	pub ptr: *mut (),
	pub load: extern fn(*mut ()),
	pub unload: extern fn(*mut ())
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RuntimeVTable {
	pub ptr: *mut ()
}