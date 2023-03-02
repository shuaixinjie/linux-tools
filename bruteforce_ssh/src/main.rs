use clap::Parser;
use cli::Args;
use crate::pwd::Password;
use crate::ssh::{port_connect, ssh_connect};

mod cli;
mod pwd;
mod ssh;

fn main() {
    let args: Args = Args::parse();
    println!("{:?}", args);

    match port_connect(args.ip.clone(), args.port) {
        Some(err) => panic!("{}", err),
        None => {},
    }

    let passwords = Password::get_passwords(args.pwd_txt);
    for pwd in passwords {
        ssh_connect(args.name.clone(), pwd, args.ip.clone(), args.port)
    }
}
