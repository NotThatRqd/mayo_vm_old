// obsolete, should be part of Memory struct
pub fn create_memory(bytes: usize) -> Vec<u8> {
    if bytes < 8 {
        panic!("bytes parameter must be greater than or equal to 8!");
    }
    vec![0; bytes]
}
