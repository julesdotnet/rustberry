use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use hound::WavReader;
use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapProd, HeapRb,
};
use std::sync::{Arc, Mutex};

use crate::core::sample::Sample;

pub struct Voice {
    pub samples: Arc<Vec<f32>>,
    pub pos: usize,
    pub channels: usize,
}

pub struct AudioEngine {
    _stream: cpal::Stream,
    producer: Mutex<HeapProd<Voice>>,
}

impl AudioEngine {
    pub fn new() -> Self {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("No output device available");

        let config = device
            .default_output_config()
            .expect("Failed to get default output config");

        let channels = config.channels() as usize;
        let stream_config: cpal::StreamConfig = config.into();

        let rb = HeapRb::<Voice>::new(64);
        let (producer, mut consumer) = rb.split();

        let mut active: Vec<Voice> = Vec::new();

        let stream = device
            .build_output_stream(
                stream_config,
                move |data: &mut [f32], _| {
                    while let Some(voice) = consumer.try_pop() {
                        active.push(voice);
                    }

                    for sample in data.iter_mut() {
                        *sample = 0.0;
                    }

                    for frame in data.chunks_mut(channels) {
                        for voice in active.iter_mut() {
                            for ch in 0..channels {
                                let wav_ch = ch.min(voice.channels.saturating_sub(1));

                                let idx =
                                    voice.pos * voice.channels + wav_ch;

                                if idx < voice.samples.len() {
                                    frame[ch] += voice.samples[idx];
                                }
                            }

                            voice.pos += 1;
                        }

                        active.retain(|voice| {
                            voice.pos * voice.channels
                                < voice.samples.len()
                        });
                    }
                },
                |err| {
                    eprintln!("Stream error: {err}");
                },
                None,
            )
            .unwrap();

        stream.play().unwrap();

        Self {
            _stream: stream,
            producer: Mutex::new(producer),
        }
    }

    pub fn play_sound(&self, sample: Arc<Sample>) {

        let voice = Voice {
            samples: sample.data.clone(),
            pos: 0,
            channels: sample.channels as usize,
        };

        if let Ok(mut producer) = self.producer.lock() {
            let _ = producer.try_push(voice);
        }
    }

    pub fn pause(&self) {
        self._stream.pause().unwrap();
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}
