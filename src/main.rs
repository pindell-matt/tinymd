
// tinymd (v0.1.0), a tiny and mostly useless markdown compiler. 
// Written by <Your Name>
// Usage: tinymd <somefile.md>

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
    print_banner()
}
