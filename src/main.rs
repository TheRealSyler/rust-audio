extern crate cpal;

mod waves;

use std::fs::File;
use std::io::prelude::*;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::waves::wave;

fn main() {
  let host = cpal::default_host();

  let device = host
    .default_output_device()
    .expect("failed to find output device");
  println!("Output device: {}", device.name().unwrap());

  let config = device.default_output_config().unwrap();
  println!("Default output config: {:?}", config);

  match config.sample_format() {
    cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
    cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
    cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
  }
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig)
where
  T: cpal::Sample,
{
  let sample_rate = config.sample_rate.0 as f32;
  let channels = config.channels as usize;

  // let arr = make_arr(sample_rate);
  const LENGTH: usize = 20_000;
  let mut arr = [0.0f32; LENGTH];

  for (i, elem) in arr.iter_mut().enumerate() {
    let sample_clock = (i + 1) % sample_rate as usize;
    *elem = wave(waves::WaveType::Triangle, 80.0, sample_clock, sample_rate)
  }

  let mut file = File::create("js/src/data.data").expect("CANNOT CREATE");
  file
    .write(sample_rate.to_string().as_bytes())
    .expect("ERROR WRITE");
  file.write(b",").expect("ERROR WRITE");

  arr.iter().for_each(|v| {
    file.write(v.to_string().as_bytes()).expect("ERROR WRITE");
    file.write(b",").expect("ERROR WRITE");
  });

  let mut i = 0;
  let mut next_value = move || {
    i = (i + 1) % LENGTH;
    if i >= LENGTH {
      println!("ERR LENGTH")
    }
    return arr[i];
  };

  let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

  let stream = device
    .build_output_stream(
      config,
      move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
        write_data(data, channels, &mut next_value)
      },
      err_fn,
    )
    .unwrap();
  stream.play().unwrap();

  std::thread::sleep(std::time::Duration::from_millis(1000));
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
  T: cpal::Sample,
{
  for frame in output.chunks_mut(channels) {
    let value: T = cpal::Sample::from::<f32>(&next_sample());
    for sample in frame.iter_mut() {
      *sample = value;
    }
  }
}
