use cargo_metadata::{MetadataCommand, CargoOpt};

const BOOST_DIR: &str = "C:\\Tools\\source\\boost";
const BOOST_LIB_DIR: &str = "C:\\Tools\\source\\boost\\stage\\lib";

fn main() {

	let metadata = MetadataCommand::new()
		.manifest_path("./Cargo.toml")
		.features(CargoOpt::AllFeatures)
		.exec()
		.unwrap();

	let plug_plugin = metadata.packages.iter()
		.find(|p|p.name.eq_ignore_ascii_case("plug-plugin"))
		.expect("Could not find metadata for plug-plugin package");

	let plug_plugin_manifest_dir = plug_plugin.manifest_path.as_path().parent()
		.expect("Failed to get manifest path of plug-plugin metadata");

	cc::Build::new()
		.cpp(true)
		.include("src")
		.include(BOOST_DIR)
		.include(plug_plugin_manifest_dir)
		.file("src/loader.cpp")
		// https://learn.microsoft.com/en-us/cpp/build/reference/eh-exception-handling-model?view=msvc-170
		.flag("/EHsc")
		.compile("loader");

	println!("cargo:rustc-link-search={}", BOOST_LIB_DIR);
	println!("cargo:rerun-if-changed=src/loader.cpp");
	println!("cargo:rerun-if-changed=src/loader.hpp");

}