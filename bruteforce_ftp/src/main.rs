use cli::Args;
use clap::Parser;
use std::net::TcpStream;
use pwd::get_pwd;
use crate::ftp::ftp_connect;

mod cli;
mod ftp;
mod pwd;

fn main() {
    let args: Args = Args::parse();
    println!("{:?}", args);

    match TcpStream::connect(format!("{}:{}", args.ip.clone(), args.port)) {
        Ok(_) => {},
        Err(err) => {
            panic!("{}", err)
        },
    }

    let passwords = get_pwd(args.pwd_txt);
    for pwd in passwords {
        ftp_connect(args.name.clone(), pwd, args.ip.clone(), args.port)
    }
}
