pub fn create_memory(bytes: usize) -> Vec<u8> {
    if bytes == 0 {
        panic!("bytes parameter can't be 0!");
    }
    vec![0; bytes]
}
