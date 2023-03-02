use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, short, default_value = "root")]
    pub name: String,

    #[arg(long, short)]
    pub ip: String,

    #[arg(long, short, default_value_t = 22)]
    pub port: u32,

    #[arg(long)]
    pub pwd_txt: Option<String>,
}