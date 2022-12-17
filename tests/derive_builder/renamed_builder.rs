use jbe::Builder;

#[derive(Builder, PartialEq, Debug)]
#[builder({
    builder_ident: TestBuilder
})]
pub struct User {
    id: usize,
    name: String,
    email: String
}

fn main() {
    let mut builder = TestBuilder::default();
    let user = builder.id(10).name(String::from("Jon")).email(String::from("jon@example.com")).build();
    assert_eq!(user, User {
        id: 10,
        name: String::from("Jon"),
        email: String::from("jon@example.com")
    });
}