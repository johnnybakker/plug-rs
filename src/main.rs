use std::time::Duration;
use plug_loader::PluginLibrary;

fn main() {

	// std::panic::set_hook(Box::new(|info|{
	// 	println!("Custom hook");
	// }));	

	

	let current_dir = std::env::current_dir()
		.expect("Current workdirecotry is not valid");

	let first_plugin_path = current_dir.join("plugins/first/target/debug/first.dll");
	let first_plugin_path = String::from(first_plugin_path.to_string_lossy());

	let mut library = PluginLibrary::new(first_plugin_path);


	library.load();

	//loop {
		std::thread::sleep(Duration::from_secs(1));
		library.unload();
		std::thread::sleep(Duration::from_secs(1));
	//}
}