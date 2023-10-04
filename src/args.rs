use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "rus key is a simple key-value store.", disable_help_flag(true))]
pub struct Opt {
    #[arg(short, long = "host", default_value = "127.0.0.1")]
    pub host: String,

    #[arg(short, long = "port", default_value = "16379")]
    pub port: u16,

    #[arg(short = 'A', long = "password")]
    pub password: Option<String>,

    // #[command(subcommand)]
    // command: Option<String>,
}