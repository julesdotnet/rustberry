use eframe::egui;

pub struct PatternEditor {
    tracks: i32,

}

impl PatternEditor {
    pub fn show() {
        let pattern_editor = egui::Window::new("Pattern Editor").constrain(true);
                pattern_editor.show(ctx, |ui| {
                    ui.horizontal(|ui|{
                        ui.label("Kick 1");
                        ui.separator();
                        ui.label("Adjust volume");
                    ui.add(egui::DragValue::new(&mut 0)
                        .speed(0.1)
                        .clamp_range(-50..=50)
                        .suffix(" db")
                    );

                    if ui.button("    ").clicked() {
                    
                    }
                    if ui.button("    ").clicked() {
                    
                    }
                    if ui.button("    ").clicked() {
                        
                    }
                    if ui.button("    ").clicked() {
                        
                    }
                    if ui.button("    ").clicked() {
                    
                    }
                    if ui.button("    ").clicked() {
                    
                    }
                    if ui.button("    ").clicked() {
                        
                    }
                    if ui.button("    ").clicked() {
                        
                    }
                    if ui.button("    ").clicked() {
                    
                    }
                    if ui.button("    ").clicked() {
                    
                    }
                    if ui.button("    ").clicked() {
                        
                    }
                    if ui.button("    ").clicked() {
                        
                    }
                    if ui.button("    ").clicked() {
                    
                    }
                    if ui.button("    ").clicked() {
                    
                    }
                    if ui.button("    ").clicked() {
                        
                    }
                    if ui.button("    ").clicked() {                    
                }
        });                
    });}
}