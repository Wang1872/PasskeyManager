use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 500.0])
            .with_min_inner_size([500.0, 350.0]),
        ..Default::default()
    };

    eframe::run_native(
        "PasskeyManager",
        options,
        Box::new(|_cc| Ok(Box::new(PasskeyManagerApp::default()))),
    )
}

// Speichert den Zustand unserer Oberfläche.
#[derive(Default)]
struct PasskeyManagerApp {
    button_clicked: bool,
}

impl eframe::App for PasskeyManagerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("PasskeyManager");
            ui.label("A learning project about encryption and passkeys.");

            ui.separator();
            ui.add_space(15.0);

            ui.label("Welcome to your first Rust desktop window!");

            if ui.button("Test button").clicked() {
                self.button_clicked = true;
            }

            if self.button_clicked {
                ui.label("The button works!");
            }
        });
    }
}
