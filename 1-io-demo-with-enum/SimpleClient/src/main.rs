use openconn::{open_conn, ConnectionState};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let mut conn = open_conn();

    match &mut conn {
        ConnectionState::Connected { socket } => {
            println!("Client: connected, sending 'Hello world'");

            socket.write_all(b"Hello world")?;
            socket.flush()?;

            let mut buf = Vec::with_capacity(1024);
            let mut tmp = [0u8; 1024];
            loop {
                let n = socket.read(&mut tmp)?;
                if n == 0 {
                    break;
                }
                buf.extend_from_slice(&tmp[..n]);
            }

            println!("Client received: {}", String::from_utf8_lossy(&buf));
        }
        ConnectionState::Disconnected => {
            println!("Client: not connected, nothing to send.");
        }
        ConnectionState::Connecting { .. } => {
            println!("Client: still connecting…");
        }
    }

    Ok(())
}
