use crate::core::sample::{Sample};
use crate::core::sample_map::sample_map;
use crate::core::{
    audioplayer::AudioEngine,
    track::PatternTrack,
};

use eframe::egui;
use egui::Color32;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(PartialEq)]
enum PlayState {
    Playing,
    Stopped,
    Paused
}

pub struct PatternEditor {
    pub tracks: Arc<Mutex<Vec<PatternTrack>>>,
    audio: Arc<AudioEngine>,
    step: usize,
    ms_per_step: f32,
    state: PlayState,
    receiver: Option<std::sync::mpsc::Receiver<usize>>,
    stop_flag: Arc<AtomicBool>
}

impl PatternEditor {
    pub fn new() -> Self {
        let bpm = 130;

        Self {
            tracks: Arc::new(Mutex::new(vec![
                PatternTrack::new(
                    String::from("Kick1"),
                    sample_map().get("kick1").unwrap(),
                ),
                PatternTrack::new(
                    String::from("Kick2"),
                    sample_map().get("zrimshot").unwrap(),
                ),
            ])),
            audio: Arc::new(AudioEngine::new()),
            step: 0,
            ms_per_step: 60000.0 / bpm as f32 / 4.0,
            state: PlayState::Stopped,
            receiver: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn play_pattern(&mut self, ms_per_step: f32) {
        let stop_flag = Arc::clone(&self.stop_flag);
        stop_flag.store(false, Ordering::SeqCst);

        if self.state == PlayState::Playing {
            return;
        }

        let (tx, rx) = std::sync::mpsc::channel::<usize>();

        self.receiver = Some(rx);
        self.state = PlayState::Playing;

        let step_duration = std::time::Duration::from_millis(ms_per_step as u64);

        let tracks = Arc::clone(&self.tracks);
        let audio = Arc::clone(&self.audio);

        thread::spawn(move || {
            let mut step = 0usize;
            let mut next_tick = std::time::Instant::now();

            loop {
                {
                    if let Ok(tracks) = tracks.lock() {
                        let paths: Vec<Arc<Sample>> = tracks
                            .iter()
                            .filter(|t| t.steps[step])
                            .map(|t| Arc::clone(&t.sample))
                            .collect();

                        for sample in paths {
                            audio.play_sound(sample);

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

    pub fn pause_pattern(&mut self) {
        if self.state == PlayState::Paused {
            return;
        }
        self.state = PlayState::Paused;
        self.audio.pause();
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
            if self.state != PlayState::Playing {
                self.play_pattern(self.ms_per_step);
            } else {
                self.state = PlayState::Paused;
                self.pause_pattern();
            }

        }

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
                            .range(-50.0..=50.0)
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
                                pattern_track.play();
                            }
                        }
                    }
                });
            }
        })
        .response
    }
}
