use std::io;
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
        //channel.exec(command.as_str()).unwrap();
        let mut buffer = [0; 20 * 1024];
        channel.exec(command.as_str()).unwrap();
        while channel.read(&mut buffer[..]).unwrap() > 0 {
            io::stdout().write_all(&buffer[..]).unwrap();
        }
        channel.wait_close().unwrap();
        Ok("".to_string())
    } else {
        return Err(Error::new(ErrorCode::Session(0), "Error connecting to ssh server"));
    }
}

pub fn execute_command(command: String, sess: Session) -> Result<String, Error> {
    Ok("".to_string())
}
