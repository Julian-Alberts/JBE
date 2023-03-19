use jbe::Builder;

#[derive(Builder, PartialEq, Debug)]
#[builder({copy: true})]
pub struct Data<T: Clone> {
    i: usize,
    data: T
}

fn main() {
    let builder = DataBuilder::default();
    let user = builder.with_i(10).with_data(String::from("Jon")).build();
    assert_eq!(user, Data {
        i: 10,
        data: String::from("Jon")
    });
}