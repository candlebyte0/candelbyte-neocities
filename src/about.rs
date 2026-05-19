use crate::PATHS;
use obsidian_parser::prelude::*;
use tera::Tera;

pub fn render_about(tera: Tera, vault: &VaultOnDisk, context: tera::Context) -> Tera {
    let context = insert_about_context(vault, context);

    let rendered = tera.render("about.html", &context).unwrap();
    std::fs::write(PATHS.output_path.to_owned() + "/about.html", rendered).unwrap();

    tera
}

fn insert_about_context(vault: &VaultOnDisk, mut context: tera::Context) -> tera::Context {
    for note in vault.notes() {
        if note.note_name().unwrap() == "About Me" {
            context.insert(
                "about_paragraphs",
                &note
                    .content()
                    .expect("Error reading 'About Me' note content!")
                    .split("---")
                    .map(str::to_owned)
                    .collect::<Vec<String>>(),
            );
        }
    }

    context
}
