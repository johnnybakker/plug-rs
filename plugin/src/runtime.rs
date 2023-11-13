// use std::{mem::MaybeUninit, sync::Arc};

use crate::ffi::RuntimeVTable;


// pub struct Runtime(&'static mut RuntimeVTable);


// impl From<*mut RuntimeVTable> for Runtime {
//     fn from(value: *mut RuntimeVTable) -> Self {
//         unsafe { Runtime(&mut *value) }
//     }
// }

// static mut RUNTIME: MaybeUninit<Arc<Runtime>> = MaybeUninit::uninit();



#[no_mangle]
#[link_section = ".vtable"]
unsafe extern "C" fn __init_plugin_runtime(_vtable: RuntimeVTable) {
	println!("Init runtime for plugin")
	// let runtime = Runtime::from(vtable);
	// let runtime_ptr = Arc::new(runtime);
	// let _ = RUNTIME.write(runtime_ptr);
}

// pub fn current_runtime() -> Arc<Runtime> {
// 	// let runtime = unsafe {
// 	// 	RUNTIME.assume_init_read()
// 	// };

// 	// runtime.clone()
// }