use std::{fmt::Display, ops::Mul};

pub fn move_case_primitive_type() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i);
    }
    let third = v[2];
    let fifth = v[4];
    println!("third: {}", third);
    println!("fifth: {}", fifth);
}

pub fn move_case_string() {
    let mut v = Vec::new();
    for i in 1..100 {
        v.push(i.to_string());
    }
    let third = v[2].clone();
    // let fifth = v[4].clone();
    let fifth = v.get(4).unwrap();
    println!("third: {}", third);
    println!("fifth: {}", fifth);
    println!("pop: {}", v.pop().unwrap());
    println!("-----------------");
    let second = v.swap_remove(0);
    println!("{v:?}");

    let first = std::mem::replace(&mut v[0], "substitute".to_string());
    println!("first: {first}");
    println!("{v:?}");

    let v = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];

    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
}

// pub fn move_case_struct() {
//     let tup1 = (1, 2, false, 3.0);
// }


#[derive(Debug)]
pub struct Person {
    pub name: Option<String>,
    pub birth: u32,
}

pub fn compose(person: Person) -> Vec<Person> {
    let mut composers = Vec::new();
    composers.push(person);
    composers
}
