use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use hound::WavReader;
use std::sync::{Arc, Mutex};

pub struct AudioEngine {
    device: cpal::Device,
    config: cpal::SupportedStreamConfig,
    stream: Option<cpal::Stream>,
}

impl AudioEngine {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device");
        let config = device.default_output_config().unwrap();
        Self {
            device,
            config,
            stream: None,
        }
    }

    pub fn play_sound(&mut self, path: &str) {
        // drop whatever was playing before
        self.stream = None;

        let mut reader = WavReader::open(path).unwrap();
        let spec = reader.spec();
        println!("WAV spec: {:?}", spec);
        println!("CPAL config: {:?}", self.config);
        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Int => reader
                .samples::<i32>()
                .map(|s| {
                    let s = s.unwrap();
                    let max = (1i64 << (spec.bits_per_sample - 1)) as f32;
                    s as f32 / max
                })
                .collect(),
            hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
        };

        let samples = Arc::new(samples);
        let samples_clone = samples.clone();
        let pos = Arc::new(Mutex::new(0usize));
        let pos_clone = pos.clone();
        let channels = self.config.channels() as usize;
        let wav_channels = spec.channels as usize;

        let stream = self
            .device
            .build_output_stream(
                self.config.clone().into(),
                move |data: &mut [f32], _| {
                    let mut pos = pos_clone.lock().unwrap();
                    for frame in data.chunks_mut(channels) {
                        for ch in 0..channels {
                            let wav_ch = ch.min(wav_channels - 1);
                            let idx = *pos * wav_channels + wav_ch;
                            frame[ch] = if idx < samples_clone.len() {
                                samples_clone[idx]
                            } else {
                                0.0
                            };
                        }
                        *pos += 1;
                    }
                },
                |err| eprintln!("stream error: {err}"),
                None,
            )
            .unwrap();

        stream.play().unwrap();
        println!("audio shouldve plyyed");
        self.stream = Some(stream);
        println!(
            "stream stored, self.stream is Some: {}",
            self.stream.is_some()
        );
    }

    pub fn pause(&self) {
        if let Some(stream) = &self.stream {
            stream.pause().unwrap();
        }
    }

    pub fn resume(&self) {
        if let Some(stream) = &self.stream {
            stream.play().unwrap();
        }
    }
}
