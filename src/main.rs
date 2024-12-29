fn tile() -> impl eframe::egui::Widget {
    move |ui: &mut eframe::egui::Ui| -> eframe::egui::Response {
        ui.allocate_ui(eframe::emath::vec2(50., 50.), |ui| {
            ui.vertical(|ui| {
                ui.label("row 1");
                ui.label("row 2");
                ui.label("row 3");
                ui.label("row 4");
                ui.label("row 5");
                ui.label("row 6");
                ui.label("row 7");
                ui.label("row 8");
                ui.label("row 9");
            });
        }).response
    }
}

fn main() -> eframe::Result {
    eframe::run_simple_native(
        "Horizontal Offset Bug Reproduction",
        eframe::NativeOptions::default(),
        |ctx, _frame| {
            eframe::egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(tile());
                    ui.add(tile());
                    ui.add(tile());
                    ui.add(tile());
                    ui.add(tile());
                    ui.add(tile());
                    ui.add(tile());
                    ui.add(tile());
                    ui.add(tile());
                    ui.add(tile());
                    ui.add(tile());
                });
            });
        },
    )
}
