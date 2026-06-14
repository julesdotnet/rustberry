use crate::core::{
    audioplayer::AudioEngine,
    track::{self, PatternTrack},
};
use eframe::egui;
use egui::{Color32, WidgetType::Button};

pub struct PatternEditor {
    pub tracks: Vec<PatternTrack>,
    volume: f32,
    audio: AudioEngine,
}

impl PatternEditor {
    pub fn new() -> Self {
        Self {
            tracks: vec![
                PatternTrack::new(String::from("Kick1")),
                PatternTrack::new(String::from("Kick1")),
            ],
            volume: 0.0,
            audio: AudioEngine::new(),
        }
    }
}

impl egui::Widget for &mut PatternEditor {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            for pattern_track in &mut self.tracks {
                ui.horizontal(|ui| {
                    ui.label(&pattern_track.name);
                    ui.separator();
                    ui.label("Adjust volume");
                    ui.add(
                        egui::DragValue::new(&mut pattern_track.volume)
                            .speed(0.1)
                            .range(-50.0..=50.0)
                            .suffix(" db"),
                    );
                    for i in 0..16 {
                        if i % 4 == 0 {
                            ui.separator();
                        }
                        let color = if pattern_track.steps[i] { Color32::from_rgb(30, 200, 30)} else  { Color32::from_rgb(10, 10, 10)};
                        if ui.add(crate::egui::Button::new("   ").fill(color)).clicked() {
                            pattern_track.steps[i] = !pattern_track.steps[i];
                            if pattern_track.steps[i] {
                                self.audio.play_sound("/home/julianrieder/Desktop/OwnCode/rustberry/rustberry/static/audio_samples/Kick1.wav");
                            }

                        }
                    }
                });
            }
        })
        .response
    }
}
