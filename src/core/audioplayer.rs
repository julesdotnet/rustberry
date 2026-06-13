use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;

pub struct AudioEngine { 
    stream: cpal::Stream
}

impl AudioEngine {
    pub fn new(frequency: i32) -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device");
        let config = device.default_output_config().unwrap();
        let sample_rate = config.sample_rate() as f32;
        let channels = config.channels() as usize;

        let mut t = 0u64;

            let stream = device.build_output_stream(
            config.into(),
            move |data: &mut [f32], _| {
                for frame in data.chunks_mut(channels) {
                    let sample: f32 = (2.0 * PI * frequency as f32 * t as f32 / sample_rate).sin() * 0.3;
                    for ch in frame.iter_mut() {
                        *ch = sample;
                    }
                    t += 1;
                }
            },
            |err| eprintln!("audio error: {err}"),
            None,
        ).unwrap();

        Self { stream }
    }

        pub fn play(&self) {
        self.stream.play().unwrap();
    }

    pub fn pause(&self) {
        self.stream.pause().unwrap();
    }
}