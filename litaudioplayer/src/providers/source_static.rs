use litaudio::*;
use litcontainers::*;
use crate::providers::node::{Provider, ProcessingFlags};
use std::marker::PhantomData;
use std::cmp::{min};
use crate::providers::AudioBuffer;

#[derive(Debug)]
pub struct SourceStaticProvider<T: Sample, R: Dim, C: Dim> {
	source: AudioDeinterleaved<T, R, C>,
	cursor: usize
}

impl<T, R, C> SourceStaticProvider<T, R, C>
	where T: Sample, R: Dim, C: Dim
{
	pub fn new(source: AudioDeinterleaved<T, R, C>) -> Self {
		Self {source, cursor: 0}
	}
}

impl<T, R, C> Provider<T> for SourceStaticProvider<T, R, C>
	where T: Sample, R: Dim, C: Dim {
	fn request(
		&mut self,
		buf: &mut AudioBuffer<T>,
		swap: &mut AudioBuffer<T>,
		in_size: usize, out_size: &mut usize, flags: ProcessingFlags)
	{
		let size = min(in_size, self.source.samples() - self.cursor);
		*out_size = size;
		self.cursor += size;

		if size > 0 {
			for c in 0..min(self.source.channels(), buf.channels()) {
				let source_data: _ = self.source.slice(c, self.cursor..self.cursor + size);
				buf.as_row_slice_mut(c).copy_from_slice(source_data.as_slice());
			}
		}
	}

	fn cursor(&self) -> usize { self.cursor }

	fn sample_count(&self) -> Option<usize> { Some(self.source.samples()) }

	fn sample_rate(&self) -> f32 { self.source.sample_rate() as f32 }
}