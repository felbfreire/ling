use ling::fsystem;


#[test]
fn test_io_hl() {
    let input = fsystem::write_hl(); //std::fs::File;
    let output = fsystem::read_hl();

    assert_eq!((), input.unwrap());
    assert_eq!( "Hello World!", output.unwrap());
}

#[test]
fn test_buffered_read() {
    let text = fsystem::read_buffered_hl(); //std::io::BufReader;

    assert_eq!("Hello World!", text.unwrap());
}
