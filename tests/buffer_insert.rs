use rune::buffer::Buffer;

#[test]
fn example_insert_between_two_chars() {
    let mut buf = Buffer::from_text("ac", 1);
    buf.insert_char('b');
    assert_eq!(buf.text(), "abc");
    assert_eq!(buf.cursor(), 2);
}

#[test]
fn oracle_insert_matches_char_vector() {
    let cases = [
        ("", 0, 'a'),
        ("hi", 2, '!'),
        ("hi", 0, 'X'),
        ("aé", 1, '你'),
        ("你", 1, 'a'),
    ];
    for (text, cursor, ch) in cases {
        let mut buf = Buffer::from_text(text, cursor);
        buf.insert_char(ch);
        let (expect_text, expect_cursor) = slow_insert(text, cursor, ch);
        assert_eq!(
            buf.text(),
            expect_text,
            "text {text:?} cursor {cursor} ch {ch:?}"
        );
        assert_eq!(buf.cursor(), expect_cursor);
    }
}

fn slow_insert(text: &str, cursor: usize, ch: char) -> (String, usize) {
    let mut chars: Vec<char> = text.chars().collect();
    chars.insert(cursor, ch);
    (chars.into_iter().collect(), cursor + 1)
}
