use crate::{
  node::{Inputs, Node},
  types::AudioValue,
};

pub struct RandomNoiseNode {
  noise: oorandom::Rand32,
  inputs: Inputs,
}

impl Node for RandomNoiseNode {
  fn get_inputs(&self) -> &Inputs {
    &self.inputs
  }
  fn get_sample(&mut self, _: usize) -> AudioValue {
    (self.noise.rand_float() * 2.0) - 1.0 as AudioValue
  }
}

impl RandomNoiseNode {
  pub fn new() -> RandomNoiseNode {
    RandomNoiseNode {
      inputs: 2.0,
      noise: oorandom::Rand32::new(0),
    }
  }
}
