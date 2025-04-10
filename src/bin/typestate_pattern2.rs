struct BusTicket;
struct BoardedBusTicket;
impl BusTicket {
    fn board(self) -> BoardedBusTicket {
        BoardedBusTicket
    }
}
fn main() {
    let ticket = BusTicket;
    let boarded = ticket.board();
    //preventing doing following because ownership moved
    //enforcing state in sequence
    // ticket.board();
}