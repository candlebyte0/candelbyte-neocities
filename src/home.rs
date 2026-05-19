use crate::bookshelf::*;
use crate::PATHS;
use obsidian_parser::prelude::*;
use tera::Tera;

pub fn render_homepage(tera: Tera, vault: &VaultOnDisk, context: tera::Context) -> Tera {
    let context = insert_homepage_context(vault, context);

    let rendered = tera.render("home.html", &context).unwrap();
    std::fs::write(PATHS.output_path.to_owned() + "/index.html", rendered).unwrap();

    tera
}

fn insert_homepage_context(vault: &VaultOnDisk, mut context: tera::Context) -> tera::Context {
    for note in vault.notes() {
        if note.note_name().unwrap() == "Homepage" {
            //Grab welcome paragraph
            let welcome = note.content().unwrap();
            context.insert("welcome", &welcome);
        } else if note.note_name().unwrap() == "Button Links" {
            //Grab names and links of 88x31 buttons
            let links: Vec<(String, String)> = note
                .content()
                .unwrap()
                .split("---")
                .filter_map(|s| s.strip_prefix("\nLINKS\n"))
                .collect::<String>()
                .lines()
                .map(|s| {
                    let strings: Vec<String> =
                        s.split(" ").map(str::trim).map(str::to_owned).collect();
                    let name = "buttons/".to_owned() + &strings[0];
                    let link = strings[1].clone();
                    (name, link)
                })
                .collect();

            context.insert("links_and_names", &links);
        } else if note.tags().unwrap().contains(&"current-book".to_owned()) {
            let book: Book = note.try_into().unwrap();
            context.insert("book", &book);
        }
    }

    //Grab button paths
    let buttons: Vec<String> = std::fs::read_dir(PATHS.buttons_path)
        .expect("Could not read buttons directory!")
        .filter_map(|e| e.ok()?.file_name().into_string().ok())
        .map(|s| "/assets/buttons/".to_owned() + &s)
        .collect();

    context.insert("buttons", &buttons);

    context
}
