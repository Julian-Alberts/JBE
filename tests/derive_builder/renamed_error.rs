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
    let mut builder = UserBuilder::default();
    let user = builder.id(10).name(String::from("Jon")).try_build();
    assert_eq!(user, Err(TestError::UnsetEmail));
}