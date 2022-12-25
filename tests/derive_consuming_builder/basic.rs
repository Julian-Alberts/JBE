use jbe::ConsumingBuilder;

#[derive(ConsumingBuilder, PartialEq, Debug)]
pub struct User {
    id: usize,
    name: String,
    email: Option<String>
}

fn main() {
    let builder = UserBuilder::default();
    let user = builder.email(String::from("jon@example.com")).build(10, String::from("Jon"));
    assert_eq!(user, User {
        id: 10,
        name: String::from("Jon"),
        email: Some(String::from("jon@example.com"))
    });
}