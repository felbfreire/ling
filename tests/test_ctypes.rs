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
