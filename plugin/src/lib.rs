use std::panic::{UnwindSafe, RefUnwindSafe};

pub mod ffi;
pub mod runtime;

pub trait Plugin : Sized + Default + UnwindSafe + RefUnwindSafe {
	fn load(&mut self);
	fn unload(&mut self);
}

pub trait Runtime : Sized {
	fn name(&self);
}