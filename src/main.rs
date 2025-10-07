use anyhow::{bail, Result};
use calc::{Context, Error};
use clap::Parser;

#[derive(Debug, Parser)]
#[structopt(about)]
struct Opt {
    /// Expression to evaluate
    expression: Vec<String>,
}

impl Opt {
    fn expr(&self) -> String {
        self.expression.join(" ")
    }
}

fn eval_and_print(ctx: &mut Context, expr: &str) -> Result<()> {
    match ctx.evaluate_annotated(expr) {
        Ok(n) => println!("{}", n),
        Err(Error::Eval(err)) => bail!(err),
        Err(Error::Format(err)) => bail!(err),
        Err(Error::Parse(err)) => {
            use lalrpop_util::ParseError::{
                ExtraToken, InvalidToken, UnrecognizedEof, UnrecognizedToken, User,
            };
            match err {
                InvalidToken { location } => {
                    bail!("invalid token\n{}\n{:>pos$}", expr, '^', pos = location + 1)
                }
                UnrecognizedEof { location, .. } => {
                    bail!(
                        "unexpected EOF\n{}\n{:>pos$}",
                        expr,
                        '^',
                        pos = location + 1,
                    );
                }
                UnrecognizedToken {
                    token: (start, _token, end),
                    ..
                } => {
                    bail!(
                        "unexpected token\n{}\n{:pad_l$}{}",
                        expr,
                        "",
                        vec!["^"; end - start].join(""),
                        pad_l = start,
                    );
                }
                ExtraToken {
                    token: (start, _token, end),
                    ..
                } => {
                    bail!(
                        "extra token\n{}\n{:pad_l$}{}",
                        expr,
                        "",
                        vec!["^"; end - start].join(""),
                        pad_l = start,
                    );
                }
                User { error } => bail!(error),
            };
        }
    }
    Ok(())
}

fn shell() -> Result<()> {
    let mut ctx = Context::default();
    let mut rl = rustyline::Editor::<(), _>::new()?;

    loop {
        let expr = rl.readline(&format!("[{}]: ", ctx.history.len()))?;
        if let Err(err) = eval_and_print(&mut ctx, &expr) {
            println!("{}", err);
        } else {
            let _ = rl.add_history_entry(expr);
        }
    }
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    if opt.expression.is_empty() {
        shell()
    } else {
        eval_and_print(&mut Context::default(), &opt.expr())
    }
}
