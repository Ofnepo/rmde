use std::{fs::{File, OpenOptions}, io::{Read, Write}};

use fltk::{window::MenuWindow, app::App, prelude::*, text::{TextEditor, TextBuffer}, button::Button};

struct RayFile{
    path: String,
    origin: String,
    buf: TextBuffer,
    edit: TextEditor
}

impl Default for RayFile {
    fn default() -> Self {
        Self {
            path: String::new(),
            origin: String::new(),
            buf: TextBuffer::default(),
            edit: TextEditor::default()
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
                let mut buf = TextBuffer::default();
                buf.set_text(&origin);
                let mut edit = TextEditor::default();
                edit.set_buffer(buf);
                Self {
                    path: path,
                    origin: origin,
                    buf: edit.buffer().unwrap(),
                    edit: edit
                }

            }
        }
    }
}

fn main() {
    let app = App::default();
    let mut win = MenuWindow::default().with_size(1270, 720);
    let mut file = RayFile::new("README.md".into());
    file.edit.set_size(win.width()-160, win.height()-40);
    file.edit.set_pos(150, 30);
    let mut save_bt = Button::new(2, 2, 50, 26, "Save");

    save_bt.set_callback(move| c |{
        let mut save_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(&file.path)
            .unwrap();
        file.origin = file.buf.text();
        save_file.write_all(file.origin.as_bytes()).expect("did't work idk");
        c.damage();
    });

    
    win.end();
    win.show();

    app.run().unwrap();
}
