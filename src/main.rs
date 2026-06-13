use eframe::egui;
use egui::accesskit::Role::Audio;
use crate::{core::audioplayer::AudioEngine, gui::pattern_editor::PatternEditor};
mod gui;
mod core;

struct RustBerry {
    song_playing: bool,
    bpm: i32,
    pattern_editor: PatternEditor,
    show_pattern_editor: bool,
    audio: AudioEngine
}

impl Default for RustBerry {
    fn default() -> Self {
        Self {
            song_playing: false,
            bpm: 130,
            pattern_editor: PatternEditor::new(),
            show_pattern_editor: false,
            audio: AudioEngine::new(404)
        }
    }
}

impl eframe::App for RustBerry {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let label = if !self.song_playing { "Play" } else { "Stop" };
                if ui.button(label).clicked() {
                    self.song_playing = !self.song_playing;
                    if(self.song_playing) {
                        self.audio.play();
                    } else {
                        self.audio.pause();
                    }
                }
                ui.add(
                    egui::DragValue::new(&mut self.bpm)
                        .speed(0.2)
                        .range(1..=500)
                        .suffix(" bpm"),
                );
                if ui.button("Pattern Editor").clicked() {
                    self.show_pattern_editor = !self.show_pattern_editor;
                }
                if ui.button("Piano Roll").clicked() {}
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {});

        egui::Window::new("Pattern Editor")
            .open(&mut self.show_pattern_editor)
            .show(ctx, |ui| {
                ui.add(&mut self.pattern_editor);
            });
    }

    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rustberry | v0.0.1",
        options,
        Box::new(|_cc| Ok(Box::new(RustBerry::default()) as Box<dyn eframe::App>)),
    )
}