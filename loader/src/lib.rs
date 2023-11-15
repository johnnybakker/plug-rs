mod ffi;

use std::{ffi::CString, any::Any};
use plug_plugin::ffi::PluginVTable;
use ffi::*;

#[derive(Debug)]
pub struct PluginData;

pub struct PluginInstance {
	instance: Option<std::thread::JoinHandle<PluginData>>
}

impl PluginInstance {

	fn new(vtable: PluginVTable) -> Self {

		let instance = std::thread::spawn(move||unsafe {
			PluginInstance::instance_thread(vtable)
		});
		
		Self { instance: Some(instance) }
	}

	unsafe fn instance_thread(vtable: PluginVTable) -> PluginData {

		let ptr = (vtable.construct)();
		let result = (vtable.load)(ptr) as *mut (dyn Any + Send);

		if !result.is_null() {
			std::panic::resume_unwind(Box::from_raw(result))
		}

		std::thread::park();
		
		if result.is_null() {
			match (vtable.unload)(ptr).is_null() {
				true => {},
				false => println!("Failed to unload plugin"),
			}
		}

		(vtable.destruct)(ptr);

		PluginData
	}
}

impl From<PluginVTable> for PluginInstance {
    fn from(vtable: PluginVTable) -> Self {
		PluginInstance::new(vtable)
    }
}

impl Drop for PluginInstance {
    fn drop(&mut self) {
		if let Some(instance) = self.instance.take() {
			instance.thread().unpark();
			match instance.join() {
				Ok(data) => println!("Plugin ended succesfully {:?}", data),
				Err(e) => println!("Plugin ended with an error {:?}", e.downcast::<&str>().unwrap_err().downcast::<&str>()		),
			}
		}
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
