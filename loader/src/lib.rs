mod ffi;

use std::ffi::CString;
use plug_plugin::ffi::PluginVTable;
use ffi::*;

pub struct PluginInstance {
	ptr: *mut(),
	vtable: PluginVTable
}

impl From<PluginVTable> for PluginInstance {
    fn from(vtable: PluginVTable) -> Self {
        let ptr = (vtable.construct)();
		(vtable.load)(ptr);
		Self { ptr, vtable }
    }
}

impl Drop for PluginInstance {
    fn drop(&mut self) {
		(self.vtable.unload)(self.ptr);
		(self.vtable.destruct)(self.ptr);
    }
}



pub struct PluginLibrary { 
	ptr: *const(),
	path: String,
	plugins: Vec<PluginInstance>
}

impl PluginLibrary {

	pub fn new<P>(path: P) -> Self where P:  AsRef<str> {
		let path = path.as_ref().to_owned();

		PluginLibrary { 
			ptr: std::ptr::null(), 
			path, 
			plugins: Vec::new()
		}
	}

	pub fn load(&mut self) -> bool {
		
		if !self.ptr.is_null() {
			return true;
		}

		self.ptr = PluginLibrary::load_ptr(&self.path);

		if self.ptr.is_null() {
			return false;
		}

		self.plugins = PluginLibrary::load_plugins(self.ptr);

		true
	}

	pub fn reload(&mut self) -> bool {
		self.unload();
		self.load()
	}

	pub fn unload(&mut self) {
		self.plugins.clear();
		self.release_ptr();
	}

	fn load_ptr<P>(path: P) -> *const () where P:  AsRef<str>  {
		unsafe {
			let path = CString::new(path.as_ref())
				.expect("Expects a string");

			_plugin_library_load(path.as_ptr())
		}
	}

	fn load_plugins(ptr: *const ()) -> Vec<PluginInstance> {
		let arr = unsafe {
			_plugin_library_vtables(ptr)
		};

		let vtables = unsafe {
			std::slice::from_raw_parts_mut(arr.vtables, arr.length)	
		};

		vtables.iter_mut().map(|v|(*v).into()).collect()
	}

	fn release_ptr(&mut self) {
		match self.ptr.is_null() {
			true => println!("Library was nullptr"),
			false => unsafe {
				_plugin_library_unload(self.ptr)
			},
		};

		self.ptr = std::ptr::null();
	}

}

impl Drop for PluginLibrary {
    fn drop(&mut self) {
		self.unload()
    }
}
