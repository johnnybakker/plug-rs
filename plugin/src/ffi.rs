use crate::Plugin;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PluginVTable {
	
	// Construct and destruct
	pub construct: extern fn() -> *mut(),
	pub destruct: extern fn(*mut()),

	// Member methods
	pub load: extern fn(*mut ()),
	pub unload: extern fn(*mut ())

}

impl PluginVTable {
	
	pub unsafe fn new<T: Plugin>() -> PluginVTable {
		PluginVTable { 
			construct: std::mem::transmute(PluginVTable::construct::<T> as *mut()), 
			destruct: std::mem::transmute(PluginVTable::destruct::<T> as *mut()), 
			load: std::mem::transmute(T::load as *mut()), 
			unload: std::mem::transmute(T::unload as *mut()) 
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