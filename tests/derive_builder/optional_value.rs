use jbe::Builder;

#[derive(Builder, PartialEq, Debug)]
pub struct User {
    id: usize,
    name: String,
    email: Option<String>
}

fn main() {
    let mut builder = UserBuilder::default();
    let user = builder.id(10).name(String::from("Jon")).email(String::from("jon@example.com")).build();
    assert_eq!(user, User {
        id: 10,
        name: String::from("Jon"),
        email: Some(String::from("jon@example.com"))
    });
}