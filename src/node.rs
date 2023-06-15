use crate::{State, Res};
use crate::nodes::*;

pub trait Eval {
    fn eval(&self, state: &mut State) -> Res<()>;
}

pub trait Connect {
    fn connect(&mut self, port: usize, id: usize);
}

enum NodeEnum {
    Start(Start),
    Number(Number),
    Binary(Binary),
    Argument(Argument),
    If(If),
    Return(Return),
    Call(Call),
}

impl Eval for NodeEnum {
    fn eval(&self, state: &mut State) -> Res<()> {
        match self {
            Self::Start(s) => s.eval(state),
            Self::Number(n) => n.eval(state),
            Self::Binary(b) => b.eval(state),
            Self::Argument(a) => a.eval(state),
            Self::If(i) => i.eval(state),
            Self::Return(r) => r.eval(state),
            Self::Call(c) => c.eval(state),
        }
    }
}

impl Connect for NodeEnum {
    fn connect(&mut self, port: usize, id: usize) {
        match self {
            Self::Start(s) => s.connect(port, id),
            Self::Number(n) => n.connect(port, id),
            Self::Binary(b) => b.connect(port, id),
            Self::Argument(a) => a.connect(port, id),
            Self::If(i) => i.connect(port, id),
            Self::Return(r) => r.connect(port, id),
            Self::Call(c) => c.connect(port, id),
        }
    }
}

pub struct Node(NodeEnum);

impl Eval for Node {
    fn eval(&self, state: &mut State) -> Res<()> {
        self.0.eval(state)
    }
}

impl Connect for Node {
    fn connect(&mut self, port: usize, id: usize) {
        self.0.connect(port, id)
    }
}

pub fn argument(index: usize) -> Node {
    Node(NodeEnum::Argument(Argument(index)))
}

pub fn subtract(left: usize, right: usize) -> Node {
    Node(NodeEnum::Binary(Binary { oper: Operator::Subtract, left, right }))
}

pub fn multiply(left: usize, right: usize) -> Node {
    Node(NodeEnum::Binary(Binary { oper: Operator::Multiply, left, right }))
}

pub fn less_equals(left: usize, right: usize) -> Node {
    Node(NodeEnum::Binary(Binary { oper: Operator::LessEquals, left, right }))
}

pub fn call(start: usize, args: Vec<usize>) -> Node {
    Node(NodeEnum::Call(Call { start_id: start, arguments: args }))
}

pub fn if_node(cond: usize, on_then: usize, on_else: usize) -> Node {
    Node(NodeEnum::If(If { condition: cond, on_then, on_else }))
}

pub fn number(v: f64) -> Node {
    Node(NodeEnum::Number(Number(v)))
}

pub fn return_node(id: usize) -> Node {
    Node(NodeEnum::Return(Return(id)))
}

pub fn start(id: usize) -> Node {
    Node(NodeEnum::Start(Start(id)))
}
