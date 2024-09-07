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
    // move |input: &str| match input.get(0..expected.len()) {
    //     Some(next) if next == expected => Ok((&input[expected.len()..], ())),
    //     _ => Err(input),
    // }
    move |input: &str| match input.strip_prefix(expected) {
        Some(remainder) => Ok((remainder, ())),
        _ => Err(input),
    }
}

fn identifier(input: &str) -> Result<(&str, String), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    // First character needs to be alphabetic
    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    // The next characters need to be either alphanumeric or a dash.
    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}

fn pair<P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Fn(&str) -> Result<(&str, (R1, R2)), &str>
where
    P1: Fn(&str) -> Result<(&str, R1), &str>,
    P2: Fn(&str) -> Result<(&str, R2), &str>,
{
    move |input: &str| match parser1(input) {
        Ok((next_input, result1)) => match parser2(next_input) {
            Ok((remainder, result2)) => Ok((remainder, (result1, result2))),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
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
    assert_eq!(parse_joe("Joe"), Ok(("", ())));
    assert_eq!(parse_joe("Joe! Joseph!"), Ok(("! Joseph!", ())));
    assert_eq!(parse_joe("Robert"), Err("Robert"));
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
        tag_opener("<my-first-element/>")
    );
    assert_eq!(Err("oops"), tag_opener("oops"));
    assert_eq!(Err("!oops"), tag_opener("<!oops"));
}
