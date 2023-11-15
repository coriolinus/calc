use calc::{types::Calcable, Context};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INPUT_RE: Regex = Regex::new(r"^\s*\[\d+\]: (.*)$").unwrap();
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
        let expect = lines
            .next()
            .expect("each input has an output")
            .trim()
            .to_owned();
        out.push(ShellCase { input, expect });
    }

    out
}

fn assert_expressions<N>(expressions: &[ShellCase])
where
    N: std::fmt::Debug + Calcable,
    <N as Calcable>::Err: 'static,
    Context<N>: Default,
{
    let mut context = Context::<N>::default();
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
    assert_expressions::<f64>(&expressions);
}

#[test]
fn issue_14_example_1() {
    const CASE: &str = r#"
    [1]: 528500/100
    5285
    [2]: @/2
    2642
    "#;

    let expressions = parse_expressions(CASE);
    assert_expressions::<i64>(&expressions);
}

#[test]
fn issue_14_example_2() {
    const CASE: &str = r#"
    [0]: 2+2*2
    6
    [1]: @*100
    600
    [2]: @*2
    1200
    [3]: @+1
    1201
    "#;

    let expressions = parse_expressions(CASE);
    assert_expressions::<u64>(&expressions);
}
