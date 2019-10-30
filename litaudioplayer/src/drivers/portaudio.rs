use crate::drivers::*;
use litaudio::{Sample, AudioDeinterleaved};
use portaudio_rs::stream as pa_stream;
use portaudio_rs::device as pa_device;
use crate::providers::{Provider, ProcessingFlags};
use litcontainers::{Dynamic, StorageConstructor, Size, SliceableMut, Storage};


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
    provider: Option<Box<dyn Provider<T>>>,
    buffer: AudioDeinterleaved<T, Dynamic, Dynamic>,
    swap: AudioDeinterleaved<T, Dynamic, Dynamic>,
    out_size: usize,
}

impl<'a, T> Driver<T> for PortaudioDriver<'a, T>
    where T: pa_stream::SampleType + Sample + 'a
{
    type DriverParams = ();

    fn create(params: DriverParameters<Self::DriverParams>) -> Result<Box<Self>, DriverError> {
        let buffer_size = Size::new(
            Dynamic::new(params.get_channel_count() as usize),
            Dynamic::new(params.get_buffer_size() as usize),
        );

        let mut ret = Box::new(Self {
            params,
            stream: None,
            provider: None,
            buffer: AudioDeinterleaved::zeros(buffer_size.clone()),
            swap: AudioDeinterleaved::zeros(buffer_size),
            out_size: 0,
        });

        let out_idx = pa_device::get_default_output_index()
            .ok_or("Cant find a default output device.")?;

        let out_lat = pa_device::get_info(out_idx).map(|d| d.default_low_output_latency * 2)
            .ok_or("Cant find default output latency.")?;

        let output = portaudio_rs::stream::StreamParameters {
            device: out_idx,
            channel_count: ret.get_params().get_channel_count(),
            suggested_latency: out_lat,
            data: T::default(),
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
            ret.params.get_buffer_size(),
            portaudio_rs::stream::StreamFlags::empty(),
            Some(callback),
        )?;
        ret.stream = Some(stream);

        Ok(ret)
    }

    fn set_provider(&mut self, provider: Box<Provider<T>>) { self.provider = Some(provider) }

    fn get_params(&self) -> &DriverParameters<Self::DriverParams> { &self.params }
}

impl<'a, T> PortaudioDriver<'a, T>
    where T: pa_stream::SampleType + Sample + 'a
{
    fn callback(&mut self, input: &[T], output: &mut [T], time: pa_stream::StreamTimeInfo, flags: pa_stream::StreamCallbackFlags)
                -> pa_stream::StreamCallbackResult
    {
        match &mut self.provider {
            None => pa_stream::StreamCallbackResult::Complete,
            Some(provider) => {
                let mut buffer = self.buffer.into_slice_mut();
                let mut swap = self.swap.into_slice_mut();

                provider.request(
                    &mut buffer,
                    &mut swap,
                    self.params.get_buffer_size() as usize,
                    &mut self.out_size,
                    ProcessingFlags::ALL,
                );

                let interleaved_iter = output.iter_mut().zip(buffer.as_col_iter());
                for (o, i) in interleaved_iter {
                    *o = *i;
                }

                match self.out_size {
                    0 => pa_stream::StreamCallbackResult::Complete,
                    _ => pa_stream::StreamCallbackResult::Continue
                }
            }
        }
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