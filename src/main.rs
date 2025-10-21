use anyhow::{bail, Context as _, Result};
use calc::{Context, Error};
use clap::Parser;

const BIN_NAME: &str = env!("CARGO_BIN_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");
const GIT_BRANCH: &str = env!("VERGEN_GIT_BRANCH");
const GIT_SHA: &str = env!("VERGEN_GIT_SHA");
const GIT_DIRTY: &str = env!("VERGEN_GIT_DIRTY");

fn formatted_version_data() -> Result<String> {
    use std::fmt::Write;

    let dirty = if GIT_DIRTY == "true" { "(dirty)" } else { "" };

    let mut out = String::new();
    writeln!(&mut out, "{BIN_NAME} v{PKG_VERSION}")?;
    writeln!(&mut out)?;
    writeln!(
        &mut out,
        "git branch:      {GIT_BRANCH} @ {GIT_SHA} {dirty}"
    )?;
    writeln!(&mut out, "build timestamp: {BUILD_TIMESTAMP}")?;

    Ok(out)
}

#[derive(Debug, Parser)]
struct Opt {
    /// Expression to evaluate
    expression: Vec<String>,

    /// Emit version and build information.
    #[arg(short = 'V', long)]
    version: bool,
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

    if opt.version {
        print!(
            "{}",
            formatted_version_data().context("failed to format version data")?
        );
        return Ok(());
    }

    if opt.expression.is_empty() {
        shell()
    } else {
        eval_and_print(&mut Context::default(), &opt.expr())
    }
}
