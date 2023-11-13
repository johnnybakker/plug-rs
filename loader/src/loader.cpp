#include "loader.hpp"

using namespace std;
namespace fs = boost::filesystem;
namespace dll = boost::dll;

extern "C" LibraryVTable* load_plugin_library(RuntimeVTable runtime, const char* path) {
	fs::path p(path);

	if(!fs::exists(p) || !fs::is_regular_file(p)) 
		return nullptr;

	dll::library_info info(p);

	const char* PLUGIN_LIBRARY_VTABLE_SECTION = ".vtable";
	const char* INIT_PLUGIN_RUNTIME_SYMBOL = "__init_plugin_runtime";

	auto sections = info.sections();

	for(auto symbol : sections)
		cout << "Found: " << symbol << endl;


	auto symbols = info.symbols(PLUGIN_LIBRARY_VTABLE_SECTION);
	cout << "Found " << symbols.size() << " symbols in plugin library vtable"  << endl;
	
	if(symbols.size() == 0) 
		return nullptr;

	for(auto symbol : symbols)
		cout << "Found: " << symbol << endl;

	auto init_runtime_pos = find(symbols.begin(), symbols.end(), INIT_PLUGIN_RUNTIME_SYMBOL);
	
	if(init_runtime_pos == symbols.end()) {
		cerr << "Failed to find init symbol: " << INIT_PLUGIN_RUNTIME_SYMBOL << endl;
		return nullptr;
	}

	// Remove the init runtime symbol from the symbols vector, 
	// so that we can treat this vector as the plugin init vector
	symbols.erase(init_runtime_pos);

	auto lib = new dll::shared_library(path);
	
	LibraryVTable* vtable = new LibraryVTable();
	vtable->ptr = lib;
	vtable->path = new char[strlen(path)];
	strcpy(vtable->path, path);
	vtable->amount = symbols.size();
	vtable->plugins = new PluginVTable[vtable->amount];
	
	// Initialize the library runtime
	auto init_runtime = lib->get<void(RuntimeVTable)>(INIT_PLUGIN_RUNTIME_SYMBOL);
	cout << "Initialize runtime" << endl;
	init_runtime(runtime);
	cout << "Runtime initialized" << endl;

	for(int i = 0; i < vtable->amount; i++) {
		auto symbol = symbols[i];
		cout << "Init vtable for plugin symbol: " << symbol << endl;
 		vtable->plugins[i] = lib->get<PluginVTable()>(symbol)();
	}

	return vtable;
}