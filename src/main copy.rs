extern crate cpal;

mod math_nodes;
mod node;
mod noise;
mod types;
mod waves;
use std::io::prelude::*;
use std::{collections::HashMap, fs::File};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use types::AudioValue;

use crate::{node::Node, noise::RandomNoiseNode, waves::wave};

struct NodeRef<'a> {
  node: Box<&'a NodeAdd<'a>>,
  output: &'a str,
}

struct RefIn<'a> {
  nodeRef: Option<NodeRef<'a>>,
  default: AudioValue,
}

impl<'a> RefIn<'a> {
  pub fn new() -> RefIn<'a> {
    RefIn {
      default: 0.0,
      nodeRef: None,
    }
  }
}

trait NodeTestT<'a> {
  fn outputs(&mut self) -> &Outputs<'a>;
  fn inputs(&mut self) -> &Inputs<'a>;
  fn compute(&mut self) -> ();
}

type Inputs<'a> = HashMap<&'a str, RefIn<'a>>;

type Outputs<'a> = HashMap<&'a str, AudioValue>;

struct NodeAdd<'a> {
  _outputs: Outputs<'a>,
  _inputs: Inputs<'a>,
}

impl<'a> NodeAdd<'a> {
  pub fn new() -> NodeAdd<'a> {
    let mut inputs: Inputs = HashMap::with_capacity(2);
    inputs.insert("a", RefIn::new());
    inputs.insert("b", RefIn::new());
    let mut outputs: Outputs = HashMap::with_capacity(1);
    outputs.insert("out", 0.0);
    NodeAdd {
      _inputs: inputs,
      _outputs: outputs,
    }
  }
}

impl<'a> NodeTestT<'a> for NodeAdd<'a> {
  fn inputs(&mut self) -> &Inputs<'a> {
    &self._inputs
  }
  fn outputs(&mut self) -> &Outputs<'a> {
    &self._outputs
  }
  fn compute(&mut self) -> () {
    let val = get_input(self._inputs.get("a")) + get_input(self._inputs.get("b"));

    println!("compute: {}", val);
    self._outputs.insert("out", val);
  }
}

fn get_input<'a>(ref_in: Option<&RefIn<'a>>) -> AudioValue {
  let r = match ref_in {
    Some(v) => v,
    None => return 0.0,
  };
  println!("awd");
  match &r.nodeRef {
    Some(node) => match node.node._outputs.get(node.output) {
      Some(val) => {
        println!("awd2 ");
        val.clone()
      }
      None => r.default,
    },
    None => r.default,
  }
}

fn main() {
  let mut n1 = NodeAdd::new();

  let mut n2 = NodeAdd::new();

  let a = n1._inputs.get_mut("a");
  match a {
    Some(v) => {
      v.nodeRef = Some(NodeRef {
        node: Box::new(&n2),
        output: "out",
      })
    }
    None => (),
  }

  match n2._inputs.get_mut("a") {
    Some(v) => v.default = 5.0,
    None => (),
  }
  match n2._inputs.get_mut("b") {
    Some(v) => v.default = 10.0,
    None => (),
  }
  match n1._inputs.get_mut("b") {
    Some(v) => v.default = 10.0,
    None => (),
  }

  n2.compute();

  n1.compute();

  let res = n1._outputs.get("out");
  println!("{:?}", res)

  // let host = cpal::default_host();

  // let device = host
  //   .default_output_device()
  //   .expect("failed to find output device");
  // println!("Output device: {}", device.name().unwrap());

  // let config = device.default_output_config().unwrap();
  // println!("Default output config: {:?}", config);

  // match config.sample_format() {
  //   cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
  //   cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
  //   cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
  // }
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig)
where
  T: cpal::Sample,
{
  let sample_rate = config.sample_rate.0;
  let channels = config.channels as usize;

  // let arr = make_arr(sample_rate);
  const LENGTH: usize = 20_000;
  let mut arr = [0.0f32; LENGTH];
  let mut noise = RandomNoiseNode::new();
  for (i, elem) in arr.iter_mut().enumerate() {
    let sample_clock = (i + 1) % sample_rate as usize;
    *elem = noise.get_sample(sample_clock); //wave(waves::WaveType::Triangle, 80.0, sample_clock, sample_rate)
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
