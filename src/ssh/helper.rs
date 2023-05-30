use std::io::{Read};
use std::net::TcpStream;
use ssh2::{Session, Error, ErrorCode};

pub fn new_ssh_client(user: String, ip: String, privatekeydata: String) -> Result<Session, Error> {
    if let Ok(stream) = TcpStream::connect(format!("{}:22", ip)) {
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(stream);
        sess.set_timeout(5000);
        sess.handshake().unwrap();
        sess.userauth_pubkey_memory(&user, None, &privatekeydata, None).unwrap();
        Ok(sess)
    } else {
        return Err(Error::new(ErrorCode::Session(0), "Error connecting to ssh server"));
    }
}

pub fn execute_command(command: String, sess: Session) -> Result<String, Error> {
    let mut channel = sess.channel_session().unwrap();
    channel.exec(command.as_str()).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    channel.wait_close().unwrap();
    Ok(s)
}
