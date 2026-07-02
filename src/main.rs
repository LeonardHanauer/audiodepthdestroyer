#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use hound;
use std::path::PathBuf;

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 240.0]) // wide enough for the drag-drop overlay text
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Audio 1bit quantizer",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    //dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Easily destroy an audio file with one button!");

            if ui.button("Open file…").clicked()
                && let Some(path) = rfd::FileDialog::new().pick_file()
            {
                self.picked_path = Some(path.display().to_string());

                let mut reader = match hound::WavReader::open(self.picked_path.as_ref().unwrap()){
                    Ok(r) => r,
                    Err(e) => {
                        ui.label("Couldn't read WAV file: {e}");
                        return;
                    }
                };
                let spec = reader.spec();

                let source_path = self.picked_path.as_ref().unwrap();
                let source_path = std::path::Path::new(source_path);
                
                let out_dir = source_path.parent().unwrap_or_else(|| std::path::Path::new("."));
                
                let random_name = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
                    .to_string();

                let out_path: PathBuf = out_dir.join(format!("{}.wav", random_name));

                let mut writer = hound::WavWriter::create(&out_path, spec).unwrap();
            
                //let bit_depth = reader.spec().bits_per_sample;
            
                for sample in reader.samples::<i16>() {
                    let sample = sample.unwrap();
                    let rounded: i16 = if sample >= 0 { i16::MAX } else { i16::MIN };
                    writer.write_sample(rounded).unwrap();
                    ui.label("Fetching Sample, quantizing it and appending to file");

                }

                writer.finalize();
                ui.label("Done!");


            }

            

            
            
        });
    }
}

