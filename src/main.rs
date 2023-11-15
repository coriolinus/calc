use anyhow::{bail, Result};
use calc::{types::Calcable, Context, Error};
use clap::Parser;
use num_runtime_fmt::Numeric;

#[derive(Debug, Parser)]
#[structopt(about)]
struct Opt {
    /// Use 64-bit floating-point values for calculation
    #[structopt(short, long)]
    f64: bool,

    /// Use unsigned 64-bit integer values for calculation
    #[structopt(short, long)]
    u64: bool,

    /// Use signed 64-bit integer values for calculation
    #[structopt(short, long)]
    i64: bool,

    /// Expression to evaluate
    expression: Vec<String>,
}

impl Opt {
    fn get_type(&self) -> Result<Type> {
        Ok(match (self.f64, self.u64, self.i64) {
            (_, false, false) => Type::F64,
            (false, true, false) => Type::U64,
            (false, false, true) => Type::I64,
            _ => bail!("conflicting fundamental type options"),
        })
    }

    fn expr(&self) -> String {
        self.expression.join(" ")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    F64,
    U64,
    I64,
}

fn eval_as(ty: Type, expr: &str) -> Result<()> {
    match ty {
        Type::F64 => eval_and_print::<f64>(&mut Context::default(), expr),
        Type::U64 => eval_and_print::<u64>(&mut Context::default(), expr),
        Type::I64 => eval_and_print::<i64>(&mut Context::default(), expr),
    }
}

fn eval_and_print<N>(ctx: &mut Context<N>, expr: &str) -> Result<()>
where
    N: std::fmt::Debug + Default + Calcable + Numeric,
    <N as Calcable>::Err: 'static + std::error::Error + Send + Sync,
{
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

fn shell_as(ty: Type) -> Result<()> {
    match ty {
        Type::F64 => shell::<f64>(),
        Type::U64 => shell::<u64>(),
        Type::I64 => shell::<i64>(),
    }
}

fn shell<N>() -> Result<()>
where
    N: std::fmt::Debug + Default + Calcable + Numeric,
    <N as Calcable>::Err: 'static + std::error::Error + Send + Sync,
{
    let mut ctx = Context::<N>::default();
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
        shell_as(opt.get_type()?)
    } else {
        eval_as(opt.get_type()?, &opt.expr())
    }
}
