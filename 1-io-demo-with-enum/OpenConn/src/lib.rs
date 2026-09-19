use std::net::TcpStream;
use std::time::Duration;

/// Enum-based design guarantees valid states
pub enum ConnectionState {
    Disconnected,
    Connecting { timeout: Duration },
    Connected { socket: TcpStream },
}

/// Try to connect to 127.0.0.1:8080.
/// Returns either Connected with a valid TcpStream or Disconnected.
pub fn open_conn() -> ConnectionState {
    let destino = "127.0.0.1:8080";

    // Here we try to connect. If it works, we return Connected.
    // If it fails, we return Disconnected. No invalid "half-connected" state is possible.
    match TcpStream::connect(destino) {
        Ok(sock) => ConnectionState::Connected { socket: sock },
        Err(_) => ConnectionState::Disconnected,
    }
}
