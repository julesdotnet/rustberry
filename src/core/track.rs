pub struct PatternTrack {
    name: String,
    volume: f32,
    pan: f32,
    steps: i32
}

impl PatternTrack {
    pub fn new(name: String) -> Self{
        Self {
            name,
            volume: 0f32,
            pan: 0f32,
            steps: 16
        }
    }
}

pub struct LoopTrack {

}