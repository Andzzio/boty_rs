use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Root {
    #[arg(short, long, default_value = "Config.toml")]
    pub config_path: PathBuf,
}
