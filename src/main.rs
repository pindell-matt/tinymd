
// tinymd (v0.1.0), a tiny and mostly useless markdown compiler. 
// Written by <Your Name>
// Usage: tinymd <somefile.md>

fn print_short_banner() {
    let title = String::from(env!("CARGO_PKG_NAME"));
    let version = String::from(env!("CARGO_PKG_VERSION"));
    let usage = format!("Usage: {title} <somefile.md>", title = title);

    let banner = format!(
        "{title} (v{version}), {description} \n{usage}",
        title = title,
        version = version,
        description = "a tiny and mostly useless markdown compiler",
        usage = usage
    );

    print!("{}", banner);
}

fn main() {
    print_short_banner()
}
