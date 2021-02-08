pub mod ast;
pub mod types;

#[derive(Default)]
pub struct Context<N> {
    expressions: Vec<N>,
}

impl<N> Context<N> {
    /// Evaluate an expression in this context.
    pub fn evaluate(&mut self, expr: &str) -> N {
        unimplemented!()
    }
}
