#include "loader.hpp"

using namespace std;

extern "C" void* plugin_library_load(const char* path)
{
	boost::filesystem::path p(path);

	if(!boost::filesystem::exists(p) || !boost::filesystem::is_regular_file(p)) 
		return nullptr;

	return new boost::dll::shared_library(path);
}

extern "C" void  plugin_library_unload(void* ptr)
{
	if(ptr != nullptr) delete from_ptr(ptr);
}

extern "C" PluginVTableArray plugin_library_vtables(void* ptr)
{
 	auto lib = from_ptr(ptr);

	boost::dll::library_info info(lib->location());

	const char* PLUGIN_LIBRARY_VTABLE_SECTION = ".vtable";
	const char* INIT_PLUGIN_RUNTIME_SYMBOL = "__init_plugin_runtime";

	// auto sections = info.sections();

	// for(auto symbol : sections)
	// 	cout << "Found: " << symbol << endl;

	auto symbols = info.symbols(PLUGIN_LIBRARY_VTABLE_SECTION);
	auto init_runtime_pos = find(symbols.begin(), symbols.end(), INIT_PLUGIN_RUNTIME_SYMBOL);
	
	if(init_runtime_pos != symbols.end()) {
		symbols.erase(init_runtime_pos);
	}

	PluginVTableArray arr;
	arr.length = symbols.size();
	arr.vtables = new PluginVTable[arr.length];
	
	for(int i = 0; i < arr.length; i++) {
 		arr.vtables[i] = lib->get<PluginVTable()>(symbols[i])();
	}

	return arr;
}


boost::dll::shared_library* from_ptr(void* ptr) {
	return (boost::dll::shared_library*)(ptr);
}