
fn parse_markdown_file(filename: &str) {
    println!("TODO: parse {}", filename)
}

fn print_banner() {
    let title = String::from(env!("CARGO_PKG_NAME"));
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
