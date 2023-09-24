struct Credidentials<T> where T: Fn(&str, &str) -> bool {
    username: String,
    password: String,
    validator: T,
}

fn main() {

    // let validator = |username: &str, password: &str| {
    //     !username.is_empty() && !password.is_empty()
    // };

    let validator = |username: &str, password: &str|
        !username.is_empty() && !password.is_empty();

    let weak_passowrd = "password123!".to_owned();

    // Fn - Immutably borrows variables in environment
    // FnMut - Mutably borrow variable in environment. Can change environment.
    // FnOnce Take ownership of variables in environment. Can only be called once.
    let validator2 = |username: &str, password: &str| {
        !username.is_empty() &&
            !password.is_empty() &&
            !password.len() >8 &&
            password.contains(['!', '@', '$', '%', '^', '&', '*' ]) &&
            password != weak_passowrd //immutably borrows weak password
    };

    println!("{weak_passowrd}");

    let creds = Credidentials {
        username: "".to_owned(),
        password: "".to_owned(),
        validator,
    };

    impl<T> Credidentials<T> where T: Fn(&str, &str) -> bool {
        fn is_valid(&self) -> bool {
            (self.validator)(&self.username, &self.password)
        }
    }

    println!("{}", validate_credidentails(&creds.username, &creds.password));
    println!("{}", validator(&creds.username, &creds.password));

    println!("{}", creds.is_valid());


    let creds2 = Credidentials {
        username: "aaa".to_owned(),
        password: "bbb".to_owned(),
        validator: validator2,
    };
    println!("{}", creds2.is_valid());
}

fn validate_credidentails(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}