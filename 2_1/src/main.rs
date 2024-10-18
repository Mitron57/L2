use clap::Parser;
use std::io;

#[derive(Parser)]
#[command(version, about = env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
    #[arg(short, requires = "file", help = "Count characters in the file")]
    c: bool,
    #[arg(short, requires = "file", help = "Count lines in the file")]
    l: bool,
    #[arg(short, requires = "file", help = "Count words in the file")]
    w: bool,
    #[arg(required = true)]
    file: String,
}

fn read_file(path: &str) -> io::Result<String> {
    std::fs::read_to_string(path)
}

fn main() {
    let args = Args::parse();
    match read_file(&args.file) {
        Ok(content) => {
            if args.c || (!args.l && !args.w) {
                println!("chars: {}", content.chars().count());
            }
            if args.l {
                println!("lines: {}", content.lines().count());
            }
            if args.w {
                println!("words: {}", content.split_whitespace().count());
            }
        }
        Err(e) => {
            eprintln!("Can't read file: {}, reason: {}", args.file, e);
        }
    }
}
