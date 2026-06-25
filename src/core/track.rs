use crate::core::sample::Sample;
use std::sync::Arc;

pub struct PatternTrack {
    pub name: String,
    pub sample: Arc<Sample>,
    pub volume: f32,
    pub pan: f32,
    pub steps: Vec<bool>,
}

impl PatternTrack {
    pub fn new(name: String, sample: Arc<Sample>) -> Self {
        Self {
            name,
            sample,
            volume: 0f32,
            pan: 0f32,
            steps: vec![false; 16],
        }
    }

    pub fn play(&self) {}
}

pub struct LoopTrack {}
