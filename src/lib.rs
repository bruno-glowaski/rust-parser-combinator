mod parsers;

pub use parsers::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

pub fn single_element<'a>() -> impl Parser<'a, Element> {
    map(
        left(element_start(), match_literal("/>")),
        |(name, attributes)| Element { 
            name, 
            attributes, 
            children: vec![],
        }
    )
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

#[test]
fn zero_or_more_combinator() {
    let parser = zero_or_more(match_literal("ha"));
    assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
    assert_eq!(Ok(("", vec![])), parser.parse(""));
    assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
}

#[test]
fn one_or_more_combinator() {
    let parser = one_or_more(match_literal("ha"));
    assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
    assert_eq!(Err(""), parser.parse(""));
    assert_eq!(Err("ahah"), parser.parse("ahah"));
}

#[test]
fn predicate_combinator() {
    let parser = pred(any_char, |c| *c == 'o');
    assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
    assert_eq!(Err("lol"), parser.parse("lol"));
}

#[test]
fn quoted_string_parser() {
    assert_eq!(
        Ok(("", "Hello Joe!".to_string())),
        quoted_string().parse("\"Hello Joe!\"")
    );
}

#[test]
fn attribute_parser() {
    assert_eq!(
        Ok((
            "",
            vec![
                ("one".to_string(), "1".to_string()),
                ("two".to_string(), "2".to_string()),
            ]
        )),
        attributes().parse(" one=\"1\" two=\"2\"")
    );
}

#[test]
fn single_element_parser() {
    assert_eq!(
        Ok((
            "",
            Element {
                name: "div".to_string(),
                attributes: vec![("class".to_string(), "float".to_string())],
                children: vec![]
            }
        )),
        single_element().parse("<div class=\"float\"/>")
    );
}
