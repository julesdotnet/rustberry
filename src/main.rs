use crate::{core::{audio_player::AudioEngine, sample_map::init_sample_map}, gui::{pattern_editor::PatternEditor, song_editor::SongEditor}};
use crate::core::application_state::app_state;
use eframe::{App, egui};
use std::sync::{Arc, Mutex};
mod core;
mod gui;

struct RustBerry {
    song_editor: SongEditor,
    pattern_editor: PatternEditor,
    audio: AudioEngine,
}

impl Default for RustBerry {
    fn default() -> Self {
        let bpm = 130;
        Self {
            song_editor: SongEditor::new(),
            pattern_editor: PatternEditor::new(),
            audio: AudioEngine::new(),
        }
    }
}

impl eframe::App for RustBerry {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut state = app_state().lock().unwrap();

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let label = if !state.song_playing { "Play" } else { "Stop" };
                if ui.button(label).clicked() {
                    state.song_playing = !state.song_playing;
                    if state.song_playing {
                        println!("tuff");
                    } else {
                        self.audio.pause();
                    }
                }
                ui.add(
                    egui::DragValue::new(&mut state.bpm)
                        .speed(0.2)
                        .range(1..=500)
                        .suffix(" bpm"),
                );
                if ui.button("Pattern Editor").clicked() {
                    state.show_pattern_editor = !state.show_pattern_editor;
                }
                if ui.button("Piano Roll").clicked() {}
            });
        });

        self.song_editor.show(ctx);

        egui::Window::new("Pattern Editor")
            .open(&mut state.show_pattern_editor)
            .show(ctx, |ui| {
                ui.add(&mut self.pattern_editor);
            });
    }

    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}
}


fn main() -> eframe::Result<()> {
    init_sample_map();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rustberry | v0.0.1",
        options,
        Box::new(|_cc| Ok(Box::new(RustBerry::default()) as Box<dyn eframe::App>)),
    )
}
