use jbe::Builder;

#[derive(Builder, PartialEq, Debug)]
#[builder({copy: true})]
pub struct User {
    id: usize,
    name: String,
    #[builder({
        default: String::from("empty")
    })]
    email: String
}

fn main() {
    let user = UserBuilder::default().with_id(10).with_name(String::from("Jon")).build();
    assert_eq!(user, User {
        id: 10,
        name: String::from("Jon"),
        email: String::from("empty")
    });
}