pub mod ast;

#[derive(Default)]
pub struct Context<N> {
    expressions: Vec<N>,
}

pub fn evaluate<N>(ctx: &mut Context<N>, expr: &str) -> N {
    unimplemented!()
}
