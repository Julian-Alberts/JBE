use jbe::Builder;

#[derive(Builder, PartialEq, Debug)]
#[builder({
    error_ident: TestError
})]
pub struct User {
    id: usize,
    name: String,
    email: String
}

fn main() {
    let builder = UserBuilder::default();
    let user = builder.with_id(10).with_name(String::from("Jon")).try_build();
    assert_eq!(user, Err(TestError::UnsetEmail));
}