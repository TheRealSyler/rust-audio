pub enum WaveType {
  Sine,
  Saw,
  SawReversed,
  Square,
  Triangle,
}

pub fn wave(wave_type: WaveType, frequency: f32, sample_clock: usize, sample_rate: f32) -> f32 {
  let phase = get_phase(frequency, sample_clock, sample_rate);
  match wave_type {
    WaveType::Sine => sine(phase),
    WaveType::Saw => saw(phase),
    WaveType::SawReversed => saw_reversed(phase),
    WaveType::Square => square(phase),
    WaveType::Triangle => triangle(phase),
  }
}

fn get_phase(frequency: f32, sample_clock: usize, sample_rate: f32) -> f32 {
  (frequency * (sample_clock as f32) / sample_rate) % 1.0
}

fn sine(phase: f32) -> f32 {
  sin(phase * 2.0 * std::f32::consts::PI)
}
fn saw(phase: f32) -> f32 {
  1.0 - phase
}
fn saw_reversed(phase: f32) -> f32 {
  phase
}

fn square(phase: f32) -> f32 {
  if sine(phase) > 0.0 {
    1.0
  } else {
    -1.0
  }
}

fn triangle(phase: f32) -> f32 {
  1.0 - (phase - 0.5).abs() * 4.0
}

#[inline]
fn sin(val: f32) -> f32 {
  val.sin()
}
