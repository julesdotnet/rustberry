pub struct PatternTrack {
    pub name: String,
    pub volume: f32,
    pub pan: f32,
    pub steps: Vec<bool>,
}

impl PatternTrack {
    pub fn new(name: String) -> Self {
        Self {
            name,
            volume: 0f32,
            pan: 0f32,
            steps: vec![false; 16],
        }
    }

    pub fn play() {}
}

pub struct LoopTrack {}
