use hound::{WavSpec, WavWriter};
use crate::core::track::{PatternTrack};

const SAMPLE_RATE: u32 = 44100;
const CHANNELS: usize = 2;

fn render_to_wav(tracks: Vec<PatternTrack>, path: &str) {

    let spec = WavSpec {
        channels: CHANNELS as u16,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float
    };

    let mut writer = WavWriter::create(path, spec);

    let mut song_end_beat = 0;

    let seconds_per_beat = 60.0 / bpm;
    let samples_per_step = (seconds_per_beat / 4.0 * sample_rate as f64) as usize; // 16th notes
    let total_samples = (total_beats as f64 * seconds_per_beat * sample_rate as f64) as usize;

    let mut left_buf = vec![0f32; total_samples];
    let mut right_buf = vec![0f32; total_samples];

    for track in &tracks {
        if track.start_beat + track.length_beats > song_end_beat {
            song_end_beat = track.start_beat + track.length_beats;
        }
    }

    let total_samples: f32 = song_end_beat as f32 * SAMPLE_RATE as f32;
    for track in tracks {
        for (step_idx, &active) in track.steps.iter().enumerate(){
            if !active {
                continue;
            }

            let step_sample = track_start_sample + step_idx * samples_per_step;
        }
    }
}

fn beat_to_sample(beat: usize, bpm: f64, sample_rate: u32) -> usize {
    let seconds_per_beat = 60.0 / bpm;
    (beat as f64 * seconds_per_beat * sample_rate as f64) as usize
}

fn pan_left(pan: f32) -> f32 {
    (1.0 - pan.max(0.0)).sqrt()
}

fn pan_right(pan: f32) -> f32 {
    (1.0 + pan.min(0.0)).sqrt()
}
