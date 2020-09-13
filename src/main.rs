// std
use std::fs;

// external
use clap::{App, Arg, SubCommand};

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
        .get_matches();

    if let Some(m) = app.subcommand_matches("new") {
        if m.is_present("name") {
            let name = m.value_of("name").expect("Failed to get name, aborting...");
            fs::write(
                format!("{}.md", name.clone().to_lowercase()),
                format!(
                    r"# {}

## See Also
",
                    name
                ),
            )
            .expect("Failed to write to file, aborting...");
        } else {
            println!("No name specified, aborting...");
            std::process::exit(1)
        }
    }
}
