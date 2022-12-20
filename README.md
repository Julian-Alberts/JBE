# JBE (WIP)
JBE allows you to generate a builder for your struct by using a derive macro and comes with two different macros, Builder and ConsumingBuilder. 
(The ConsumingBuilder macro is not yet implemented.)
The difference is how they construct the object in the end. 
While Builder copies the values to the newly created struct, a ConsumingBuilder moves ownership to the new struct.
By default, the builder has the same name as the struct followed by `Builder`.
```rust
#[derive(PartialEq, Builder)]
pub struct MyStruct {
    a: u8
}

fn main() {
    let my_struct = MyStructBuilder::new().a(25).build();
    assert_eq!(my_struct, MyStruct { a: 25 })
}
```

## Macros
### Builder
By default the builder is named `<struct name>Builder`.
For each property of the struct a Builder has a function with the same name.
To build a builder has two different functions `build` and `try_build`. 
`build` panics if not all required values are set. 
In contrast `try_build` can return an error. By default the error is named `<builder name>Error`.

You can change the default identifiers like this:
```rust
#[derive(Builder, PartialEq, Debug)]
#[builder({
    builder_ident: MyUserBuilder,
    error_ident: MyCustomUserBuilderErrorIdent
})]
pub struct User {
    id: usize,
    name: String,
    email: Option<String>
}
```