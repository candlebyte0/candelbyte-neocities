use crate::PATHS;
use obsidian_parser::prelude::*;
use tera::Tera;

#[derive(serde::Serialize)]
pub struct Book {
    title: String,
    authors: Vec<String>,
    cover: String,
    review: String,
    link: String,
}

pub fn render_bookshelf(tera: Tera, vault: &VaultOnDisk, mut context: tera::Context) -> Tera {
    //Collect book
    let mut books: Vec<Book> = vec![];

    std::fs::create_dir_all(PATHS.output_path.to_owned() + "/bookshelf").unwrap();

    for note in vault.notes() {
        if note
            .tags()
            .expect("Failed to retrieve note tags!")
            .contains(&"book".to_owned())
        {
            let book = note.try_into().unwrap();
            render_book_page(&tera, &book, context.clone());
            books.push(book);
        } else if note.note_name().unwrap() == "Bookshelf" {
            context.insert("intro", &note.content().unwrap());
        }
    }

    let shelves: Vec<&[Book]> = books.chunks(6).collect();

    context.insert("shelves", &shelves);

    let rendered = tera.render("bookshelf.html", &context).unwrap();
    std::fs::write(PATHS.output_path.to_owned() + "/bookshelf.html", rendered).unwrap();

    tera
}

fn render_book_page(tera: &Tera, book: &Book, mut context: tera::Context) {
    context.insert("book", book);

    let rendered = tera.render("book.html", &context).unwrap();
    std::fs::write(PATHS.output_path.to_owned() + &book.link, rendered).unwrap();
}

fn linkify_title(title: String) -> String {
    "/bookshelf/".to_owned()
        + &title
            .to_lowercase()
            .chars()
            .map(|c| if c.is_whitespace() { '+' } else { c })
            .collect::<String>()
        + ".html"
}

impl TryFrom<&NoteOnDisk> for Book {
    type Error = obsidian_parser::note::note_on_disk::Error;

    fn try_from(value: &NoteOnDisk) -> Result<Book, Self::Error> {
        let props = value
            .properties()?
            .expect("Note must have properties to be converted to a book!");
        let title = value.note_name().unwrap_or("UNTITLED".to_owned());
        let cover = "/assets/book-covers/".to_owned()
            + props
                .get("cover")
                .expect("Note must have 'cover' property!")
                .as_str()
                .expect("'cover' property must be a String!");
        let authors: Vec<String> = props
            .get("authors")
            .expect("Note must have an 'authors' property!")
            .as_sequence()
            .expect("'authors' property must be a List!")
            .iter()
            .map(|v| {
                v.as_str()
                    .expect("'authors' elements must be Strings!")
                    .to_owned()
            })
            .collect();

        let review = value.content()?.to_string();

        Ok(Self {
            authors,
            cover,
            review,
            link: linkify_title(title.clone()),
            title,
        })
    }
}
