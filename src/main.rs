use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_markdown_file(filename: &str) {
    println!(" [ INFO ] Starting parser!");

    let path = Path::new(filename);
    let file = File::open(&path)
                 .expect(" [ ERROR ] Failed to open file!");
    let reader = BufReader::new(file);

    // let mut ptag = false;
    // let mut htag = false;
    // let mut tokens: Vec<String> = Vec::new();

    for line in reader.lines() {
        let mut input_line = line.unwrap();
        let mut first_char = input_line.chars().nth(0);

        // let mut output_line = String::new();

        match first_char {
            Some('#') => {
                println!("<h1>{}</h1>", &input_line[2..])
            },
            Some('-') => {
                println!("<li>{}</li>", &input_line[2..])
            },
            _ => {
                println!("<p>{}</p>", &input_line)
            }
        }
    }

}

fn print_banner() {
    let title = env!("CARGO_PKG_NAME");
    print!(
        "{title} (v{version}), {description} \n{usage}",
        title = title,
        version = env!("CARGO_PKG_VERSION"),
        description = "a tiny and mostly useless markdown compiler",
        usage = format!("Usage: {title} <somefile.md>", title = title)
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("\n[ ERROR ] Invalid invocation\n");
            print_banner();
        }
    }
}
