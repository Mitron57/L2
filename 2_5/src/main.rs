mod args;

use args::Args;
use clap::Parser;
use regex::Regex;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;

fn count_or_print(args: &Args, line: (usize, &str), count: &mut usize) {
    let (i, line) = line;
    if args.count {
        *count += 1;
    } else if args.line_num {
        println!("{}: {}", i + 1, line);
    } else {
        println!("{}", line);
    }
}

fn print_context(context: Vec<impl Display>) {
    if !context.is_empty() {
        println!("---");
        for line in context {
            println!("{}", line);
        }
    }
}

fn new_reader(file: &Option<String>) -> Option<Box<dyn Read>> {
    match file {
        Some(path) => {
            let filestream = File::open(path);
            match filestream {
                Ok(file) => Some(Box::new(file)),
                Err(e) => {
                    println!("Error: {e}");
                    None
                }
            }
        }
        None => Some(Box::new(std::io::stdin())),
    }
}

fn main() {
    let mut args = Args::parse();
    if args.count {
        args.context = None;
        args.after = None;
        args.before = None;
    }
    if args.ignore {
        args.pattern = args.pattern.to_lowercase();
    }
    let regex_pattern = if args.fixed {
        ""
    } else {
        args.pattern.as_str()
    };
    let engine = match Regex::new(regex_pattern) {
        Ok(pattern) => pattern,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    let mut input: Box<dyn Read> = if let Some(reader) = new_reader(&args.file) {
        reader
    } else {
        return;
    };
    let mut buffer = Vec::new();
    if let Err(e) = input.read_to_end(&mut buffer) {
        println!("Error: {e}");
        return;
    }
    let lines: Vec<String> = match String::from_utf8(buffer) {
        Ok(string) => string.lines().map(str::to_owned).collect(),
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    let mut count = 0;
    if args.fixed {
        for (i, line) in lines.iter().enumerate() {
            if *line == args.pattern || args.invert {
                count_or_print(&args, (i, line), &mut count);
            }
        }
        if args.count {
            println!("{count}");
        }
        return;
    }
    let mut after = Vec::new();
    let mut before = Vec::new();
    let mut context = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let line: &str = if args.ignore {
            &line.to_lowercase()
        } else {
            line
        };
        if engine.is_match(line) && !args.invert {
            if let Some(after_count) = args.after {
                let after_count = (i + after_count).clamp(0, lines.len());
                after.extend(&lines[i..after_count]);
            }
            if let Some(before_count) = args.before {
                let before_count = (i - before_count).clamp(0, lines.len());
                before.extend(&lines[before_count..i]);
            }
            if let Some(context_count) = args.context {
                let low = (i - context_count).clamp(0, lines.len());
                let high = (i + context_count).clamp(0, lines.len());
                context.extend(&lines[low..high]);
            }
            count_or_print(&args, (i, line), &mut count);
        } else if args.invert {
            count_or_print(&args, (i, line), &mut count);
        }
    }
    if args.count {
        println!("{count}");
        return;
    }
    print_context(after);
    print_context(before);
    print_context(context);
}
