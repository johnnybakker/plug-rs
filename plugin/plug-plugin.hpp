#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct RuntimeVTable {
  void *ptr;
};

struct PluginVTable {
  void *ptr;
  void (*load)(void*);
  void (*unload)(void*);
};

struct LibraryVTable {
  char *path;
  void *ptr;
  PluginVTable *plugins;
  uintptr_t amount;
};

extern "C" {

void __init_plugin_runtime(RuntimeVTable _vtable);

} // extern "C"
