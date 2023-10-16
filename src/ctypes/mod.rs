pub fn vector() -> Vec<u8> {
    let mut vector: Vec<u8> = vec!();

    vector.push(1);
    vector.push(2);
    vector.push(3);

    vector
}

pub fn option(x: u8) -> Option<u8> {
    if x > 1 {
        Some(x)
    } else {
        None
    }
}

pub fn an_array() -> [u8; 3] {
    let arr: [u8; 3] = [1, 2, 3];

    arr
}
