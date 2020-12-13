// std
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::{read_to_string, write, File};
use std::io::{stdout, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

// external
use clap::{App, Arg, SubCommand};
use crossterm::{
    execute,
    style::{style, Color, Print},
    Result,
};
use lazy_static::lazy_static;
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use rayon::prelude::*;
use regex::Regex;
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
        let dir = current_dir().expect("Failed to get current directory, aborting...");
        index(&dir);
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
        log(&"Nothing specified...".to_string(), "f").unwrap();
        std::process::exit(1)
    }
}

fn index(dir: &PathBuf) -> HashMap<Vec<String>, Vec<HashMap<String, String>>> {
    lazy_static! {
        static ref PRIORITY_ONE: Regex = Regex::new("p::1").unwrap();
    }

    let mut errs: Vec<HashMap<String, String>> = Vec::new();
    let mut index: Vec<String> = Vec::new();
    for i in WalkDir::new(dir.clone()) {
        let original_file_path = i.unwrap();

        let path = original_file_path.path();
        if path.is_file() == false {
            continue;
        };
        match File::open(path) {
            Ok(file) => {
                for line in BufReader::new(file).lines() {
                    if let Ok(ln) = line {
                        if PRIORITY_ONE.is_match(&ln.as_str()) {
                            index.push(ln);
                        }
                    } else if let Err(err) = line {
                        let mut report: HashMap<String, String> = HashMap::new();
                        report.insert(path.to_string_lossy().to_string(), err.to_string());
                        errs.push(report);
                    }
                }
            }
            Err(err) => {
                let mut report: HashMap<String, String> = HashMap::new();
                report.insert(path.to_string_lossy().to_string(), err.to_string());
                errs.push(report);
            }
        };
    }

    let mut rtval = HashMap::new();
    rtval.insert(index, errs);
    rtval
}

fn download_urls(index: Vec<String>) -> std::result::Result<Vec<String>, reqwest::Error> {
    let mut results: Vec<String> = Vec::new();
    /*for url in index.par_iter() {
        // not sure how futures work, just using Rayon instead
        results.push(get(&url)?.text()?);
    }*/

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_priority_regex() {
        let current_dir = std::env::current_dir().unwrap();
        let regex_dir = current_dir.join("testing_regex");
        let file = regex_dir.join("priority.md");

        std::fs::create_dir(regex_dir.clone()).unwrap();
        std::fs::write(file.clone(), "(GitHub)[https://github.com] [[p::1]]").unwrap();

        let items = index(&regex_dir);

        items.iter().enumerate().for_each(|(f, i)| {
            if f == 0 {
                assert_eq!("(GitHub)[https://github.com] [[p::1]]", i.0.get(0).unwrap());
            }
        });

        std::fs::remove_file(file).unwrap();
        std::fs::remove_dir(regex_dir).unwrap();
    }
}
