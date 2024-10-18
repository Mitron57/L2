use clap::{Parser, ArgAction};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(disable_help_flag = true)]
pub struct Args {
    #[arg(
        short,
        long,
        requires = "input",
        value_name = "POS",
        help = "Start a key at POS",
        action = ArgAction::Append
    )]
    pub key: Vec<usize>,

    #[arg(
        short('n'),
        long,
        requires = "input",
        help = "Compare according to string numerical value",
        action = ArgAction::Count
    )]
    pub numeric_sort: u8,

    #[arg(short, long, requires = "input", help = "Reverse the result of comparisons")]
    pub reverse: bool,

    #[arg(short, long, requires = "input", help = "Print only unique values into [OUTPUT]")]
    pub unique: bool,

    #[arg(
        short('M'), 
        long, 
        requires = "input", 
        help = "Compare (unknown) < 'JAN' < ... < 'DEC'", 
        action = ArgAction::Count
    )]
    pub month_sort: u8,

    #[arg(
        short,
        long("ignore-leading-blanks"),
        requires = "input",
        help = "Ignore leading and trailing blanks"
    )]
    pub blanks: bool,

    #[arg(short, long, requires = "input", help = "Check for sorted input; do not sort")]
    pub check: bool,

    #[arg(
        short,
        long("human-numeric-sort"),
        requires = "input",
        help = "Compare human readable numbers (e.g., 2K 1G)",
        action = ArgAction::Count
    )]
    pub human: u8,

    #[arg(long, help = "Print this message")]
    pub help: bool,

    #[arg(help = "required")]
    pub input: Option<String>,

    #[arg(short, long, default_value = "sorted.txt")]
    pub output: String,
}