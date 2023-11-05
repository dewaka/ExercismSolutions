use std::collections::HashMap;

fn plant(p: char) -> Option<&'static str> {
    match p {
        'V' => Some("violets"),
        'R' => Some("radishes"),
        'C' => Some("clover"),
        'G' => Some("grass"),
        _ => None,
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {

    let mut children = vec![
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    children.sort();
    let children : HashMap<&str, i32> = children.into_iter().zip(0..).collect();
    dbg!(&children);

    if !children.contains_key(student) {
        return vec![];
    }

    let lines: Vec<&str> = diagram.lines().collect();

    unimplemented!("Solve kindergarten-garden exercise");
}
