use crate::drivers::*;
use litaudio::Sample;
use portaudio_rs::stream as pa_stream;
use portaudio_rs::device as pa_device;


impl From<portaudio_rs::PaError> for DriverError {
	fn from(e: portaudio_rs::PaError) -> Self {
		DriverError::Other(e.to_string())
	}
}

impl From<portaudio_rs::PaError> for PlaybackError {
	fn from(e: portaudio_rs::PaError) -> Self {
		PlaybackError::Other(e.to_string())
	}
}

pub struct PortaudioDriver<'a, T: pa_stream::SampleType + Sample + 'a> {
	params: DriverParameters<()>,
	stream: Option<pa_stream::Stream<'a, T, T>>,
	tmp: usize
}

impl<'a, T> Driver<T> for PortaudioDriver<'a, T>
	where T: pa_stream::SampleType + Sample + 'a
{
	type DriverParams = ();

	fn create(params: DriverParameters<Self::DriverParams>) -> Result<Box<Self>, DriverError> {
		let mut ret = Box::new(Self { params, stream: None, tmp: 0});

		let out_idx = pa_device::get_default_output_index()
			.ok_or("Cant find a default output device.")?;

		let out_lat = pa_device::get_info(out_idx).map(|d| d.default_low_output_latency)
			.ok_or("Cant find default output latency.")?;

		let output = portaudio_rs::stream::StreamParameters {
			device: out_idx,
			channel_count: ret.get_params().get_channel_count(),
			suggested_latency: out_lat,
			data: T::default()
		};

		let sr = ret.get_params().get_sample_rate() as f64;
		portaudio_rs::stream::is_format_supported::<i16, _>(None, Some(output), sr)?;

		let mut driver_instance = &mut *ret as *mut Self;
		let callback = Box::new(move |input: &[T], output: &mut [T], time: pa_stream::StreamTimeInfo, flags: pa_stream::StreamCallbackFlags| -> pa_stream::StreamCallbackResult {
			// stream is destroyed along with the object thus callback remains valid
			unsafe { (*driver_instance).callback(input, output, time, flags) }
		});

		let stream = portaudio_rs::stream::Stream::<_, _>::open(
			None,
			Some(output),
			sr,
			portaudio_rs::stream::FRAMES_PER_BUFFER_UNSPECIFIED,
			portaudio_rs::stream::StreamFlags::empty(),
			Some(callback)
		)?;
		ret.stream = Some(stream);

		Ok(ret)
	}

	fn get_params(&self) -> &DriverParameters<Self::DriverParams> { &self.params }
}

impl<'a, T> PortaudioDriver<'a, T>
	where T: pa_stream::SampleType + Sample + 'a
{
	fn callback(&mut self, input: &[T], output: &mut [T], time: pa_stream::StreamTimeInfo, flags: pa_stream::StreamCallbackFlags)
		-> pa_stream::StreamCallbackResult {
		let delta  = (std::f32::consts::PI * 2.) / (self.params.get_sample_rate() as f32 / 130.813);
		let max: f32 = num_traits::cast(T::max_val()).unwrap();

		for x in output {
			self.tmp += 1;
			let val = f32::cos(delta * self.tmp as f32) * max;
			*x = num_traits::cast(val).unwrap();
		}
		pa_stream::StreamCallbackResult::Continue
	}
}

impl<'a, T> PlaybackObserver for PortaudioDriver<'a, T>
	where T: pa_stream::SampleType + Sample + 'a
{
	fn on_start(&mut self) -> Result<(), PlaybackError> {
		self.stream.as_ref().unwrap().start()?;
		Ok(())
	}

	fn on_pause(&mut self) -> Result<(), PlaybackError> {
		self.stream.as_ref().unwrap().stop()?;
		Ok(())
	}

	fn on_seek(&mut self, p: f32) -> Result<(), PlaybackError> {
		Ok(())
	}

	fn on_reset(&mut self) -> Result<(), PlaybackError> {
		Ok(())
	}

	fn on_stop(&mut self) -> Result<(), PlaybackError> {
		self.stream.as_ref().unwrap().stop()?;
		Ok(())
	}
}