use std::rc::Rc;

struct Database {}

struct Authservice {
    db: Rc<Database>,
}

struct ContentService {
    db: Rc<Database>,
}

fn main() {
    let db = Rc::new(Database {});
    let auth_service = Ref::clone(Authservice { db: db });
    let content_service = Ref::clone(Authservice { db: db });
}
