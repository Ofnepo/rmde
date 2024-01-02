mod my_lib;

use eframe::egui::{self, Ui};
use markdown::{
    mdast::{Node, Root},
    to_mdast, ParseOptions,
};
use my_lib::md_to_frame;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_dir, OpenOptions},
    io::{Read, Write},
};

#[derive(Debug, Deserialize, Serialize)]
struct RayFile {
    path: String,
    name: String,
    origin: String,
    buf: String,
}

impl Default for RayFile {
    fn default() -> Self {
        Self {
            path: String::new(),
            name: String::new(),
            origin: String::new(),
            buf: String::new(),
        }
    }
}

impl RayFile {
    pub fn new(path: String) -> Self {
        match path {
            c if c.is_empty() => Self::default(),
            _ => {
                let mut origin = String::new();
                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(false)
                    .open(&path)
                    .unwrap();
                file.read_to_string(&mut origin).ok();

                let name = path.split('/').last().unwrap().to_string();
                let buf = origin.clone();

                //println!("Path: {}\nOrigin: {}\nBuf: {}", &path, &origin, &buf);
                Self {
                    name,
                    path,
                    origin,
                    buf,
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct RayFolder {
    path: String,
    name: String,
    files: Vec<String>,
    folders: Vec<RayFolder>,
}

impl Default for RayFolder {
    fn default() -> Self {
        Self {
            name: String::new(),
            path: String::new(),
            files: Vec::new(),
            folders: Vec::new(),
        }
    }
}

impl RayFolder {
    fn new(path: String) -> Self {
        if path.is_empty() {
            Self::default()
        } else {
            let dir = read_dir(&path).ok().unwrap();
            let mut dirs: Vec<RayFolder> = Vec::new();
            let mut files: Vec<String> = Vec::new();
            dir.for_each(|a| {
                let i = a.ok().unwrap();

                if i.file_type().ok().unwrap().is_dir() {
                    dirs.push(RayFolder::new(i.path().to_str().unwrap().to_string()));
                } else {
                    files.push(i.path().to_str().unwrap().to_string());
                }
            });
            let name = path.split('/').last().unwrap().to_string();

            Self {
                name,
                path,
                files,
                folders: dirs,
            }
        }
    }
    fn _data(self) -> String {
        let mut file_temp = String::new();
        for i in &self.files {
            file_temp += &i.split('/').last().unwrap();
            file_temp += ", ";
        }
        let mut folder_temp = String::new();
        for dir in self.folders {
            for line in dir._data().split('\n') {
                folder_temp += "\n\t";
                folder_temp += line;
            }
        }

        "\nname: ".to_string()
            + &self.name
            + "\nfiles: "
            + &file_temp
            + "\nfolders: "
            + &folder_temp
    }
    fn set_ui(&self, ui: &mut Ui, c: &mut RayFile) {
        for folder in &self.folders {
            ui.collapsing(&folder.name, |ui| {
                folder.set_ui(ui, c);
            });
        }
        for file in &self.files {
            if ui.small_button(file.split('/').last().unwrap()).clicked() {
                let new = RayFile::new(file.clone());
                c.name = new.name;
                c.origin = new.origin;
                c.buf = new.buf;
                c.path = new.path;
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

    eframe::run_native("rmde", options, Box::new(|_| Box::<MyApp>::default()))
}

struct MyApp {
    file: RayFile,
    folder: RayFolder,
    md: Node,
}

impl Default for MyApp {
    fn default() -> Self {
        let state = RayFile::new(".state.ron".to_string()).origin;

        let mut res = Self {
            file: RayFile::default(),
            folder: RayFolder::default(),
            md: Node::Root(Root {
                children: Vec::new(),
                position: None,
            }),
        };
        if state.is_empty() {
            return res;
        }
        let state: (RayFile, RayFolder) = ron::from_str(&state).unwrap();
        res.file = state.0;
        res.folder = state.1;
        res.md = to_mdast(&res.file.buf, &ParseOptions::default()).unwrap();
        res
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    let mut save_file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(&self.file.path)
                        .unwrap();
                    self.file.origin = self.file.buf.clone();
                    save_file.write_all(self.file.origin.as_bytes()).ok();
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
                    self.folder.set_ui(ui, &mut self.file)
                });
                ui.vertical(|ui| {
                    md_to_frame(ui, &self.md);
                });
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(&self.file.name);
                        if self.file.origin != self.file.buf {
                            let _ = ui.radio(true, "");
                            self.md = to_mdast(&self.file.buf, &ParseOptions::default()).unwrap();
                        }
                    });
                    ui.code_editor(&mut self.file.buf);
                });
            });
            ui.label("hi");
        });
        ctx.input(|i| {
            if i.viewport().close_requested() {
                let state = ron::ser::to_string_pretty(
                    &(&self.file, &self.folder),
                    ron::ser::PrettyConfig::default(),
                )
                .unwrap();

                let mut save_file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(".state.ron")
                    .unwrap();
                save_file.write_all(state.as_bytes()).ok();
            }
        });
    }
}
