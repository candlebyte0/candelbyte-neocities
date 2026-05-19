use crate::PATHS;
use obsidian_parser::prelude::*;
use tera::Tera;

pub fn render_guestbook(tera: Tera, vault: &VaultOnDisk, context: tera::Context) -> Tera {
    let rendered = tera.render("guestbook.html", &context).unwrap();
    std::fs::write(PATHS.output_path.to_owned() + "/guestbook.html", rendered).unwrap();

    tera
}
