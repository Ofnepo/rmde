use eframe::egui::{Ui, Widget, Response};

fn editor_with_title_ui(ui: &mut Ui, title: &str, content: &mut String, is_modified: &mut bool) -> Response {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(title);
            if *is_modified {
                let _ = ui.radio(true, "");
            }
        });
        let editor_response = ui.code_editor(content);
        if editor_response.changed() {
            *is_modified = true;
        }
        editor_response
    }).inner
}

pub fn editor_with_title<'a>(title: &'a str, content: &'a mut String, is_modified: &'a mut bool) -> impl Widget + 'a {
    move |ui: &mut Ui| editor_with_title_ui(ui, title, content, is_modified)
}