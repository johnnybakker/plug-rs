#pragma once

#include <map>
#include <memory>
#include <iostream>
#include <boost/dll.hpp>
#include <boost/system.hpp>
#include <boost/function.hpp>
#include <boost/filesystem.hpp>
#include <boost/filesystem/directory.hpp>
#include <plug-plugin.hpp>

extern "C" {
	LibraryVTable* load_plugin_library(RuntimeVTable runtime, const char* path);
	void unload_plugin_library(dll::shared_library* vtable);
}