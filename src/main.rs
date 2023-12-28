use std::{fs::{File, OpenOptions}, io::{Read, Write}};

use eframe::egui;


struct RayFile{
    path: String,
    origin: String,
}

impl Default for RayFile {
    fn default() -> Self {
        Self {
            path: String::new(),
            origin: String::new(),
        }
    }
}

impl RayFile {
    fn new(path: String,) -> Self {
        match path {
            c if c.is_empty() => Self::default(),
            _ => {
                let mut origin = String::new();
                let mut file =  File::open(&path).ok().unwrap();
                file.read_to_string(&mut origin).ok();
                Self {
                    path: path,
                    origin: origin,
                }

            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    //let mut file = RayFile::new("README.md".into());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| {
            // This gives us image support:

            Box::<MyApp>::default()
        }),
    )
}


struct MyApp {
    current: RayFile,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current: RayFile::default()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui|{
                if ui.button("Save").clicked() {
                    let mut save_file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(false)
                        .open(&self.current.path)
                        .unwrap();
                    save_file.write_all(&self.current.origin.as_bytes()).ok();
                }
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.current = RayFile::new(path.display().to_string());
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.text_edit_multiline(&mut self.current.origin);
            });
            

            
        });
    }
}