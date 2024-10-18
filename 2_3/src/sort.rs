use crate::compare::{compare_as_memory, compare_as_months, compare_as_numeric};
use std::cmp::Ordering;

fn get_by_column(line: &str, column: usize) -> Option<&str> {
    line.split_whitespace().nth(column - 1)
}

pub fn sort_by(args: [bool; 4], content: &mut [String], column: Option<usize>) {
    let [num, month, human, reversed] = args;
    if num {
        sort_as(compare_as_numeric, content, reversed, column);
    } else if month {
        sort_as(compare_as_months, content, reversed, column);
    } else if human {
        sort_as(compare_as_memory, content, reversed, column);
    } else {
        sort(content, reversed);
    }
}

pub fn sort_as<F>(compare_as: F, content: &mut [String], reverse: bool, column: Option<usize>)
where
    F: Fn(&str, &str) -> Ordering,
{
    content.sort_by(|a, b| {
        let result = if let Some(column) = column {
            let a = get_by_column(a, column).unwrap_or("");
            let b = get_by_column(b, column).unwrap_or("");
            compare_as(a, b)
        } else {
            compare_as(a, b)
        };
        if reverse {
            result.reverse()
        } else {
            result
        }
    })
}

pub fn sort(content: &mut [String], reverse: bool) {
    content.sort_by(|a, b| {
        let result = a.cmp(b);
        if reverse {
            result.reverse()
        } else {
            result
        }
    })
}
