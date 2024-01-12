use eframe::egui::Ui;
use markdown::mdast::Node;

pub fn md_view_show(ui: &mut Ui, md: &Node) {
    match md {
        Node::Root(_) => (),
        Node::Paragraph(_) => {
            ui.label(get_text(md));
        }
        Node::Heading(_) => {
            ui.heading(get_text(md));
        }
        Node::Link(a) => {
            if let Some(title) = &a.title {
                ui.hyperlink_to(title, &a.url);
            } else {
                ui.hyperlink(&a.url);
            }
        }
        _ => (),
    }
    for child in md.children().into_iter().flatten() {
        md_view_show(ui, child);
    }
}

fn get_text(md: &Node) -> &str {
    if let Some(c) = md.children().unwrap().iter().next() {
        return match c {
            Node::Text(a) => &a.value,
            _ => "",
        };
    }
    ""
}
