use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn html_content(tag: &str, content: &str) -> String {
    format!(
        "<{tag}>{content}</{tag}>",
        tag = tag,
        content = content.trim()
    )
}

fn get_tag(markdown_tag: &str) -> &'static str {
    match markdown_tag {
        "#" => "h1",
        "##" => "h2",
        "###" => "h3",
        "*" | "+" | "-" => "li",
        _ => "p"
    }
}

fn parse_markdown_file(filename: &str) -> Vec<String> {
    println!(" [ INFO ] Starting parser!");

    let path = Path::new(filename);
    let file = File::open(&path).expect(" [ ERROR ] Failed to open file!");
    let reader = BufReader::new(file);

    // Flag for unordered list
    let mut ul_tag = false;

    // Create a place to store all our tokens
    let mut tokens: Vec<String> = Vec::new();

    for line in reader.lines() {
        let input_line = line.unwrap();
        let mut output_line = String::new();

        if input_line.starts_with(|c: char| c.is_ascii_punctuation()) {
            let element = input_line.split_whitespace().nth(0).unwrap();
            let tag = get_tag(element);
            let new_line = html_content(tag, &input_line[2..]);

            if !ul_tag && tag == "li" {
                ul_tag = true;
                output_line.push_str("<ul>");
            }

            output_line.push_str(&new_line);
        } else if !input_line.is_empty() {
            let new_line = html_content("p", &input_line);
            output_line.push_str(&new_line);
        } else {
          if ul_tag {
              ul_tag = false;
              output_line.push_str("</ul>");
          }
        }

        if !output_line.is_empty() {
          tokens.push(output_line);
        }
    }

    println!(" [ INFO ] Parsing complete!");

    return tokens;
}

fn print_usage_banner() {
    let title = env!("CARGO_PKG_NAME");
    print!(
        "{title} (v{version}), {description} \n{usage}",
        title = title,
        version = env!("CARGO_PKG_VERSION"),
        description = "a tiny and mostly useless markdown compiler",
        usage = format!("Usage: {title} <somefile.md>", title = title)
    );
}

fn runner(args: Vec<String>) {
    let input_filename = &args[1];
    let output_filename = &input_filename.replace("md", "html");
    let mut outfile = File::create(output_filename)
        .expect(" [ ERROR ] Could not create output file");

    let tokens = parse_markdown_file(input_filename);
    for line in &tokens {
        outfile.write_all(line.as_bytes())
            .expect(" [ ERROR ] Could not write to output file");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        length if length >= 2 => runner(args),
        _ => {
            println!("\n[ ERROR ] Invalid invocation\n");
            print_usage_banner();
        }
    }
}
