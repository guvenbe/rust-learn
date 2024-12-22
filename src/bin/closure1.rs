struct Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    username: String,
    password: String,
    validator: T,
}

impl<T> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}
fn main() {
    let validator = |username: &str, password: &str| !username.is_empty() && !password.is_empty();
    
    let weak_password = "password123!";

    let validator2 = move |username: &str, password: &str| {
        !username.is_empty()
            && !password.is_empty()
            && password.len() > 8
            && password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
            && password != weak_password
    };
    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password123".to_owned(),
        validator: validator2,
    };

    println!("{}", validate_credentails(&creds.username, &creds.password));
    println!("{}", validator(&creds.username, &creds.password));
    //instead now we can call is valid with which excepts a closure
    println!("{}", creds.is_valid());
}
fn validate_credentails(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}
