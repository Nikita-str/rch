use super::*;

fn test_helper(s: &str, expected_tokens: &[(usize, &str)]) {
    let mut tokenizer = SimpleTokenizer::new(s);

    let mut prev_pos = 0;
    for x in expected_tokens {
        let token = tokenizer.next_token().token;
        assert_eq!(token, ((prev_pos, x.0), x.1));
        if prev_pos == x.0 {
            assert!(tokenizer.is_ended());
        }
        prev_pos = x.0;
    }
    assert!(tokenizer.is_ended());
}

#[test]
fn test_tokenizer_01() {
    let s = "today  I'll finally find [xY]xYxo--tag[/xY]";

    let expected_tokens = [
        (5_, "today"),
        (7_, "  "),
        (8_, "I"),
        (9_, "'"),
        (11, "ll"),
        (12, " "),
        (19, "finally"),
        (20, " "),
        (24, "find"),
        (25, " "),
        (26, "["),
        (28, "xY"),
        (29, "]"),
        (33, "xYxo"),
        (34, "-"),
        (35, "-"),
        (38, "tag"),
        (39, "["),
        (40, "/"),
        (42, "xY"),
        (43, "]"),
        (43, ""),
        (43, ""),
    ];

    test_helper(s, &expected_tokens);
}

#[test]
fn test_tokenizer_02() {
    let s = "aaa\n000bbb111c+c\n\ncc";

    let expected_tokens = [
        (3_, "aaa"),
        (4_, "\n"),
        (7_, "000"),
        (10, "bbb"),
        (13, "111"),
        (14, "c"),
        (15, "+"),
        (16, "c"),
        (17, "\n"),
        (18, "\n"),
        (20, "cc"),
    ];

    test_helper(s, &expected_tokens);
}

#[test]
fn test_tokenizer_03() {
    let s = "а сегодня я воздушных шариков купил";
    let expected_words = s.split(' ');
    let mut tokenizer = SimpleTokenizer::new(s);
    for word in expected_words {
        let mut token = tokenizer.next_token().token;
        if token.token == " " {
            token = tokenizer.next_token().token
        }
        assert_eq!(token.token, word)
    }
    assert!(tokenizer.is_ended());
}
