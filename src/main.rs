use eframe::egui;

struct RustBerry {
    song_playing: bool,
    bpm: i32,

    show_pattern_editor: bool
}

impl Default for RustBerry {
    fn default() -> Self {
        Self {
            song_playing: false,
            bpm: 130,
            show_pattern_editor: false
        }
    }
}

impl eframe::App for RustBerry {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let label = if !self.song_playing {"Play"} else { "Stop" };
            ui.horizontal(|ui|{
                if ui.button(label).clicked() {
                    self.song_playing = !self.song_playing;
                }
                ui.add(egui::DragValue::new(&mut self.bpm)
                    .speed(0.2)
                    .clamp_range(1..=500)
                    .suffix(" bpm")
                );

                if ui.button("Pattern Editor").clicked() {
                    self.show_pattern_editor = !self.show_pattern_editor;
                }

                if ui.button("Piano Roll").clicked() {
                    
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rustberry | v0.0.1",
        options,
        Box::new(|_cc| Box::new(RustBerry::default()) as Box<dyn eframe::App>),
    )
}