pub mod my_lib;
pub mod core;

use eframe::egui::{self, Ui};
use my_lib::md_to_frame;
use crate::core::{RayFile, RayFolder, MyApp};

fn main() -> Result<(), eframe::Error> {
    //let mut file = RayFile::new("README.md".into());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native("rmde", options, Box::new(|_| Box::<MyApp>::default()))
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    self.file.save();
                }
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.file = RayFile::new(path.display().to_string());
                    }
                }
                if ui.button("Open Folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.folder = RayFolder::new(path.display().to_string());
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.collapsing(&self.folder.name, |ui| {
                    fn set_ui(ui: &mut Ui, folder: &RayFolder, c: &mut RayFile) {
                        for subfolder in &folder.folders {
                            ui.collapsing(&subfolder.name, |ui| {
                                set_ui( ui, subfolder, c);
                            });
                        }
                        for file in &folder.files {
                            if ui.small_button(file.split('/').last().unwrap()).clicked() {
                                let new = RayFile::new(file.clone());
                                c.name = new.name;
                                c.origin = new.origin;
                                c.buf = new.buf;
                                c.path = new.path;
                            }
                        }
                    }
                    set_ui(ui, &self.folder, &mut self.file)
                });
                ui.vertical(|ui| {
                    md_to_frame(ui, &self.md);
                });
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(&self.file.name);
                        if self.file.origin != self.file.buf {
                            let _ = ui.radio(true, "");
                            self.parse_md();
                        }
                    });
                    ui.code_editor(&mut self.file.buf);
                });
            });
            ui.label("hi");
        });
        ctx.input(|i| {
            if i.viewport().close_requested() {
                self.save_state();
            }
        });
    }
}
