use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf)?;
    if n > 0 {
        // Echo back exactly what we received
        stream.write_all(&buf[..n])?;
        stream.flush()?;
    }
    // Drop the stream here (closing the connection) so the client can finish reading.
    Ok(())
}

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr)?;
    println!("SimpleServer listening on {addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle each connection (simple, single-threaded loop)
                if let Err(e) = handle_client(stream) {
                    eprintln!("Client error: {e}");
                }
            }
            Err(e) => eprintln!("Accept error: {e}"),
        }
    }
    Ok(())
}
