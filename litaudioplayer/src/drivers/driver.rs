use litaudio::Sample;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum PlaybackError {
	Other(String)
}

impl From<&str> for PlaybackError {
	fn from(s: &str) -> Self { PlaybackError::Other(s.to_string()) }
}

impl error::Error for PlaybackError {

}

impl fmt::Display for PlaybackError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			PlaybackError::Other(s) => s,
		})
	}
}

pub trait PlaybackObserver {
	fn on_start(&mut self) -> Result<(), PlaybackError>;

	fn on_pause(&mut self) -> Result<(), PlaybackError>;

	fn on_seek(&mut self, p: f32) -> Result<(), PlaybackError>;

	fn on_reset(&mut self) -> Result<(), PlaybackError>;

	fn on_stop(&mut self) -> Result<(), PlaybackError>;
}


#[derive(Clone, new)]
pub struct DriverParameters<P: Clone> {
	channel_count: u32,
	sample_rate: u32,
	buffer_size: u64,
	driver_params: P,
}

impl<P: Clone> DriverParameters<P> {
	pub fn get_channel_count(&self) -> u32 { self.channel_count }

	pub fn get_sample_rate(&self) -> u32 { self.sample_rate }

	pub fn get_buffer_size(&self) -> u64 { self.buffer_size }

	pub fn get_driver_params(&self) -> &P { &self.driver_params }
}

#[derive(Debug, Clone)]
pub enum DriverError {
	Other(String)
}

impl From<&str> for DriverError {
	fn from(s: &str) -> Self { DriverError::Other(s.to_string()) }
}

impl error::Error for DriverError {

}

impl fmt::Display for DriverError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			DriverError::Other(s) => s,
		})
	}
}

pub trait Driver<T: Sample>: PlaybackObserver + Sized {
	type DriverParams: Clone;

	fn create(params: DriverParameters<Self::DriverParams>) -> Result<Box<Self>, DriverError>;

	fn get_params(&self) -> &DriverParameters<Self::DriverParams>;
}
