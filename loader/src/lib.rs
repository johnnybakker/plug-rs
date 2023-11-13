use std::ffi::CString;

use plug_plugin::ffi::{
	LibraryVTable, 
	PluginVTable
};

mod loader;

pub struct PluginLibrary {
	pub path: String,
	pub library: Box<()>,
	pub plugins: Vec<PluginVTable>,
}

impl Drop for PluginLibrary {
    fn drop(&mut self) {
        println!("Dropping {}", self.path);
		loader::unload_plugin_library(Box::into_raw(self.library));
    }
}

impl PluginLibrary {
	pub fn load<T: AsRef<str>>(path: T) -> Option<PluginLibrary> {
		loader::load_plugin_library(path)
			.and_then(|vtable|Some(vtable.into()))
	}
}

impl From<LibraryVTable> for PluginLibrary {
    fn from(vtable: LibraryVTable) -> Self {	
		unsafe {
			
			let path = CString::from_raw(vtable.path)
				.to_string_lossy()
				.into_owned();

			let library = Box::from_raw(vtable.ptr);
			
			let mut plugins = Vec::with_capacity(vtable.amount);
			for i in 0..vtable.amount as isize {
				let ptr  = vtable.plugins.offset(i);
				plugins.push(*ptr);
			}

			Self {
				path,
				library,
				plugins
			}

		}
    }
}