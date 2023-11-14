use std::path::PathBuf;

use cbindgen::Config;


fn main() {

	let package_name = std::env::var("CARGO_PKG_NAME").unwrap();
	let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	//let source_dir = manifest_dir.join("src");

		//let output_dir = PathBuf::from(output_dir);

    let output_file = manifest_dir.join(format!("{}.hpp", package_name));

    let mut config = Config::default();
	config.export.include.push("PluginVTable".to_owned());
	config.export.include.push("RuntimeVTable".to_owned());
	config.export.include.push("PluginVTableArray".to_owned());

    cbindgen::generate_with_config(&manifest_dir, config).unwrap().write_to_file(&output_file);
}