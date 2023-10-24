use std::collections;


// Sequences

pub fn make_vec() -> Vec<&'static str> {
    let mut a_vec: Vec<&'static str> = vec!();

    a_vec.push("a");
    a_vec.push("b");
    a_vec.push("c");

    a_vec
}

pub fn make_vecdeque() -> collections::VecDeque<&'static str> {
    let mut a_vecdeque = collections::VecDeque::new();

    a_vecdeque.push_back("a");
    a_vecdeque.push_back("b");
    a_vecdeque.push_back("c");

    a_vecdeque
}

