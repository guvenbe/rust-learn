use data_model::{NewUser, User};

#[test]
fn integration_builds_user_with_id() {
    let new = NewUser { name: "Bob".into() };
    let u = User::from_new(new, 7).unwrap();
    assert_eq!(u.id, 7);
    assert_eq!(u.name, "Bob");
}
