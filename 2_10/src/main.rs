mod args;
mod duration_parser;

use crate::args::Args;
use clap::Parser;
use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream};

fn main() {
    let args = Args::parse();
    let timeout = match duration_parser::parse(&args.timeout) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };
    let ip_addr = match args.host.parse() {
        Ok(addr) => addr,
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };
    let address = SocketAddr::new(ip_addr, args.port);
    let mut stream = match TcpStream::connect_timeout(&address, timeout) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };
    let mut input = String::new();
    while let Ok(bytes) = std::io::stdin().read_line(&mut input) {
        if bytes == 0 {
            let _ = stream.flush();
            break;
        }
        if let Err(err) = stream.write_all(input.as_bytes()) {
            eprintln!("Error: {err}");
            match err.kind() {
                ErrorKind::BrokenPipe
                | ErrorKind::ConnectionRefused
                | ErrorKind::ConnectionAborted => break,
                _ => {}
            }
        }
        input.clear();
        match stream.read_to_string(&mut input) {
            Ok(_) => {
                println!("{}", input);
            }
            Err(err) => {
                eprintln!("Error: {err}");
            }
        }
        input.clear();
    }
}
