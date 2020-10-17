// std
use std::fs;
use std::io::{stdout, Write};

// external
use clap::{App, Arg, SubCommand};
use crossterm::{
    execute,
    style::{style, Color, Print},
    Result,
};

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
        .get_matches();

    if let Some(m) = app.subcommand_matches("new") {
        if m.is_present("name") {
            let name = m.value_of("name").expect("Failed to get name, aborting...");
            match fs::write(
                format!("{}.md", name.clone().to_lowercase()),
                format!(
                    r"# {}

## See Also
",
                    name
                ),
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
        } else {
            log(&"no name specified, aborting...".to_string(), "f").unwrap();
            std::process::exit(1)
        }
    }
}
