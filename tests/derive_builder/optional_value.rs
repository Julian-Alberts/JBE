use jbe::Builder;

#[derive(Builder, PartialEq, Debug)]
#[builder({copy: true})]
pub struct User {
    id: usize,
    name: String,
    email: Option<String>
}

fn main() {
    let builder = UserBuilder::default();
    let user = builder.with_id(10).with_name(String::from("Jon")).with_email(String::from("jon@example.com")).build();
    assert_eq!(user, User {
        id: 10,
        name: String::from("Jon"),
        email: Some(String::from("jon@example.com"))
    });
}