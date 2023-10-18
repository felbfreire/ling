use ling::ctypes;


#[test]
fn test_ctypes_vec() {
    let mut vector = ctypes::vector();
    let mut another_vector: Vec<u8> = vec!();

    vector.push(4);
    vector[2] = 5;

    for n in &vector {
        another_vector.push(*n);
    }

    assert_eq!(vec!(1, 2, 5, 4), vector);
    assert_eq!(vec!(1, 2, 5, 4), another_vector);
}

#[test]
fn test_ctypes_vec_add_range_for() {
    let mut vec = ctypes::vector();
    let mut cap_vec = Vec::with_capacity(6);

    for n in 4..7 {
        vec.push(n);
    }

    for n in 1..7 {
        cap_vec.push(n);
    }

    assert_eq!(vec!(1, 2, 3, 4, 5, 6), vec);
    assert_eq!(vec!(1, 2, 3, 4, 5, 6), cap_vec);
}

#[test]
fn test_slice() {
    let mut vec1 = ctypes::vector();
    let mut vec2 = ctypes::vector();
    let full_slice = &mut vec1[..];
    let not_full_slice = &mut vec2[0..2];

    assert_eq!(&[1, 2, 3], full_slice);
    assert_eq!(&[1, 2], not_full_slice); // cant borrow
}

#[test]
fn test_option_type() {
    let some = ctypes::option(3);
    let none = ctypes::option(0);

    assert_eq!(Some(3), some);
    assert_eq!(None, none);
}

#[test]
fn test_array_type() {
    let arr = ctypes::an_array();

    assert_eq!([1, 2, 3], arr);
}

