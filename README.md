# JBE (WIP)
JBE allows you to generate a builder for your struct by using a derive macro and comes with two different macros, Builder and ConsumingBuilder. 
The ConsumingBuilder macro is not yet implemented.
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

## Attributes
All attributes are WIP
### builder_ident
This attribute defines how the builder should be named
```rust
#[derive(Builder)]
#[builder_ident(MyBuilder)]
pub struct MyStruct {
    a: u8
}

fn main() {
    let my_struct = MyBuilder::new().a(25).build();
    assert_eq!(my_struct, MyStruct { a: 25 })
}
```
### error_ident
### builder_default
