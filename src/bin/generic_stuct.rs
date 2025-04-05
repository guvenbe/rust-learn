trait Seat {
    fn show(&self);
}

struct Ticket<T: Seat> {
    location: T,
}
#[derive(Debug)]
enum ConcertSeat {
    FrontRow,
    MidSection(u32),
    Back(u32),
}
impl Seat for ConcertSeat {
    fn show(&self) {
        println!("Seat: {:?}", &self);
    }
}
#[derive(Debug)]
enum AirlineSeat {
    BusinessClass,
    Economy,
    FirstClass,
}
impl Seat for AirlineSeat {
    fn show(&self) {
        println!("Seat: {:?}", &self);
    }
}

fn ticket_info<T: Seat>(ticket: Ticket<T>) {
    ticket.location.show();
}

fn main() {
    let airline = Ticket {
        location: AirlineSeat::FirstClass,
    };
    let concert  = Ticket {
        location: ConcertSeat::FrontRow,
    };
    
    ticket_info(airline);
    ticket_info(concert);
}
