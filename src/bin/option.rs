fn main() {
    let username = get_username(1);
    // match username {
    //     Some(name)=>println!("{name}"),
    //     None=> {}
    // }

    if let Some(name) = username{
        println!("{name}")
    }



}

fn get_username(user_id: u32) -> Option<String> {
    let db_result = String::from("Ferris");
    if user_id == 1 {
        Some(db_result)
    } else {
        None
    }
}