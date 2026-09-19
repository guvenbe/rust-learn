use std::net::TcpStream;

/// Error-prone design with flags and optional fields
/// Can be None even when `is_connected` is true (buggy behavior on purpose).
pub struct BadConnection {
    pub is_connected: bool,
    pub socket: Option<TcpStream>,
}

/// Opens a connection to SimpleServer on 127.0.0.1:8080 and returns `BadConnection`.
/// BUG (intentional): it sets `is_connected = true` even if the socket fails to open,
/// returning `socket = None` without flipping `is_connected` to false.
pub fn open_conn() -> BadConnection {
    let destino = "127.0.0.1:8080";

    // BUG: unconditionally mark as "connected"
    let mut conn = BadConnection {
        is_connected: true,
        socket: None,
    };

    // Try to open the socket; on error, leave `is_connected` as true (bug).
    if let Ok(sock) = TcpStream::connect(destino) {
        conn.socket = Some(sock);
    }
    conn
}
