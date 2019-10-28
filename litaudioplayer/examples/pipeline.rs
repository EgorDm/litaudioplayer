use litaudioplayer::drivers::*;
use bitflags::_core::time::Duration;
use litaudioplayer::providers::source_static::SourceStaticProvider;
use std::path::{PathBuf};
use litcontainers::*;
use litaudio::{AudioDeinterleaved, AudioStorage};


fn main() {
	portaudio_rs::initialize().unwrap();

	let in_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/test_audio.wav");
	let audio: AudioDeinterleaved<i16, U2, Dynamic> = litaudioio::read_audio(in_path.as_path()).unwrap();

	let params = DriverParameters::new(2, audio.sample_rate() as u32, 2048, ());
	let mut driver = PortaudioDriver::<i16>::create(params).unwrap();
	let audio_provider = SourceStaticProvider::new(audio);

	driver.set_provider(Box::new(audio_provider));

	println!("Start");
	driver.on_start().unwrap();
	std::thread::sleep(Duration::from_secs(5));
	println!("Stop");
}