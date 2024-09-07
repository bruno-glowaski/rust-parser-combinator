mod parsers;

pub use parsers::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

#[test]
fn a_parser() {
    assert_eq!(the_letter_a("a"), Ok(("", ())));
    assert_eq!(the_letter_a("abc"), Ok(("bc", ())));
    assert_eq!(the_letter_a("cba"), Err("cba"));
}

#[test]
fn literal_parser() {
    let parse_joe = match_literal("Joe");
    assert_eq!(parse_joe.parse("Joe"), Ok(("", ())));
    assert_eq!(parse_joe.parse("Joe! Joseph!"), Ok(("! Joseph!", ())));
    assert_eq!(parse_joe.parse("Robert"), Err("Robert"));
}

#[test]
fn identifier_parser() {
    assert_eq!(
        Ok(("", "i-am-an-identifier".to_string())),
        identifier("i-am-an-identifier")
    );
    assert_eq!(
        Ok((" entirely an identifier", "not".to_string())),
        identifier("not entirely an identifier")
    );
    assert_eq!(Err("!not an identifier"), identifier("!not an identifier"));
}

#[test]
fn pair_combinator() {
    let tag_opener = pair(match_literal("<"), identifier);
    assert_eq!(
        Ok(("/>", ((), "my-first-element".to_string()))),
        tag_opener.parse("<my-first-element/>")
    );
    assert_eq!(Err("oops"), tag_opener.parse("oops"));
    assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
}
