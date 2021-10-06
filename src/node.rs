use crate::types::AudioValue;

pub trait Node {
  fn get_sample(&mut self, sample_pos: usize) -> AudioValue;
  fn get_inputs(&self) -> &Inputs;
}

struct NodeOutput<T> {
  value: T,
}

pub trait Input {
  fn name(&self) -> &String;
  fn value(&self) -> AudioValue;
}

pub struct InputF32 {
  value: f32,
  name: String,
}

impl Input for InputF32 {
  fn name(&self) -> &String {
    &self.name
  }
  fn value(&self) -> AudioValue {
    self.value
  }
}

pub type Inputs = f32;
