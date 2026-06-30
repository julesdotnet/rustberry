use hound::{WavSpec, WavWriter};
use crate::core::track::{PatternTrack};
use crate::core::application_state::app_state;

const SAMPLE_RATE: u32 = 44100;
const CHANNELS: usize = 2;

fn render_to_wav(tracks: Vec<PatternTrack>, path: &str) -> Result<(), hound::Error> {
    let bpm = app_state().lock().unwrap().bpm as f64;
    let seconds_per_beat = 60.0 / bpm;
    let samples_per_step = (seconds_per_beat / 4.0 * SAMPLE_RATE as f64) as usize;

    let mut song_end_beat = 0;
    for track in &tracks {
        if track.start_beat + track.length_beats > song_end_beat {
            song_end_beat = track.start_beat + track.length_beats;
        }
    }
    let total_samples = beat_to_sample(song_end_beat, bpm, SAMPLE_RATE);

    let mut left_buf = vec![0f32; total_samples];
    let mut right_buf = vec![0f32; total_samples];

    for track in &tracks {
        let track_start_sample = beat_to_sample(track.start_beat, bpm, SAMPLE_RATE);
        let l_gain = pan_left(track.pan) * track.volume;
        let r_gain = pan_right(track.pan) * track.volume;

        for (step_idx, &active) in track.steps.iter().enumerate() {
            if !active {
                continue;
            }
            let step_sample = track_start_sample + step_idx * samples_per_step;
            for (i, &s) in track.sample.data.iter().enumerate() {
                let pos = step_sample + i;
                if pos >= left_buf.len() {
                    break;
                }
                left_buf[pos] += s * l_gain;
                right_buf[pos] += s * r_gain;
            }
        }
    }

    let spec = WavSpec {
        channels: CHANNELS as u16,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = WavWriter::create(path, spec)?;
    for i in 0..total_samples {
        writer.write_sample(left_buf[i])?;
        writer.write_sample(right_buf[i])?;
    }
    writer.finalize()?;
    Ok(())
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
