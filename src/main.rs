use std::{fs::{File, OpenOptions, read_dir}, io::{Read, Write}, path::Path, str::FromStr};

use fltk::{window::MenuWindow, app::App, prelude::*, text::{TextEditor, TextBuffer}, button::Button, tree::{Tree, TreeItem}};


#[derive(Debug, Clone)]
struct RayFile{
    path: String,
    name: String,
    origin: String,
}

impl Default for RayFile {
    fn default() -> Self {
        Self {
            path: String::new(),
            name: String::new(),
            origin: String::new(),
        }
    }
}


impl RayFile {
    fn new(path: String) -> Self {
        match path {
            c if c.is_empty() => Self::default(),
            _ => {
                let mut origin = String::new();
                let mut file =  File::open(&path).ok().unwrap();
                file.read_to_string(&mut origin).ok();

                let mut name = path
                    .split("/")
                    .last()
                    .unwrap()
                    .to_string();
                
                Self {
                    name: name,
                    path: path,
                    origin: origin,
                }


            }
        }
    }


}


#[derive(Debug, Clone)]
struct RayFolder {
    path: String,
    name: String,
    files: Vec<String>,
    folders: Vec<RayFolder>
}

impl Default for RayFolder {
    fn default() -> Self {
        Self { 
            name: String::new(),
            path: String::new(),
            files: Vec::new(), 
            folders: Vec::new()
        }
    }
}




impl RayFolder {
    fn new(path: String) -> Self {
        if path.is_empty(){
            Self::default()
        } else {
            let dir = read_dir(&path)
                .ok().unwrap();
            let mut dirs: Vec<RayFolder> = Vec::new();
            let mut files: Vec<String> = Vec::new();
            dir.for_each(|a|{
                let i = a.ok().unwrap();

                if i.file_type().ok().unwrap().is_dir() {
                    dirs.push(RayFolder::new(i.path().to_str().unwrap().to_string()));
                } else {
                    files.push(i.path().to_str().unwrap().to_string());
                }
            });
            let mut name = String::new();
            name = path.split("/").last().unwrap().to_string();
            
            Self { 
                name: name, 
                path: path, 
                files: files, 
                folders: dirs 
            }
            
        }
    }
    fn data(self) -> String{
        let mut file_temp = String::new();
        for i in &self.files{
            file_temp+= &i;
            file_temp+= ", ";
        }
        let mut folder_temp = String::new();
        for dir in self.folders{
            for line in dir.data().split("\n"){
                folder_temp+= "\t";
                folder_temp+= line;
            }
        } 

        let my = "path: ".to_string() +  &self.path + "\nname: " + &self.name + "\nfiles: " + &file_temp + "\nfolders: " + &folder_temp;
        my
    }
    
}




fn main() {
    let app = App::default();
    let mut win = MenuWindow::default().with_size(1270, 720);
    let mut file = RayFile::new("README.md".into());
    let mut save_bt = Button::new(2, 2, 50, 26, "Save");
    let mut tree = Tree::new(2,30,146,win.height()-40,"");

    let mut edit = TextEditor::default();

    edit.set_size(win.width()-160, win.height()-40);
    edit.set_pos(150, 30);

    let dir = RayFolder::new(".".to_string());
    for file in &dir.files{
        tree.add(&file);
        tree.set_callback( |c| {
            println!("{:?}", &c.find_clicked(true).unwrap().label().unwrap());

            let mut buf = TextBuffer::default();
            buf.set_text(RayFile::new((&c).find_clicked(true).unwrap().label().unwrap()).origin.as_str());
            //edit.set_buffer(buf);
        });
    }




    save_bt.set_callback(move| c |{
        let mut save_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(&file.path)
            .unwrap();
        //file.origin = file.buf.text();
        save_file.write_all(file.origin.as_bytes()).expect("did't work idk");
        c.damage();
    });

    
    win.end();
    win.show();

    app.run().unwrap();
}
