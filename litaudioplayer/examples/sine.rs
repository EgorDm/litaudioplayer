use litaudioplayer::drivers::*;
use bitflags::_core::time::Duration;

fn main() {
	portaudio_rs::initialize().unwrap();

	let params = DriverParameters::new(2, 22050, 0, ());
	let mut driver = PortaudioDriver::<i16>::create(params).unwrap();

	println!("Start");
	driver.on_start().unwrap();
	std::thread::sleep(Duration::from_secs(5));
	println!("Stop");
}