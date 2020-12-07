// std
use std::env::current_dir;
use std::fs::{read_to_string, write};
use std::io::{stdout, Write};

// external
use clap::{App, Arg, SubCommand};
use crossterm::{
    execute,
    style::{style, Color, Print},
    Result,
};
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use rayon::prelude::*;
use reqwest::blocking::get;
use walkdir::WalkDir;

// local
use notes::Index;

fn log(message: &String, message_type: &str) -> Result<()> {
    match message_type {
        "f" => execute!(
            stdout(),
            Print(style("ERROR  ").with(Color::Red)),
            Print(style(message)),
            Print("\n")
        ),
        "s" => execute!(
            stdout(),
            Print(style("SUCESS ").with(Color::Green)),
            Print(style(message)),
            Print("\n")
        ),
        _ => execute!(stdout(), Print(style(message)), Print("\n")),
    }
}

fn main() {
    let app = App::new("notes")
        .about("Tooling for managing my notes.")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("new")
                .about("Generate a new note.")
                .arg(Arg::with_name("name").min_values(1).takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("index")
                .about("Index a directory")
                .arg(Arg::with_name("recursive").takes_value(false).short("r")),
        )
        .subcommand(SubCommand::with_name("archive").about("Archive external content."))
        .get_matches();

    if let Some(m) = app.subcommand_matches("new") {
        let name = match m.value_of("name") {
            Some(v) => v,
            None => "index",
        };
        match write(
            format!("{}.md", name.clone().to_lowercase()),
            format!("# {}\n\n``\n\n## See Also\n", name),
        ) {
            Ok(_) => {
                log(&format!("generated note {}", name), "s").unwrap();
            }
            Err(e) => {
                log(
                    &format!("failed to write to file, aborting... ({})", e.to_string()),
                    "f",
                )
                .unwrap();
            }
        };
    } else if let Some(m) = app.subcommand_matches("index") {
    } else if let Some(subcommand) = app.subcommand_matches("archive") {
        let mut index: Vec<String> = Vec::new();
        let dir = current_dir().expect("Failed to get current directory, aborting...");
        for i in WalkDir::new(dir.clone()) {
            let options = Options::all();
            let text = read_to_string(i.unwrap().path()).expect("Failed to read file");
            for event in Parser::new_ext(text.as_str(), options).into_iter() {
                if let Event::Start(tag) = event {
                    if let Tag::Link(link, destination, title) = tag {
                        index.push(destination.to_string().clone());
                    }
                }
            }
        }

        println!("Beginning to download {} items.", index.len());
        let results = download_urls(index.clone());
    /*        for result in results.call_once(args).iter() {
        for num in 0..index.len() {
            write(std::path::Path::new("/archive/").join(num), result)
                .expect("Failed to write content to file");
        }
    }
    let json_index = Index { items: index };

    write(
        dir.join("index.json"),
        serde_json::to_string_pretty(&json_index)
            .expect("Failed to serialise index into JSON."),
    )
    .expect("Failed to write index."); */
    } else {
        log(&"no name specified, aborting...".to_string(), "f").unwrap();
        std::process::exit(1)
    }
}

fn download_urls(index: Vec<String>) -> std::result::Result<Vec<String>, reqwest::Error> {
    let mut results: Vec<String> = Vec::new();
    /*for url in index.par_iter() {
        // not sure how futures work, just using Rayon instead
        results.push(get(&url)?.text()?);
    }*/

    Ok(results)
}
