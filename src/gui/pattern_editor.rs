use crate::core::{
    audioplayer::AudioEngine,
    track::PatternTrack,
};

use eframe::egui;
use egui::Color32;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct PatternEditor {
    pub tracks: Arc<Mutex<Vec<PatternTrack>>>,
    audio: Arc<AudioEngine>,
    step: usize,
    ms_per_step: f32,
    playing: bool,
    receiver: Option<std::sync::mpsc::Receiver<usize>>,
}

impl PatternEditor {
    pub fn new() -> Self {
        let bpm = 130;

        Self {
            tracks: Arc::new(Mutex::new(vec![
                PatternTrack::new(
                    String::from("Kick1"),
                    String::from(
                        "/home/julianrieder/Desktop/OwnCode/rustberry/rustberry/static/audio_samples/Kick1.wav",
                    ),
                ),
                PatternTrack::new(
                    String::from("Kick2"),
                    String::from(
                        "/home/julianrieder/Desktop/OwnCode/rustberry/rustberry/static/audio_samples/Zrimshot.wav",
                    ),
                ),
            ])),
            audio: Arc::new(AudioEngine::new()),
            step: 0,
            ms_per_step: 60000.0 / bpm as f32 / 4.0,
            playing: false,
            receiver: None,
        }
    }

    pub fn play_pattern(&mut self, ms_per_step: f32) {
        // Prevent multiple sequencer threads.
        if self.playing {
            return;
        }

        let (tx, rx) = std::sync::mpsc::channel::<usize>();

        self.receiver = Some(rx);
        self.playing = true;

        let step_duration = std::time::Duration::from_millis(ms_per_step as u64);

        let tracks = Arc::clone(&self.tracks);
        let audio = Arc::clone(&self.audio);

        thread::spawn(move || {
            let mut step = 0usize;
            let mut next_tick = std::time::Instant::now();

            loop {
                {
                    if let Ok(tracks) = tracks.lock() {
                        let paths: Vec<String> = tracks
                            .iter()
                            .filter(|t| t.steps[step])
                            .map(|t| t.path.clone())
                            .collect();

                        for path in paths {
                            audio.play_sound(&path);
                        }
                    }
                }

                if tx.send(step).is_err() {
                    break;
                }

                step = (step + 1) % 16;

                next_tick += step_duration;

                let now = std::time::Instant::now();

                if next_tick > now {
                    thread::sleep(next_tick - now);
                }
            }
        });
    }
}

impl Default for PatternEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl egui::Widget for &mut PatternEditor {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        if let Some(rx) = &self.receiver {
            while let Ok(step) = rx.try_recv() {
                self.step = step;
                ui.ctx().request_repaint();
            }
        }

        if ui.button("Play Pattern").clicked() {
            self.play_pattern(self.ms_per_step);
        }

        let audio = Arc::clone(&self.audio);

        ui.vertical(|ui| {
            let mut tracks = self.tracks.lock().unwrap();

            for pattern_track in tracks.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(&pattern_track.name);

                    ui.separator();

                    ui.label("Adjust volume");

                    ui.add(
                        egui::DragValue::new(&mut pattern_track.volume)
                            .speed(0.1)
                            .clamp_range(-50.0..=50.0)
                            .suffix(" dB"),
                    );

                    for i in 0..16 {
                        if i % 4 == 0 {
                            ui.separator();
                        }

                        let color = if i == self.step {
                            Color32::YELLOW
                        } else if pattern_track.steps[i] {
                            Color32::from_rgb(30, 200, 30)
                        } else {
                            Color32::from_rgb(10, 10, 10)
                        };

                        if ui
                            .add(egui::Button::new("   ").fill(color))
                            .clicked()
                        {
                            pattern_track.steps[i] = !pattern_track.steps[i];

                            if pattern_track.steps[i] {
                                audio.play_sound(&pattern_track.path);
                            }
                        }
                    }
                });
            }
        })
        .response
    }
}
