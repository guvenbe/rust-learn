use std::rc::Rc;
struct Database {}

struct AuthService {
    db: Rc<Database>,
}
struct ContentService {
    db: Rc<Database>,
}
fn main() {
    //Not multi threaded applications
    let db = Rc::new(Database {});
    let auth_service = AuthService { db: Rc::clone(&db) }; //This does not clone, It simply increments the reference count //similar to shared smart pointer in c++
    let content_service = ContentService { db: Rc::clone(&db) };
}
