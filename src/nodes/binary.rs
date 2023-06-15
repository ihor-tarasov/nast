use crate::{Eval, State, Res, Value, Connect};

#[derive(Clone, Copy)]
pub enum Operator {
    Subtract,
    Multiply,
    LessEquals,
}

fn subtract(left: Value, right: Value) -> Res<Value> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left - right)),
        _ => Err(format!("Unsupported input values {left:?} and {right:?} for Subtract node.")),
    }
}

fn multiply(left: Value, right: Value) -> Res<Value> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left * right)),
        _ => Err(format!("Unsupported input values {left:?} and {right:?} for Multiply node.")),
    }
}

fn less_equals(left: Value, right: Value) -> Res<Value> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Value::Boolean(left <= right)),
        _ => Err(format!("Unsupported input values {left:?} and {right:?} for LessEquals node.")),
    }
}

fn eval_operator(oper: Operator, left: Value, right: Value) -> Res<Value> {
    match oper {
        Operator::Subtract => subtract(left, right),
        Operator::Multiply => multiply(left, right),
        Operator::LessEquals => less_equals(left, right),
    }
}

pub struct Binary {
    pub oper: Operator,
    pub left: usize,
    pub right: usize,
}

impl Binary {
    fn push_left(&self, state: &mut State) -> Res<()> {
        state.set_state(state.id(), 1);
        state.push_state();
        Ok(state.set_state(self.left, 0))
    }

    fn push_right(&self, state: &mut State) -> Res<()> {
        state.set_state(state.id(), 2);
        state.push_state();
        state.push_value();
        Ok(state.set_state(self.right, 0))
    }

    fn eval_internal(&self, state: &mut State) -> Res<()> {
        state.pop_state()?;
        let right = state.get_value();
        state.pop_value();
        let left = state.get_value();
        Ok(state.set_value(eval_operator(self.oper, left, right)?))
    }
}

impl Eval for Binary {
    fn eval(&self, state: &mut State) -> Res<()> {
        match state.mark() {
            0 => self.push_left(state),
            1 => self.push_right(state),
            2 => self.eval_internal(state),
            _ => panic!(),
        }
    }
}

impl Connect for Binary {
    fn connect(&mut self, port: usize, id: usize) {
        match port {
            0 => self.left = id,
            1 => self.right = id,
            _ => panic!(),
        }
    }
}
