
# tinymd

A tiny and terrible markdown compiler!

It is just out here trying to live its best life - compilin' simple Markdown files into valid HTML.

I got the ball rolling via inspiration from [Jesse Lawson's Article: Getting Started with Rust by Building a Markdown Compiler](https://jesselawson.org/rust/getting-started-with-rust-by-building-a-tiny-markdown-compiler/) - although I go about things a slightly different way.

## Setup

Make sure [Rust and Cargo](https://www.rust-lang.org/) are installed and then build / install dependencies with:

```
cargo build
```

## Running

Pass your test Markdown as a arg:

```
cargo run -q [test.md]
```
