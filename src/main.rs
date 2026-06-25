use crate::{core::{audioplayer::AudioEngine, sample_map::init_sample_map, }, gui::pattern_editor::PatternEditor};
use eframe::egui;
mod core;
mod gui;

struct RustBerry {
    song_playing: bool,
    bpm: i32,
    pub ms_per_beat: f32,
    pattern_editor: PatternEditor,
    show_pattern_editor: bool,
    audio: AudioEngine,
}

impl Default for RustBerry {
    fn default() -> Self {
        let bpm = 130;
        Self {
            song_playing: false,
            bpm: 130,
            ms_per_beat: 60000f32 / bpm as f32,
            pattern_editor: PatternEditor::new(),
            show_pattern_editor: false,
            audio: AudioEngine::new(),
        }
    }
}

impl eframe::App for RustBerry {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Panel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let label = if !self.song_playing { "Play" } else { "Stop" };
                if ui.button(label).clicked() {
                    self.song_playing = !self.song_playing;
                    if(self.song_playing) {

                        //jus play the song idk bru figure it out future juelz
                        println!("tuff");
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
    init_sample_map();


    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rustberry | v0.0.1",
        options,
        Box::new(|_cc| Ok(Box::new(RustBerry::default()) as Box<dyn eframe::App>)),
    )
}
