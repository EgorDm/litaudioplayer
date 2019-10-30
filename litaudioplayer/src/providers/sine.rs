use crate::providers::{Provider, AudioBuffer, ProcessingFlags};
use std::marker::PhantomData;
use litaudio::{Sample, AudioStorageMut};
use num_traits::{Float, cast::cast};
use std::f32;
use litcontainers::StorageMut;

#[derive(Debug)]
pub struct Sine<T: Sample> {
    cursor: usize,
    _phantoms: PhantomData<T>,
}

impl<T: Sample> Sine<T> {
    pub fn new() -> Self {
        Self { cursor: 0, _phantoms: PhantomData }
    }
}

impl<T: Sample> Provider<T> for Sine<T> {
    fn request(
        &mut self,
        buf: &mut AudioBuffer<T>,
        swap: &mut AudioBuffer<T>,
        in_size: usize, out_size: &mut usize, flags: ProcessingFlags)
    {
        let delta = std::f32::consts::PI * 2. * 14000. / self.sample_rate();
        let min = cast::<T, f32>(T::min_val()).unwrap();
        let max = cast::<T, f32>(T::max_val()).unwrap();
        let mut channels: Vec<_> = buf.as_row_slice_iter_mut().collect();
        for i in 0..in_size {
            let val = ((self.cursor as f32 * delta).sin() + 1.) * max + min + 1.;
            for c in 0..channels.len() {
                channels[c][i] = cast::<f32, T>(val).unwrap();
            }


            self.cursor += 1;
        }
        *out_size = in_size;
    }

    fn cursor(&self) -> usize { self.cursor }

    fn sample_count(&self) -> Option<usize> { None }

    fn sample_rate(&self) -> f32 { 440100. }
}