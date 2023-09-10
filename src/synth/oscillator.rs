use std::f64::consts::PI;

pub enum VcoWaveform { Sine }

pub struct Vco {
    pub waveform: VcoWaveform,
    pub frequency: f64,
    pub amplitude: f64,
}

impl Vco {
    pub fn generate_samples(&self, n: i32, sample_rate: i32) -> Vec<f64> {
        (0..n).map(|i| {
            let x = ((i as f64) * (self.frequency / (sample_rate as f64))).fract();
            (x * 2.0 * PI).sin() * self.amplitude
        }).collect()
    }
}