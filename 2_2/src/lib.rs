mod unpack_error;

use std::iter::Peekable;
pub use unpack_error::UnpackError;

fn unescape(chars: &mut impl Iterator<Item = char>) -> Result<char, UnpackError> {
    match chars.next() {
        Some(next_char) => {
            if next_char.is_ascii_digit() || next_char == '\\' {
                Ok(next_char)
            } else {
                let next = next_char.to_string();
                Err(UnpackError::InvalidEscaping("\\".to_owned() + &next))
            }
        }
        None => Err(UnpackError::InvalidEscaping("\\".to_owned())),
    }
}

fn parse_number(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<usize, UnpackError> {
    if let Some(next_char) = chars.peek() {
        if next_char.is_ascii_digit() {
            let mut repeat_str = String::new();
            while let Some(num) = chars.peek() {
                if num.is_ascii_digit() {
                    let num = chars.next().unwrap();
                    repeat_str.push(num);
                } else {
                    break;
                }
            }
            return match repeat_str.parse() {
                Ok(repeat) => Ok(repeat),
                Err(_) => Err(UnpackError::InvalidRepeatsNumber(repeat_str)),
            };
        }
    }
    Ok(1)
}

pub fn unpack(line: &str) -> Result<String, UnpackError> {
    if line.is_empty() {
        return Ok(String::new());
    }
    let mut chars = line.chars().peekable();
    let &first = chars.peek().unwrap();
    if first.is_ascii_digit() {
        return Err(UnpackError::StartsWithNumber(first));
    }
    let mut result = String::new();
    while let Some(mut elem) = chars.next() {
        if elem == '\\' {
            elem = unescape(&mut chars)?;
        }
        let repeat = parse_number(&mut chars)?;
        result.extend(std::iter::repeat(elem).take(repeat));
    }
    Ok(result)
}
