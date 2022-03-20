use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::*;

/// print_usage
/// will print the usage of the program
/// to the cli.
fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green().bold()
    );
    eprintln!(
        "{}: quickreplace {} {} {} {}",
        "Usage".yellow(),
        "<target>".green(),
        "<replacement>".blue(),
        "<INPUT>".green(),
        "<OUTPUT>".blue()
    );
}


/// We need to parse the command line arguments
/// into an Arguments struct.
fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong {} of arguments: expected {} got {}",
            "Error: ".red().bold(),
            "number".red().bold(),
            "4".blue().bold(),
            args.len().to_string().red().bold()
        );
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

/// The main Magic happens here.
/// Here, we filter the text we read in from a 
/// file and replace every occurrence of the provided
/// regular expression with the replacement expression.
fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args = parse_args();

    // Read in the contents of INPUT into a string.
    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}' : {:?}",
                "Error".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    // filter the data - string
    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text {:?}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    };

    // write the filtered string into the OUTPUT file.
    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}' : {:?} ",
                "Error".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    println!(
        "{} - {} written to: {}",
        "SUCCESS".green().bold(),
        "output".blue().bold(),
        args.output.blue().bold()
    );
}
