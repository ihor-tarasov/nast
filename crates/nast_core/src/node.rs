use std::collections::HashMap;

use crate::{State, Res, Build};
use crate::nodes::*;

pub trait Eval {
    fn eval(&self, state: &mut State) -> Res<()>;
}

#[derive(Debug)]
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

impl Build for NodeEnum {
    fn push_input(&mut self, name: &String, id: usize, builder: &crate::Builder) -> Res<()> {
        match self {
            NodeEnum::Start(s) => s.push_input(name, id, builder),
            NodeEnum::Number(n) => n.push_input(name, id, builder),
            NodeEnum::Binary(b) => b.push_input(name, id, builder),
            NodeEnum::Argument(a) => a.push_input(name, id, builder),
            NodeEnum::If(i) => i.push_input(name, id, builder),
            NodeEnum::Return(r) => r.push_input(name, id, builder),
            NodeEnum::Call(c) => c.push_input(name, id, builder),
        }
    }

    fn push_flow(&mut self, name: &String, id: usize, builder: &crate::Builder) -> Res<()> {
        match self {
            NodeEnum::Start(s) => s.push_flow(name, id, builder),
            NodeEnum::Number(n) => n.push_flow(name, id, builder),
            NodeEnum::Binary(b) => b.push_flow(name, id, builder),
            NodeEnum::Argument(a) => a.push_flow(name, id, builder),
            NodeEnum::If(i) => i.push_flow(name, id, builder),
            NodeEnum::Return(r) => r.push_flow(name, id, builder),
            NodeEnum::Call(c) => c.push_flow(name, id, builder),
        }
    }

    fn push_content(&mut self, content: &crate::Content, builder: &crate::Builder) -> Res<()> {
        match self {
            NodeEnum::Start(s) => s.push_content(content, builder),
            NodeEnum::Number(n) => n.push_content(content, builder),
            NodeEnum::Binary(b) => b.push_content(content, builder),
            NodeEnum::Argument(a) => a.push_content(content, builder),
            NodeEnum::If(i) => i.push_content(content, builder),
            NodeEnum::Return(r) => r.push_content(content, builder),
            NodeEnum::Call(c) => c.push_content(content, builder),
        }
    }
}

#[derive(Debug)]
pub struct Node(NodeEnum);

pub type Nodes = HashMap<usize, Node>;

impl TryFrom<&String> for Node {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Argument" => Ok(Node(NodeEnum::Argument(Argument::default()))),
            "Subtract" => Ok(Node(NodeEnum::Binary(Binary::from(Operator::Subtract)))),
            "Multiply" => Ok(Node(NodeEnum::Binary(Binary::from(Operator::Multiply)))),
            "LessEquals" => Ok(Node(NodeEnum::Binary(Binary::from(Operator::LessEquals)))),
            "If" => Ok(Node(NodeEnum::If(If::default()))),
            "Number" => Ok(Node(NodeEnum::Number(Number::default()))),
            "Return" => Ok(Node(NodeEnum::Return(Return::default()))),
            "Start" => Ok(Node(NodeEnum::Start(Start::default()))),
            _ => Err(())
        }
    }
}

pub fn call() -> Node {
    Node(NodeEnum::Call(Call::default()))
}

impl Eval for Node {
    fn eval(&self, state: &mut State) -> Res<()> {
        self.0.eval(state)
    }
}

impl Build for Node {
    fn push_input(&mut self, name: &String, id: usize, builder: &crate::Builder) -> Res<()> {
        self.0.push_input(name, id, builder)
    }

    fn push_flow(&mut self, name: &String, id: usize, builder: &crate::Builder) -> Res<()> {
        self.0.push_flow(name, id, builder)
    }

    fn push_content(&mut self, content: &crate::Content, builder: &crate::Builder) -> Res<()> {
        self.0.push_content(content, builder)
    }
}
