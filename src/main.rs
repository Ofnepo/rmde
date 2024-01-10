pub mod my_lib;
pub mod core;
mod ui;

use eframe::egui::{self};
use my_lib::md_to_frame;
use crate::core::{RayFile, RayFolder, MyApp};
use crate::ui::{editor_with_title_show, TreeViewNode, TreeViewLeaf, tree_view_show};

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
                if let Some(file) = tree_view_show(ui, &self.folder).clicked_leaf{
                    self.file = file;
                    self.parse_md();
                }
                
                ui.vertical(|ui| {
                    md_to_frame(ui, &self.md);
                });
                ui.vertical(|ui| {
                    if editor_with_title_show(ui, &self.file.name, &mut self.file.buf, &mut self.file.is_modified).changed() {
                        self.parse_md();
                    }
                });
            });
        });
        ctx.input(|i| {
            if i.viewport().close_requested() {
                self.save_state();
            }
        });
    }
}


impl TreeViewNode<RayFile> for &RayFolder
{
    fn title(&self) -> &str {
        &self.name
    }

    fn children(&self) -> Vec<Self> {
        self.folders.iter().collect()
    }

    fn leaves(&self) -> Vec<RayFile> {
        self.files.iter().map(|name|RayFile::new(name.clone())).collect()
    }
}

impl TreeViewLeaf for RayFile {
    fn title(&self) -> &str {
        &self.name
    }
}