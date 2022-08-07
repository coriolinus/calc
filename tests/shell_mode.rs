use calc::Context;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INPUT_RE: Regex = Regex::new(r"^\[\d+\]: (.*)$").unwrap();
}

#[derive(Debug)]
struct ShellCase {
    input: String,
    expect: String,
}

fn parse_expressions(input: &str) -> Vec<ShellCase> {
    let mut lines = input.lines();
    let mut out = Vec::new();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }

        let input = INPUT_RE
            .captures(line)
            .expect("line must match the RE")
            .get(1)
            .expect("capture group 1 always exists")
            .as_str()
            .to_owned();
        let expect = lines.next().expect("each input has an output").to_owned();
        out.push(ShellCase { input, expect });
    }

    out
}

fn assert_expressions(expressions: &[ShellCase]) {
    let mut context = Context::<f64>::default();
    for ShellCase { input, expect } in expressions {
        let result = context.evaluate(&input).unwrap();
        assert_eq!(&result.to_string(), expect);
    }
}

#[test]
fn readme_2_shell_mode() {
    const CASE: &str = r#"
[0]: 1 + 1
2
[1]: 3*(5/(3-4))
-15
[2]: 3*pi**2
29.608813203268074
[3]: @+1
30.608813203268074
[4]: @@@*2
-30
[5]: ln(-1)
NaN
"#;

    let expressions = parse_expressions(CASE);
    assert_expressions(&expressions);
}