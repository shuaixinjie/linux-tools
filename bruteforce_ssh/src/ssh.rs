use std::net::TcpStream;
use std::io::Error;

use ssh_rs::ssh;

pub fn port_connect(host: impl Into<String>, port: u32) -> Option<Error> {
    match TcpStream::connect(format!("{}:{}", host.into(), port)) {
        Ok(_) => None,
        Err(err) => Some(err),
    }
}

pub fn ssh_connect(username: String, password: String, ip: String, port: u32) {
    let session = ssh::create_session()
        .username(username.as_str())
        .password(password.as_str())
        .connect(format!("{}:{}", ip, port));

    match session {
        Ok(_) => {
            println!("[+] login success! Username: {} | Password: {}", username, password);
            return
        }
        Err(_) => {}
    }
}


