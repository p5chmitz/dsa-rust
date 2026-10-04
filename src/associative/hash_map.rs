#![allow(unused)]
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Key<'a> {
    Name(&'a str),
    Food(&'a str),
    Age(u8),
    Active,
}

enum Val<'a> {
    Name(&'a str),
    Food(&'a str),
    Age(u8),
    Bool(bool),
    Person,
    Place,
    Thing,
}

struct Person {
    name: String,

}

fn test() {
    //let mut map = HashMap::<MapKey, MapVal>::new();
    HashMap::from([
        (Key::Name("Bobson"), Val::Age(36)),
        (Key::Age(36), Val::Food("hamburgers")),
        (Key::Active, Val::Bool(true)),
        (Key::Active, Val::Person)
    ]);
}
