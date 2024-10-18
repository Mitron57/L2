use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, default_value = "10s")]
    pub timeout: String,

    #[arg(required = true)]
    pub host: String,

    #[arg(required = true)]
    pub port: u16,
}
