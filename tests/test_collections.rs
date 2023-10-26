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

#[test]
fn test_hashmap() {
    let mut a_hashmap = ling::collections::make_hashmap();

    assert_eq!(3 , a_hashmap["three"]);
}

#[test]
fn test_hashset() {
    let mut a_hashset = ling::collections::make_hashset();

    a_hashset.insert("lol");

    assert_eq!(a_hashset.contains("lol"), true);
    assert_eq!(!a_hashset.contains("lol"), false);
}

