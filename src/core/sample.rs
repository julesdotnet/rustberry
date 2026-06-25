use std::sync::Arc;
use hound::WavReader;

pub struct Sample {
    pub name: &'static str,
    pub data: Arc<Vec<f32>>,
    pub channels: usize
}

impl Sample {
    pub fn new(name: &'static str, path: &str) -> Self {
        let mut reader = WavReader::open(path)
            .unwrap_or_else(|e| panic!("Failed to open {path}: {e}"));

        let spec = reader.spec();

        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Int => {
                let max =
                    ((1i64 << (spec.bits_per_sample - 1)) - 1) as f32;

                reader
                    .samples::<i32>()
                    .map(|s| s.unwrap() as f32 / max)
                    .collect()
            }

            hound::SampleFormat::Float => reader
                .samples::<f32>()
                .map(|s| s.unwrap())
                .collect(),
        };
        Self {
            name: name,
            data: Arc::new(samples),
            channels: spec.channels as usize
        }
    }
}
