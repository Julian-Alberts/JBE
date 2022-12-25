use jbe::ConsumingBuilder;

#[derive(ConsumingBuilder, PartialEq, Debug)]
#[consuming_builder({
    builder_ident: TestBuilder
})]
pub struct User {
    id: usize,
    name: String,
    email: String
}

fn main() {
    let builder = TestBuilder::default();
    let user = builder.build(10, String::from("Jon"), String::from("jon@example.com"));
    assert_eq!(user, User {
        id: 10,
        name: String::from("Jon"),
        email: String::from("jon@example.com")
    });
}