
pub fn print_char_from_a_to_b(char_start: char, char_end: char) -> String {
     let char_start = char_start as u8;
     let char_end = char_end as u8;

    let (idx_start, idx_end) = if char_start > char_end {
        (char_end, char_start)
    } else {
        (char_start, char_end)
    };

    let mut ret = String::new();
    for i in idx_start + 1..idx_end {
        ret.push(i as char);
    }
    if char_start > char_end {
        ret = ret.chars().rev().collect();
    }
    ret
}
#[test]
fn test_print_char_from_a_to_b() {
    assert_eq!(print_char_from_a_to_b('a', 'e'), "bcd");
    assert_eq!(print_char_from_a_to_b('e', 'a'), "dcb");
}