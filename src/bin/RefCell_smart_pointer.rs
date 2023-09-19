use std::rc::Rc;
use std::cell::RefCell;

struct Database{
    max_connection: u32,
}

struct AuthService {
    db: Rc<RefCell<Database>>
}
struct ContentService {
    db: Rc<RefCell<Database>>
}
fn main() {
    //Not multi threaded applications
    let db =  Rc::new( RefCell::new(Database { //refcell wrapper is needed for mutable references
        max_connection: 100
    }));
    let auth_service = AuthService {db: Rc::clone(&db)}; //This does does clone, It simply increments the reference count //similar to shared smart pointer in c++
    let content_service = ContentService {db: Rc::clone(&db)};

    //RefCell use with caution unsafe in safe wrapper
    let mut r1 = db.borrow_mut();
    let r2 = db.borrow_mut();
    r1.max_connection =201;

    //this will cause: " thread 'main' panicked at 'already borrowed: BorrowMutError', src/bin/RefCell_smart_pointer.rs:22:17"

}