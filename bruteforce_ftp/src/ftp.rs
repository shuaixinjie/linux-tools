use ftp::FtpStream;

pub fn ftp_connect(user: String, pass: String, ip: String, port: u32) {
    let mut ftp_stream = FtpStream::connect(format!("{}:{}", ip, port)).unwrap();
    match ftp_stream.login(&user, &pass) {
        Ok(_) => {
            println!("[+] login success! Username: {} | Password: {}", user, pass)
        }
        Err(_) => {
            return
        }
    }
}