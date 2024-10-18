mod args;

use args::Args;
use clap::Parser;
use std::io::Read;
fn print_fields(line: &[&str], fields: &[usize]) {
    for &column in fields.iter() {
        if column > 0 && column < line.len() {
            print!("{}\t", line[column - 1]);
        }
    }
    println!();
}

fn main() {
    let args = Args::parse();
    let mut content = Vec::new();
    if std::io::stdin().read_to_end(&mut content).is_err() {
        eprintln!("error: could not read from stdin");
        return;
    }
    let content: Vec<String> = match String::from_utf8(content) {
        Ok(content) => content.lines().map(|line| line.trim().to_owned()).collect(),
        Err(e) => {
            eprintln!("error: {}", e);
            return;
        }
    };
    let delimiter = if let Some(delim) = args.delimiter {
        delim
    } else {
        "\t".to_owned()
    };
    for line in content.iter() {
        let split: Vec<&str> = line.split(delimiter.as_str()).collect();
        if split.len() == 1 && args.only_delimited {
            continue;
        }
        if args.fields.is_empty() {
            println!("{}", split.join("\t"));
            continue;
        }
        print_fields(split.as_slice(), args.fields.as_slice());
    }
}
