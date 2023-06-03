use std::io::{Read};
use std::net::TcpStream;
use ssh2::{Session, Error, ErrorCode};

pub fn new_ssh_client(user: String, ip: String, privatekey: String) -> Result<Session, Error> {
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
        Ok(sess)
    } else {
        return Err(Error::new(ErrorCode::Session(0), "Error connecting to ssh server"));
    }
}

pub fn execute_command(command: String, sess: Session) -> Result<String, Error> {
    let mut channel = sess.channel_session().unwrap();
    channel.exec(command.as_str()).unwrap();
    let mut s = String::new();
    let result = channel.read_to_string(&mut s);
    match result {
        Ok(_) => {}
        Err(err) => {
            println!("Error reading from ssh channel: {}", err);
            return Err(Error::new(ErrorCode::Session(-39), "Error reading from ssh channel"));
        }
    }
    channel.wait_close().unwrap();
    Ok(s)
}
