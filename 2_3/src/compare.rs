use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

pub fn compare_as_numeric(a: &str, b: &str) -> Ordering {
    match (a.parse::<i64>(), b.parse::<i64>()) {
        (Ok(a), Ok(b)) => a.cmp(&b),
        _ => a.cmp(b),
    }
}

pub fn compare_as_months(a: &str, b: &str) -> Ordering {
    const MONTHS: [&str; 12] = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];
    if a.len() < 3 || b.len() < 3 {
        return a.cmp(b);
    }
    let a_month = MONTHS
        .iter()
        .position(|&m| a[..3].to_ascii_lowercase() == m);
    let b_month = MONTHS
        .iter()
        .position(|&m| b[..3].to_ascii_lowercase() == m);

    match (a_month, b_month) {
        (Some(a), Some(b)) => a.cmp(&b),
        _ => a.cmp(b),
    }
}

fn parse_number(iter: &mut Peekable<Chars>) -> Option<i64> {
    let mut num = String::new();
    while let Some(&ch) = iter.peek() {
        if ch.is_ascii_digit() {
            num.push(ch);
            iter.next();
        } else {
            break;
        }
    }
    num.parse::<i64>().ok()
}

fn get_unit_value(unit: char) -> i32 {
    match unit {
        'B' => 0,
        'K' => 1,
        'M' => 2,
        'G' => 3,
        'T' => 4,
        'P' => 5,
        'E' => 6,
        _ => -1,
    }
}

pub fn compare_as_memory(a: &str, b: &str) -> Ordering {
    let mut a_iter = a.chars().peekable();
    let mut b_iter = b.chars().peekable();
    let num_a = parse_number(&mut a_iter);
    let num_b = parse_number(&mut b_iter);
    match (num_a, num_b) {
        (Some(num_a), Some(num_b)) => {
            let unit_a = a_iter.next().map(get_unit_value).unwrap_or(-1);
            let unit_b = b_iter.next().map(get_unit_value).unwrap_or(-1);
            if unit_a == -1 || unit_b == -1 {
                return a.cmp(b);
            }
            match unit_a.cmp(&unit_b) {
                Ordering::Equal => num_a.cmp(&num_b),
                Ordering::Less => (num_a / 2_i64.pow(10 * (unit_b - unit_a) as u32)).cmp(&num_b),
                Ordering::Greater => (num_a * 2_i64.pow(10 * (unit_a - unit_b) as u32)).cmp(&num_b),
            }
        }
        _ => a.cmp(b),
    }
}
