use crate::{State, Res};
use crate::nodes::*;

pub trait Eval {
    fn eval(&self, state: &mut State) -> Res<()>;
}

pub enum Node {
    Start(Start),
    Number(Number),
    Binary(Binary),
    Argument(Argument),
    If(If),
    Return(Return),
    Call(Call),
}

impl Eval for Node {
    fn eval(&self, state: &mut State) -> Res<()> {
        match self {
            Node::Start(s) => s.eval(state),
            Node::Number(n) => n.eval(state),
            Node::Binary(b) => b.eval(state),
            Node::Argument(a) => a.eval(state),
            Node::If(i) => i.eval(state),
            Node::Return(r) => r.eval(state),
            Node::Call(c) => c.eval(state),
        }
    }
}
