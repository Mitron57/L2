use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, num_args = .., value_delimiter = ' ', help = "Select only these fields;")]
    pub fields: Vec<usize>,

    #[arg(short, long, help = "Use DELIM instead of TAB for field delimiter")]
    pub delimiter: Option<String>,

    #[arg(
        short('s'),
        long,
        help = "Do not print lines not containing delimiters"
    )]
    pub only_delimited: bool,
}
