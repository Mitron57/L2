mod args;
mod report;

use args::Args;
use clap::Parser;
use report::Report;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;

fn count_chars_by<F>(line: &str, counter: F)
where
    F: FnMut(char),
{
    line.chars()
        .filter(char::is_ascii_alphabetic)
        .for_each(counter);
}

fn count_into_btree(count_map: &mut BTreeMap<char, usize>, elem: char, count: usize) {
    count_map
        .entry(elem.to_ascii_lowercase())
        .and_modify(|v| *v += count)
        .or_insert(count);
}

fn count_into(count_map: &mut HashMap<char, usize>, elem: char, count: usize) {
    count_map
        .entry(elem.to_ascii_lowercase())
        .and_modify(|v| *v += count)
        .or_insert(count);
}

fn start_worker(sender: Sender<HashMap<char, usize>>, chunk: String) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let mut count_map = HashMap::new();
        count_chars_by(&chunk, |elem| {
            count_into(&mut count_map, elem, 1);
        });
        sender.send(count_map).unwrap();
    })
}

fn count_multithreaded(content: &str, threads: usize) -> Receiver<HashMap<char, usize>> {
    let (tx, rx) = channel();
    let chunk_size = content.len() / threads;
    for i in 0..threads - 1 {
        let low = i * chunk_size;
        let high = low + chunk_size;
        let chunk = content[low..high].to_owned();
        start_worker(tx.clone(), chunk);
    }
    let low = chunk_size * (threads - 1);
    start_worker(tx, content[low..].to_owned());
    rx
}

fn main() {
    let args = Args::parse();
    let start = std::time::Instant::now();
    let mut content = Vec::new();
    let mut file = match File::open(args.file) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("There was a problem opening the file: {}", error);
            return;
        }
    };
    if let Err(err) = file.read_to_end(&mut content) {
        eprintln!("There was a problem reading the file: {}", err);
        return;
    }
    let content = match String::from_utf8(content) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("String contains non-utf8 characters: {}", err);
            return;
        }
    };
    let mut report = Report::default();
    if args.threads != 1 {
        let receiver = count_multithreaded(&content, args.threads);
        while let Ok(count_map) = receiver.recv() {
            for (elem, count) in count_map {
                count_into_btree(&mut report.result, elem, count);
            }
        }
    } else {
        count_chars_by(&content, |elem| {
            count_into_btree(&mut report.result, elem, 1);
        });
    }
    let stop = start.elapsed();
    report.elapsed = stop;
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
