extern crate memchr;

#[inline]
/// Finds the index of the given 'needle' in 'haystack'.
pub fn index(haystack: &[u8], needle: u8) -> Option<usize> {
    memchr::memchr(needle, haystack)
}

#[test]
fn test_index_byte_not_found() {
    let xs: Vec<u8> = (1i32..1025).map(|i| (i % 255) as u8).collect();
    assert_eq!(index(&xs[..], 255), None);

    let xs = [1, 2, 3];
    assert_eq!(index(&xs[..], 100), None);
}

#[test]
fn test_index() {
    let xs: Vec<u8> = (1u8..201).collect();

    for i in 0 .. xs.len() {
        assert_eq!(index(&xs[..], (i+1) as u8), Some(i));
    }
}

#[test]
fn test_index_non16b_start() {
    let xs: Vec<u8> = (1u8 .. 101).collect();
    assert_eq!(index(&xs[1..], 20), Some(18));
}
