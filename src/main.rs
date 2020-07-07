use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_tag(markdown_tag: &str) -> &'static str {
    match markdown_tag {
        "#" => "h1",
        _ => "p"
    }
}

fn parse_markdown_file(filename: &str) {
    println!(" [ INFO ] Starting parser!");

    let path = Path::new(filename);
    let file = File::open(&path).expect(" [ ERROR ] Failed to open file!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let input_line = line.unwrap();

        let mut output_line = String::new();

        if input_line.starts_with(|c: char| c.is_ascii_punctuation()) {
            let element = input_line.split_whitespace().nth(0).unwrap();
            let tag = get_tag(element);

            let new_line = format!(
                "<{tag}>{content}</{tag}>",
                tag = tag,
                content = &input_line[2..]
            );

            output_line.push_str(&new_line);
        } else if !input_line.is_empty() {
            let new_line = format!(
                "<{tag}>{content}</{tag}>",
                tag = "p",
                content = &input_line
            );

            output_line.push_str(&new_line);
        }

        println!("{}", output_line);
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
