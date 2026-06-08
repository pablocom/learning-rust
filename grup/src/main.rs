use std::{env, error::Error, fs, process};

use grup::{search_case_insensitive, search_case_sensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    let query = TextSearchQuery::new(&args, ignore_case).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(query) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}

fn run(query: TextSearchQuery) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(query.file_path)?;

    let results = if query.ignore_case {
        search_case_insensitive(&query.text_to_find, &file_content)
    } else {
        search_case_sensitive(&query.text_to_find, &file_content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

struct TextSearchQuery<'a> {
    text_to_find: &'a str,
    file_path: &'a str,
    ignore_case: bool
}

impl<'a> TextSearchQuery<'a> {
    pub fn new(args: &'a [String], ignore_case: bool) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err(
                "Not enough arguments. Example usage '(grup / cargo run --) word poem.txt'",
            );
        }

        Ok(Self {
            text_to_find: &args[1],
            file_path: &args[2],
            ignore_case,
        })
    }
}
