use eframe::egui::Ui;
use markdown::mdast::Node;

pub fn md_to_frame(ui: &mut Ui, md: &Node) {
    for c in md.children().unwrap() {
        match &c {
            &Node::Root(_) => md_to_frame(ui, md),
            &Node::Paragraph(_) => {
                ui.label(get_text(c));
            }
            &Node::Heading(_) => {
                ui.heading(get_text(c));
            }
			&Node::Link(a) => {
				println!("{}, {}", a.title.as_ref().unwrap(), a.url);
				if a.title != None {
					ui.hyperlink_to(a.title.as_ref().unwrap(), &a.url);
				} else {
					ui.hyperlink(&a.url);
				}
			}
            _ => (),
        }
    }
}

fn get_text(md: &Node) -> &str {
    for c in md.children().unwrap() {
        return match c {
            Node::Text(a) => &a.value,
            _ => "",
        };
    }
    ""
}
