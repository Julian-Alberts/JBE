use jbe::Builder;

#[derive(Builder, PartialEq, Debug)]
pub struct User {
    id: usize,
    name: String,
    #[builder({
        default: String::from("empty")
    })]
    email: String
}

fn main() {
    let mut builder = UserBuilder::default();
    let user = builder.id(10).name(String::from("Jon")).build();
    assert_eq!(user, User {
        id: 10,
        name: String::from("Jon"),
        email: String::from("empty")
    });
}