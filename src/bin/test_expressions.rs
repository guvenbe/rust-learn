enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn print_message(is_gt_100: bool) {
    match is_gt_100 {
        true => println!("is big"),
        _ => println!("is small"),
    }
}
fn main() {
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };
    println!("can access file: {:?}", can_access_file);
    let value = 100;
    let is_gt_100 = value > 100;
    print_message(is_gt_100);
}
