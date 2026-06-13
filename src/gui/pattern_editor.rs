use eframe::egui;
use crate::core::track::PatternTrack;

pub struct PatternEditor {
    pub tracks: Vec<PatternTrack>,
    volume: f32,
}

impl PatternEditor {
    pub fn new() -> Self {
        Self {
            tracks: vec![PatternTrack::new(String::from("Kick"))],
            volume: 0.0,
        }
    }
}

impl egui::Widget for &mut PatternEditor {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Kick 1");
                ui.separator();
                ui.label("Adjust volume");
                ui.add(
                    egui::DragValue::new(&mut self.volume)
                        .speed(0.1)
                        .range(-50.0..=50.0)
                        .suffix(" db"),
                );
                for i in 0..16 {
                    if i % 4 == 0 {
                        ui.separator();
                    }
                    if ui.button("  ").clicked() {}
                }
            });
        })
        .response
    }
}