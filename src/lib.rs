#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {
    match input.chars().next() {
        Some('a') => Ok((&input['a'.len_utf8()..], ())),
        _ => Err(input),
    }
}

fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
    move |input: &str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

#[test]
fn a_parser() {
    assert_eq!(the_letter_a("a"), Ok(("",())));
    assert_eq!(the_letter_a("abc"), Ok(("bc",())));
    assert_eq!(the_letter_a("cba"), Err("cba"));
}

#[test]
fn literal_parser() {
    let parse_joe = match_literal("Joe");
    assert_eq!(
        parse_joe("Joe"),
        Ok(("",())),
    );
    assert_eq!(
        parse_joe("Joe! Joseph!"),
        Ok(("! Joseph!",())),
    );
    assert_eq!(
        parse_joe("Robert"),
        Err("Robert"),
    );
}