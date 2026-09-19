use openconn::open_conn;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let mut conn = open_conn();

    println!(
        "Client: is_connected={}, has_socket={}",
        conn.is_connected,
        conn.socket.is_some()
    );

    // According to the spec: "if is_connected, send 'Hello world'".
    // This unwrap() will PANIC if the library returned is_connected=true with socket=None,
    // demonstrating the broken design.
    if conn.is_connected {
        let sock = conn.socket.as_mut().expect(
            "Bug triggered: is_connected == true but socket == None (from OpenConn)",
        );
        sock.write_all(b"Hello world")?;
        sock.flush()?;

        // Read echo from server until it closes
        let mut buf = Vec::with_capacity(1024);
        let mut tmp = [0u8; 1024];
        loop {
            let n = sock.read(&mut tmp)?;
            if n == 0 {
                break; // server closed
            }
            buf.extend_from_slice(&tmp[..n]);
        }

        println!("Client received: {}", String::from_utf8_lossy(&buf));
    } else {
        println!("Client: not connected, nothing to send.");
    }

    Ok(())
}
