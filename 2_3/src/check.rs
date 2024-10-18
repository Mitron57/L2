use std::fmt::Display;

pub fn check_sorted<T: PartialOrd + Display>(lines: Vec<T>) -> String {
    for i in 0..lines.len() - 1 {
        if lines[i + 1] < lines[i] {
            return lines[i + 1].to_string();
        }
    }
    String::new()
}
