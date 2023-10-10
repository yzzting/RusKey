use serde::Deserialize;
use clap::Parser;

#[derive(Parser, Deserialize, Debug)]
#[command(author, version, about, long_about = "rus key is a simple key-value store.", disable_help_flag(true))]
pub struct Opt {
    #[arg(short, long = "host")]
    pub host: Option<String>,

    #[arg(short, long = "port")]
    pub port: Option<String>,

    #[arg(short = 'A', long = "password")]
    pub password: Option<String>,

    // #[command(subcommand)]
    // command: Option<String>,
}

impl Opt {
    pub fn get(&self, key: &str) -> Option<&String> {
        match key {
            "host" => self.host.as_ref(),
            "port" => self.port.as_ref(),
            "password" => self.password.as_ref(),
            _ => None,
        }
    }
}