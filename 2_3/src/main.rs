mod args;
mod check;
mod compare;
mod sort;

use crate::sort::sort_by;
use args::Args;
use check::check_sorted;
use clap::{CommandFactory, Parser};

fn main() {
    let mut args = Args::parse();
    if args.help {
        Args::command().print_help().unwrap();
        return;
    }
    let input = args.input.unwrap_or(String::new());
    if input.is_empty() {
        let mut cmd = Args::command();
        cmd.error(
            clap::error::ErrorKind::MissingRequiredArgument,
            "The required argument [INPUT] is missing.",
        )
        .exit()
    }
    let mut content: Vec<String> = match std::fs::read_to_string(input) {
        Ok(content) => content.lines().map(str::to_owned).collect(),
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };
    if args.blanks {
        content
            .iter_mut()
            .for_each(|elem| *elem = elem.trim().to_owned());
    }
    if args.check {
        if args.unique {
            content.dedup();
        }
        let unsorted = check_sorted(content);
        if !unsorted.is_empty() {
            println!("Unsorted line: {}", unsorted);
        }
        return;
    }
    if !args.key.is_empty() {
        for key in args.key {
            let num = args.numeric_sort > 0;
            let month = args.month_sort > 0;
            let human = args.human > 0;
            sort_by([num, month, human, args.reverse], &mut content, Some(key));
            if num {
                args.numeric_sort -= 1;
            }
            if month {
                args.month_sort -= 1;
            }
            if human {
                args.human -= 1;
            }
        }
    } else {
        sort_by(
            [
                args.numeric_sort > 0,
                args.month_sort > 0,
                args.human > 0,
                args.reverse,
            ],
            &mut content,
            None,
        );
    }
    if args.unique {
        content.dedup();
    }
    if let Err(err) = std::fs::write(&args.output, content.join("\n")) {
        eprintln!("Error while writing into {}: {err}", args.output);
    }
}
