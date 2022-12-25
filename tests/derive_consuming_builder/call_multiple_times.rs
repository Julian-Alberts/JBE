
use jbe::ConsumingBuilder;

#[derive(ConsumingBuilder, PartialEq, Debug)]
pub struct User {
    id: usize,
    name: String,
    email: Option<String>
}

fn main() {
    let builder = UserBuilder::default().email(String::from("jon@example.com"));
    let user = builder.build(10, String::from("Jon"));
    assert_eq!(user, User {
        id: 10,
        name: String::from("Jon"),
        email: Some(String::from("jon@example.com"))
    });
    let user = builder.build(11, String::from("Jon2"));
    assert_eq!(user, User {
        id: 11,
        name: String::from("Jon2"),
        email: Some(String::from("jon@example.com"))
    });
}