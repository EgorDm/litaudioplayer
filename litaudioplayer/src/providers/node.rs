use litcontainers::*;
use litaudio::{AudioDeinterleaved, Sample};

pub enum ProcessingFlags {
	ALL
}

/// Deinterleaved Audio Buffer used for
pub type AudioBuffer<'a, T> = SliceMut<'a, T, Dynamic, Dynamic, Dynamic, U1>;

pub trait Provider<T: Sample> {
	/// # Arguments
	/// * in_size: requested sample count
	fn request(
		&mut self,
		buf: &mut AudioBuffer<T>,
		swap: &mut AudioBuffer<T>,
		in_size: usize,
		out_size: &mut usize,
		flags: ProcessingFlags,
	);

	fn cursor(&self) -> usize;

	fn sample_count(&self) -> Option<usize>;

	fn sample_rate(&self) -> f32;

	fn ended(&self) -> bool {
		match self.sample_count() {
			Some(s) => self.cursor() >= s,
			None => false
		}
	}
}