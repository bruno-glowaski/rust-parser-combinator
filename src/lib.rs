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

}
