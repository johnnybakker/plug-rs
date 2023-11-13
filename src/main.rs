use std::{io::Read, time::Duration};

use plug_loader::PluginLibrary;
use plug_plugin::{Plugin, ffi::PluginVTable};

pub struct PluginWrapper(PluginVTable);

impl Plugin for PluginWrapper {
    fn load(&mut self) {
        (self.0.load)(self.0.ptr);
    }

    fn unload(&mut self) {
        (self.0.unload)(self.0.ptr);
    }
}



fn main() {

	let current_dir = std::env::current_dir()
		.expect("Current workdirecotry is not valid");

	let first_plugin_path = current_dir.join("plugins/first/target/debug/first.dll");
	let first_plugin_path = String::from(first_plugin_path.to_string_lossy());

	if let Some(library) = PluginLibrary::load(first_plugin_path) {

		println!("Got library!");

		for plugin in library.plugins.iter() {

		}

	} else {
		println!("Cannot load library!");
	}

	loop {
		println!("Hello, world");
		std::thread::sleep(Duration::from_secs(1))
	}

}