use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser)]
#[command(version)]
pub struct Config {
    #[clap(env = "API_HOST", default_value = "127.0.0.1")]
    pub host: String,

    #[clap(env = "API_PORT", default_value = "8080")]
    pub port: u16,
}

pub fn conf() -> &'static Config {
    lazy_static! {
        static ref CONFIG: Config = Config::parse();
    }

    &CONFIG
}
