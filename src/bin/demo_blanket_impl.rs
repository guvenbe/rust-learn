trait IdentifyUser {
    // return user id to identify them
    fn get_user_id(&self) -> u32;
}

struct User {
    user_id: u32,
}

impl IdentifyUser for User {
    fn get_user_id(&self) -> u32 {
        self.user_id
    }
}
struct PowerUser {
    user_id: u32,
}

impl IdentifyUser for PowerUser {
    fn get_user_id(&self) -> u32 {
        self.user_id
    }
}

struct PoserUser {
    user_id: u32,
}

trait AuthenticateUser {
    fn authenticate(&self) -> bool;
}

impl<T> AuthenticateUser for T
where
    T: IdentifyUser,
{
    fn authenticate(&self) -> bool {
        self.get_user_id() % 2 == 0
    }
}
fn main() {
    let user = User { user_id: 42 };

    // using the `get_user_id` method from the `IdentifyUser` trait
    println!("User ID: {}", user.get_user_id());
    println!("Authenticated: {}", user.authenticate());
    let user = PowerUser { user_id: 42 };

    // using the `get_user_id` method from the `IdentifyUser` trait
    println!("Power User ID: {}", user.get_user_id());
    println!("Power User Authenticated: {}", user.authenticate());
    let user = User { user_id: 41 };

    // using the `get_user_id` method from the `IdentifyUser` trait
    println!("User ID: {}", user.get_user_id());
    println!("Authenticated: {}", user.authenticate());
}
