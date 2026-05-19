mod about;
mod bookshelf;
mod guestbook;
mod home;

use about::render_about;
use copy_dir::copy_dir;
use home::render_homepage;
use obsidian_parser::prelude::*;
use std::{collections::HashMap, fs};
use tera::{Tera, Value};

use crate::{bookshelf::render_bookshelf, guestbook::render_guestbook};

pub struct Paths {
    html_path: &'static str,
    styles_path: &'static str,
    content_path: &'static str,
    assets_path: &'static str,
    output_path: &'static str,
    buttons_path: &'static str,
}

pub const PATHS: Paths = Paths {
    html_path: "templates/html",
    styles_path: "templates/styles",
    content_path: "content",
    assets_path: "content/assets",
    output_path: "output",
    buttons_path: "content/assets/buttons",
};

fn main() {
    //Housekeeping
    fs::remove_dir_all(PATHS.output_path).unwrap();
    fs::create_dir(PATHS.output_path).unwrap();

    copy_dir(PATHS.assets_path, PATHS.output_path.to_owned() + "/assets").unwrap();
    copy_dir(PATHS.styles_path, PATHS.output_path.to_owned() + "/styles").unwrap();

    //Create Tera and Vault
    let mut tera = Tera::new(&(PATHS.html_path.to_owned() + "/*.html")).unwrap();

    tera.autoescape_on(vec![]);
    tera.register_filter("markdown", markdown_filter);
    tera.register_filter("commafy", comma_seperated_list_filter);
    tera.register_function("strip_asset_path", strip_asset_path);

    let vault_options = VaultOptions::new(PATHS.content_path);
    let vault: VaultOnDisk = VaultBuilder::new(&vault_options)
        .into_iter()
        .filter_map(Result::ok)
        .build_vault(&vault_options);

    //Insert basic context
    let context = insert_basic_context(&vault, tera::Context::new());

    //Rendering
    println!("Rendering site pages...");
    render_homepage(tera.clone(), &vault, context.clone());
    render_about(tera.clone(), &vault, context.clone());
    render_bookshelf(tera.clone(), &vault, context.clone());
    render_guestbook(tera, &vault, context);
    println!("All pages rendered!");
}

fn insert_basic_context(vault: &VaultOnDisk, mut context: tera::Context) -> tera::Context {
    for note in vault.notes() {
        if note.note_name().unwrap() == "Links" {
            let links = note
                .content()
                .expect("'Links' note must have content!")
                .lines()
                .map(|s| {
                    let strings: Vec<String> = s.split("|").map(str::to_owned).collect();
                    let text = strings[0].trim().to_owned();
                    let link = strings[1].trim().to_owned();
                    (text, link)
                })
                .collect::<Vec<(String, String)>>();

            let mut html = "".to_owned();

            for (text, link) in links {
                html += &format!("<a href=\"{link}\">{text}</a><br>\n");
            }

            context.insert("links", &html)
        }
    }

    context
}

//Tera filter to turn markdown to HTML. Must be used with the safe filter.
fn markdown_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let markdown = tera::from_value::<String>(value.clone())
        .expect("Markdown filter can only be used with values that can covnert to strings!");
    let options = markdown::Options {
        compile: markdown::CompileOptions {
            allow_dangerous_html: true,
            ..Default::default()
        },
        ..Default::default()
    };
    Ok(tera::to_value(
        markdown::to_html_with_options(&markdown, &options)
            .expect("Failed to convert markdown to html"),
    )?)
}

//Tera function to strip asset names down to just relative path from asset directory without extension
fn strip_asset_path(args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    match args.get("asset_path") {
        Some(value) => match tera::from_value::<String>(value.clone()) {
            Ok(button_path) => Ok(tera::to_value(
                button_path
                    .strip_prefix("/assets/")
                    .ok_or("Asset path prefix missing!")?
                    .split(".")
                    .next()
                    .ok_or("File name before dot missing!")?
                    .to_owned(),
            )?),
            Err(_) => Err(tera::Error::msg(
                "Failed to convert passed value to string!",
            )),
        },
        None => Err(tera::Error::msg("Please set the 'asset_path' argument!")),
    }
}

fn comma_seperated_list_filter(
    value: &Value,
    _: &HashMap<String, Value>,
) -> Result<Value, tera::Error> {
    let list = tera::from_value::<Vec<String>>(value.clone())
        .expect("Must be a list of stringable values!");

    let mut out = "".to_owned();

    for item in list {
        out += &(item + ", ");
    }

    Ok(tera::to_value(out.strip_suffix(", ").unwrap())?)
}
