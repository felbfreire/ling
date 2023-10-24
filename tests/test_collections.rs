use ling::collections;


#[test]
fn test_vector() {
    let mut a_vec: Vec<&str> = collections::make_vec();

    a_vec.push("d");

    assert_eq!(vec!["a", "b", "c", "d"], a_vec);

}

#[test]
fn test_vecdeque() {
    let mut a_vecdeque = collections::make_vecdeque();

    assert_eq!(a_vecdeque[0], "a");
    assert_eq!(a_vecdeque[1], "b");

    a_vecdeque.push_front("d");

    assert_eq!(a_vecdeque[0], "d");
    assert_eq!(a_vecdeque[1], "a");

}

