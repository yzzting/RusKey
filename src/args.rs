use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "rus key is a simple key-value store.", disable_help_flag(true))]
pub struct Opt {
    #[arg(short, long = "host", default_value = "")]
    pub host: String,

    #[arg(short, long = "port", default_value = "")]
    pub port: String,

    #[arg(short = 'A', long = "password")]
    pub password: Option<String>,

    // #[command(subcommand)]
    // command: Option<String>,
}