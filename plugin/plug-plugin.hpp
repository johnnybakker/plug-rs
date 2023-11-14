#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct RuntimeVTable {
  void *ptr;
};

struct PluginVTable {
  void *(*construct)();
  void (*destruct)(void*);
  void (*load)(void*);
  void (*unload)(void*);
};

struct PluginVTableArray {
  uintptr_t length;
  PluginVTable *vtables;
};

extern "C" {

void __init_plugin_runtime(RuntimeVTable _vtable);

} // extern "C"
