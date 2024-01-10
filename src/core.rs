use serde::{Deserialize, Serialize};
use std::{
    fs::{read_dir, OpenOptions},
    io::{Read, Write},
};
use markdown::{
    mdast::{Node, Root},
    to_mdast, ParseOptions,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct RayFile {
    pub path: String,
    pub name: String,
    pub buf: String,
    pub is_modified: bool
}

impl Default for RayFile {
    fn default() -> Self {
        Self {
            path: String::new(),
            name: String::new(),
            buf: String::new(),
            is_modified: false
        }
    }
}

impl RayFile {
    pub fn new(path: String) -> Self {
        match path {
            c if c.is_empty() => Self::default(),
            _ => {
                let mut buf = String::new();
                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(false)
                    .open(&path)
                    .unwrap();
                file.read_to_string(&mut buf).ok();

                let name = path.split('/').last().unwrap().to_string();

                //println!("Path: {}\nOrigin: {}\nBuf: {}", &path, &origin, &buf);
                Self {
                    name,
                    path,
                    buf,
                    is_modified: false
                }
            }
        }
    }

    pub fn save(&mut self){
        let mut save_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .unwrap();
        self.is_modified = false;
        save_file.write_all(self.buf.as_bytes()).ok();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RayFolder {
    pub path: String,
    pub name: String,
    pub files: Vec<String>,
    pub folders: Vec<RayFolder>,
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
    pub fn new(path: String) -> Self {
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
}

pub struct MyApp {
    pub file: RayFile,
    pub folder: RayFolder,
    pub md: Node,
}

impl Default for MyApp {
    fn default() -> Self {
        let state = RayFile::new(".state.ron".to_string()).buf;

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

impl MyApp {
    pub fn save_state(&self) {
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

    pub fn parse_md(&mut self) {
        self.md = to_mdast(&self.file.buf, &ParseOptions::default()).unwrap();
    }
}