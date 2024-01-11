use serde::{Deserialize, Serialize};
use std::{
    fs::{read_dir, read_to_string, write},
    io::{Error, ErrorKind}, path::{PathBuf, Path},
};
use markdown::{
    mdast::{Node, Root},
    to_mdast, ParseOptions,
};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RayFile {
    pub path: Option<PathBuf>,
    pub buf: String,
    pub is_modified: bool
}

impl RayFile {
    pub fn new(path: &Path) -> Self {
        let read_result = read_to_string(path).ok();
        let is_modified = read_result.is_none();
        let buf = read_result.unwrap_or(String::default());
        Self {
            path: Some(path.to_path_buf()),
            buf,
            is_modified: is_modified
        }
    }

    pub fn save(&mut self) -> Result<(), Error>{ 
        if let Some(path) = &self.path {
            let result = write(&path, &mut self.buf);
            if result.is_ok() {
                self.is_modified = false
            }
            result
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "tried to save file with empty path"))
        }
    }

    pub fn name(&self) -> Option<String> {
        self.path.clone()?.file_name()?.to_str().map(str::to_string)
    }

    pub fn name_or_default(&self) -> String {
        self.name().unwrap_or("<unnamed file>".to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RayFolder {
    pub path: Option<PathBuf>,
    pub files: Vec<PathBuf>,
    pub folders: Vec<RayFolder>,
}

impl RayFolder {
    pub fn new(path: &Path) -> Self {
        let mut dirs: Vec<RayFolder> = Vec::new();
        let mut files: Vec<PathBuf> = Vec::new();
        if let Ok(dir) = read_dir(&path) {
            for entry in dir.flat_map(Result::ok) {
                match entry.file_type() {
                    Ok(typ) => {
                        if typ.is_file() {
                            files.push(entry.path())
                        }else if typ.is_dir() {
                            dirs.push(RayFolder::new(&entry.path()))
                        }
                    }
                    _ => ()
                }
            }
        }
        Self {
            path: Some(path.to_path_buf()),
            files,
            folders: dirs,
        }
    }

    pub fn name(&self) -> Option<String> {
        self.path.clone()?.file_name()?.to_str().map(str::to_string)
    }

    pub fn name_or_default(&self) -> String {
        self.name().unwrap_or("<unnamed folder>".to_string())
    }
}

pub struct MyApp {
    pub file: RayFile,
    pub folder: RayFolder,
    pub md: Node,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file: RayFile::default(),
            folder: RayFolder::default(),
            md: Node::Root(Root { children: Vec::new(), position: None })
        }
    }
}

impl MyApp {
    const STATE_FILE_PATH: &str = ".state.ron";

    pub fn save_state(&self) -> Result<(), Error> {
        let state = ron::ser::to_string_pretty(
            &(&self.file, &self.folder),
            ron::ser::PrettyConfig::default(),
        ).map_err(|err| Error::new(ErrorKind::Other, err))?;

        write(Self::STATE_FILE_PATH, state)
    }
    
    pub fn load_state(&mut self) -> Result<(), Error>{
        let (file, folder) = ron::from_str(&read_to_string(Self::STATE_FILE_PATH)?)
            .map_err(|err| Error::new(ErrorKind::Other, err))?;

        self.file = file;
        self.folder = folder;

        self.parse_md();

        Ok(())
    }

    pub fn parse_md(&mut self) {
        self.md = to_mdast(&self.file.buf, &ParseOptions::default()).unwrap();
    }
}