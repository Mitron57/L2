use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Args {
    #[arg(
        short,
        default_value_t = 1,
        help = "Specify how many workers will proceed the data"
    )]
    pub threads: usize,

    #[arg(required = true, help = "File with content to process")]
    pub file: String,
}
