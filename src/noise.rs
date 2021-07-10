pub fn noise(sample_clock: u64) -> f32 {
  let mut rng = oorandom::Rand32::new(sample_clock);
  rng.rand_float() * 2.0 - 1.0
}
