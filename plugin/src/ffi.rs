use std::{panic::catch_unwind};

use crate::Plugin;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PluginVTable {
	
	// Construct and destruct
	pub construct: extern fn() -> *mut(),
	pub destruct: extern fn(*mut()),

	// Member methods
	pub load: extern fn(*mut ()) -> *mut(),
	pub unload: extern fn(*mut ()) -> *mut()
}

impl PluginVTable {
	
	pub unsafe fn new<T: Plugin>() -> PluginVTable {
		PluginVTable { 
			construct: std::mem::transmute(PluginVTable::construct::<T> as *mut()), 
			destruct: std::mem::transmute(PluginVTable::destruct::<T> as *mut()), 
			load: std::mem::transmute(PluginVTable::load::<T> as *mut()), 
			unload: std::mem::transmute(PluginVTable::unload::<T> as *mut()) 
		}
	}

	unsafe fn unload<T: Plugin>(ptr: *mut T) -> *mut () {
		match catch_unwind(||{ T::unload(&mut *ptr) }) {
			Ok(_) => std::ptr::null_mut(),
			Err(e) => Box::into_raw(e) as *mut(),
		}	
	}

	unsafe fn load<T: Plugin>(ptr: *mut T) -> *mut() {
		match catch_unwind(||{ T::load(&mut *ptr) }) {
			Ok(_) => std::ptr::null_mut(),
			Err(e) => {
				//println!("{:?}", e.downcast::<&str>());
				Box::into_raw(e) as *mut()
				//std::ptr::null_mut()
			},
		}
	}

	fn construct<T: Plugin>() -> *mut T {
		let plugin = Box::new(T::default());
		Box::into_raw(plugin)
	}

	unsafe fn destruct<T: Plugin>(plugin: *mut T) {
		let _ = Box::from_raw(plugin);
	}

}

#[repr(C)]
pub struct PluginVTableArray {
	pub length: usize,
	pub vtables: *mut PluginVTable,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RuntimeVTable {
	pub ptr: *mut ()
}