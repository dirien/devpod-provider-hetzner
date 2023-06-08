use std::{io};
use std::io::{Read, Write};
use std::net::TcpStream;
use ssh2::{Error, ErrorCode, Session};


pub async fn new_ssh_client(user: String, ip: String, privatekey: String, command: String) -> Result<String, Error> {
    if let Ok(stream) = TcpStream::connect(format!("{}:22", ip)) {
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(stream);
        sess.handshake().unwrap();

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            sess.userauth_pubkey_memory(&user, None, &privatekey, None).unwrap();
        }
        #[cfg(target_os = "windows")]
        {
            sess.userauth_pubkey_file(&user, None, std::path::Path::new(&privatekey), None).unwrap();
        }
        let mut channel = sess.channel_session().unwrap();
        channel.exec(command.as_str()).unwrap();

        let mut buffer = [0; 100 * 1024];
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            let size = match channel.read(&mut buffer) {
                Ok(size) => size,
                Err(_) => break,
            };
            if size > 0 {
                let data = std::str::from_utf8(&buffer[..size]).unwrap();
                if data.contains("ping") {
                    stdout.write_all(&buffer[..size]).unwrap();
                    stdin.read_exact(&mut buffer[..size]).unwrap();
                    channel.write_all(&buffer[..size]).unwrap();
                } else {
                    stdout.write_all(&buffer[..size]).unwrap();
                    let mut s = String::new();
                    channel.stderr().read_to_string(&mut s).unwrap();
                    //print!("{}", s);
                    //io::stdout().write_all(s.as_bytes()).unwrap();
                    eprint!("{}", s);
                    //io::stdout().write_all(&buffer[..size]).unwrap();
                }
            } else {
                channel.send_eof().unwrap();
                channel.wait_close().unwrap();
                break;
            }
        }
        Ok("".to_string())
    } else {
        return Err(Error::new(ErrorCode::Session(0), "Error connecting to ssh server"));
    }
}

pub fn execute_command(command: String, sess: Session) -> Result<String, Error> {
    Ok("".to_string())
}
