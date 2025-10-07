use calc::Context;

struct ExpressionCase {
    input: String,
    expect: String,
}

fn parse_expressions(input: &str) -> Vec<ExpressionCase> {
    let mut lines = input.lines();
    let mut out = Vec::new();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }
        const START: &str = "$ calc \"";
        const END: &str = "\"";
        assert!(
            line.starts_with(START),
            "line must begin with a quoted calc invocation"
        );
        assert!(line.ends_with(END), "line must end with an end quote");
        let input = line[START.len()..(line.len() - END.len())].to_owned();
        let expect = lines.next().expect("each input has an output").to_owned();
        out.push(ExpressionCase { input, expect });
    }

    out
}

fn assert_expressions(expressions: &[ExpressionCase]) {
    for ExpressionCase { input, expect } in expressions {
        let mut context = Context::default();
        let result = context.evaluate(input).unwrap();
        assert_eq!(&result.to_string(), expect);
    }
}

const CASE: &str = r#"
$ calc "1/(2+(3*(4-5)))"
-1
$ calc "round(12345 / 543)"
23
"#;

#[test]
fn readme_1_expression_mode() {
    let expressions = parse_expressions(CASE);
    assert_expressions(&expressions);
}
