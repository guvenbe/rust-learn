use data_model::{NewUser, User};

fn main() {
    let new_user = NewUser { name: "Alice".into() };
    let user = User::from_new(new_user, 1).expect("valid user");
    println!("{user:?}");
}
