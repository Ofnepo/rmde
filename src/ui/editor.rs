use eframe::egui::{Ui, Response};

pub fn editor_with_title_show(ui: &mut Ui, title: &str, content: &mut String, is_modified: &mut bool) -> Response {
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