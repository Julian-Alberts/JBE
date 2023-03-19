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
    let builder = TestBuilder::default().with_id(10).with_name(String::from("Jon")).with_email(String::from("jon@example.com"));
    let user = builder.build();
    assert_eq!(user, User {
        id: 10,
        name: String::from("Jon"),
        email: String::from("jon@example.com")
    });
}