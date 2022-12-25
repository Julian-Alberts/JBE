use jbe::ConsumingBuilder;

#[derive(ConsumingBuilder, PartialEq, Debug)]
pub struct User {
    id: usize,
    name: String,
    #[consuming_builder({
        default: String::from("empty")
    })]
    email: String
}

fn main() {
    let user = UserBuilder::default().build(10, String::from("Jon"));
    assert_eq!(user, User {
        id: 10,
        name: String::from("Jon"),
        email: String::from("empty")
    });
    let user = UserBuilder::default().email(String::from("jon@example.com")).build(10, String::from("Jon"));
    assert_eq!(user, User {
        id: 10,
        name: String::from("Jon"),
        email: String::from("jon@example.com")
    });
}