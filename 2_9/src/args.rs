use clap::Parser;
#[derive(Parser)]
pub struct Args {
    #[arg(required = true)]
    pub url: String,
}
