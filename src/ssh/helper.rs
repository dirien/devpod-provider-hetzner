use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use ssh2::{ErrorCode};
use openssh::*;


pub async fn new_ssh_client(user: String, ip: String, privatekey: String, command: String) -> Result<String, Error> {
    let session = SessionBuilder::default()
        .user(user)
        .keyfile(privatekey)
        .port(22)
        .connect(ip).await.unwrap();

    let ls = session.raw_command(command.as_str()).output().await.unwrap();
    print!("{}", String::from_utf8(ls.stdout).unwrap());


    session.close().await.unwrap();
    Ok("".to_string())
    /*
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
        let mut mode = ssh2::PtyModes::new();
        mode.
        channel.request_pty("xterm", None, None).unwrap();
        channel.shell().unwrap();
        //channel.exec(command.as_str()).unwrap();
        channel.write(command.as_ref()).unwrap();
        channel.write(b"\n").unwrap();
        while channel.read(&mut buffer[..]).unwrap() > 0 {
            io::stdout().write_all(&buffer[..]).unwrap();
        }
        channel.write(b"exit\n").unwrap();
        channel.wait_close().unwrap();
        Ok("".to_string())
    } else {
        return Err(Error::new(ErrorCode::Session(0), "Error connecting to ssh server"));
    }

     */
}

pub fn execute_command(command: String, sess: Session) -> Result<String, Error> {
    Ok("".to_string())
}
