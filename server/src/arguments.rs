use clap::Parser;

#[derive(Parser, Default, Debug, Clone)]
#[command(author = "aeghn", version = "0.1", about = "chnots server")]
pub struct Arguments {
    #[clap(long, short, help = "Config file to read")]
    pub config: String,
}

unsafe impl Sync for Arguments {}
