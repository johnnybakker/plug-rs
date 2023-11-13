pub mod ffi;
pub mod runtime;

pub trait Plugin : Sized {
	fn load(&mut self);
	fn unload(&mut self);
}

pub trait Runtime : Sized {
	fn name(&self);
}