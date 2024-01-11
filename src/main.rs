mod core;
mod ui;

use eframe::egui::{self};
use crate::core::{RayFile, RayFolder, MyApp};
use crate::ui::{editor::editor_with_title_show, treeview::{TreeViewNode, TreeViewLeaf, tree_view_show}, mdview::md_view_show};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native("rmde", options, Box::new(|_| {
        let mut app = Box::<MyApp>::default();
        let _ = app.load_state();
        app
    }))
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    let _ = self.file.save();
                }
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.file = RayFile::new(&path);
                    }
                }
                if ui.button("Open Folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.folder = RayFolder::new(&path);
                    }
                }
            });
            ui.horizontal(|ui| {
                if let Some(file) = tree_view_show(ui, &self.folder).clicked_leaf{
                    self.file = file;
                    self.parse_md();
                }
                
                ui.vertical(|ui| {
                    md_view_show(ui, &self.md);
                });
                ui.vertical(|ui| {
                    if editor_with_title_show(ui, &self.file.name_or_default().to_owned() , &mut self.file.buf, &mut self.file.is_modified).changed() {
                        self.parse_md();
                    }
                });
            });
        });
        ctx.input(|i| {
            if i.viewport().close_requested() {
                let _ = self.save_state();
            }
        });
    }
}


impl TreeViewNode<RayFile> for &RayFolder
{
    fn title(&self) -> String {
        self.name_or_default()
    }

    fn children(&self) -> Vec<Self> {
        self.folders.iter().collect()
    }

    fn leaves(&self) -> Vec<RayFile> {
        self.files.iter().map(|path|RayFile::new(path)).collect()
    }
}

impl TreeViewLeaf for RayFile {
    fn title(&self) -> String {
        self.name_or_default()
    }
}