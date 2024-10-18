use std::error::Error;
use std::time::Duration;

pub fn parse(time: &str) -> Result<Duration, Box<dyn Error>> {
    let duration = time
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>();
    let unit = time.strip_prefix(&duration).unwrap();
    let duration = duration.parse::<u64>()?;
    match unit {
        "s" => Ok(Duration::from_secs(duration)),
        "ms" => Ok(Duration::from_millis(duration)),
        "m" => Ok(Duration::from_secs(duration * 60)),
        _ => Err(format!("unknown unit: {}", unit).into()),
    }
}
