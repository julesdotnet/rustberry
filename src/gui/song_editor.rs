use egui::Panel;

use crate::core::{sample_map::{SampleMap, sample_map}, track::PatternTrack};

#[derive(PartialEq)]
enum Tab { PatternTracks, Samples }

pub struct SongEditor {
    pub lanes: Vec<Vec<PatternTrack>>,
    pub current_tab: Tab
}


impl SongEditor {
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                if ui.selectable_label(self.current_tab == Tab::PatternTracks, "Pattern Tracks").clicked() {
                    self.current_tab = Tab::PatternTracks;
                }
                if ui.selectable_label(self.current_tab == Tab::Samples, "Samples").clicked() {
                    self.current_tab = Tab::Samples;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::PatternTracks => {
                    let beat_width = 40.0;
                    let lane_height = 48.0;
                    for (i, lane) in self.lanes.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.set_min_height(lane_height);
                            ui.label(egui::RichText::new(format!("Lane {}", i + 1)).size(16.0).family(egui::FontFamily::Monospace));
                            let origin = ui.next_widget_position();
                            for track in lane {
                                let x = track.start_beat as f32 * beat_width;
                                let w = track.length_beats as f32 * beat_width;
                                let rect = egui::Rect::from_min_size(
                                    egui::pos2(origin.x + x, origin.y),
                                    egui::vec2(w, lane_height),
                                );
                                ui.painter().rect_filled(rect, 4.0, egui::Color32::from_rgb(80, 120, 200));
                                ui.painter().text(rect.min, egui::Align2::LEFT_TOP, track.sample.name, egui::FontId::default(), egui::Color32::WHITE);
                            }
                        });
                    }
                }
                Tab::Samples => { /* render sample browser */ }
            }
        });
    }
    pub fn new() -> Self {
        Self {
            lanes: vec![vec![PatternTrack::new(String::from(""), sample_map().get("zrimshot").unwrap())],
                vec![PatternTrack::new(String::from(""), sample_map().get("zrimshot").unwrap())]],
            current_tab: Tab::PatternTracks
        }
    }
}
