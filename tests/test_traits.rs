use ling::ctypes:: {an_array, vector};


// Iterator

fn test_iter_method() {
    
    let arr: [u8; 3] = an_array();
    let mut arr_iter = arr.iter().map(|n| n + 1);

    assert_eq!([1, 2, 3], arr);

    assert_eq!(Some(2), arr_iter.next());
    assert_eq!(Some(3), arr_iter.next());
    assert_eq!(Some(4), arr_iter.next());
    assert_eq!(None, arr_iter.next());
}

fn test_get_iterator_from_vector() {
    // Vec does not impl Iterator but has into_iter method.
    // into_iter moves a_vec, which cant be used anymore.
    // collect collects map result into a Vec (a_mapped_vec)

    let a_vec: Vec<u8> = vector();

    let a_maped_vec: Vec<_> = a_vec
        .into_iter()
        .map(|n| n + 1)
        .collect();

    assert_eq!(vec!(2, 3, 4), a_maped_vec);
}

