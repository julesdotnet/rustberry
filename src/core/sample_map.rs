use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;
use crate::core::sample::Sample;

static SAMPLE_MAP: OnceLock<Arc<SampleMap>> = OnceLock::new();

pub fn init_sample_map() {
    print!("Sample map init!");
    if SAMPLE_MAP
        .set(Arc::new(SampleMap::load_default())).is_err() {
            panic!("i am panicking and it is because the of the sample map load default erroring in init sample map")
        }
}

pub fn sample_map() -> &'static Arc<SampleMap> {
    SAMPLE_MAP
        .get()
        .expect("sample bank not initialized — call init_sample_bank() first")
}

pub struct SampleMap {
    samples: HashMap<&'static str, Arc<Sample>>
}

impl SampleMap {
    pub fn load_default() -> Self{
        let mut samples = HashMap::new();

        samples.insert("kick1", Arc::new(Sample::new("Kick1", "/home/julianrieder/Desktop/OwnCode/rustberry/rustberry/static/audio_samples/Kick1.wav")));

        samples.insert("zrimshot", Arc::new(Sample::new("Zrimshot", "/home/julianrieder/Desktop/OwnCode/rustberry/rustberry/static/audio_samples/Zrimshot.wav")));

        Self {
            samples
        }
    }

    pub fn get(&self, name: &str) -> Option<Arc<Sample>> {
        self.samples.get(name).cloned()
    }
}
