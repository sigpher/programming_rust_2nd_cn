use ch04::{compose, Person};

fn main() {
    let mut composers = compose(Person {
        name: Some("choi".into()),
        birth: 32,
    });
    let lora: Person = Person {
        name: Some("lora".into()),
        birth: 26,
    };
    composers.push(lora);

    let first_name = composers[0].name.take().unwrap();
    println!("{composers:?}");
}
