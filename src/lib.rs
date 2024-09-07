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

#[test]
fn a_parser() {
    assert_eq!(the_letter_a("a"), Ok(("",())));
    assert_eq!(the_letter_a("abc"), Ok(("bc",())));
    assert_eq!(the_letter_a("cba"), Err("cba"));
}
