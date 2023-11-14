#pragma once

#include <map>
#include <memory>
#include <iostream>
#include <plug-plugin.hpp>
#include <boost/dll.hpp>
#include <boost/system.hpp>
#include <boost/function.hpp>
#include <boost/filesystem.hpp>
#include <boost/filesystem/directory.hpp>

extern "C" {

	void* plugin_library_load(const char*);
	void  plugin_library_unload(void* ptr);
	PluginVTableArray plugin_library_vtables(void* ptr);
}

boost::dll::shared_library* from_ptr(void* ptr);