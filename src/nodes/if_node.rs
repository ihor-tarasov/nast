use crate::{Eval, State, Res, Value, Connect};

pub struct If {
    pub condition: usize,
    pub on_then: usize,
    pub on_else: usize,
}

impl If {
    fn push_condition(&self, state: &mut State) -> Res<()> {
        state.set_state(state.id(), 1);
        state.push_state();
        Ok(state.set_state(self.condition, 0))
    }

    fn push_then_or_else(&self, state: &mut State) -> Res<()> {
        let condition = state.get_value();
        let condition = match condition {
            Value::Boolean(v) => v,
            _ => return Err(format!("For If node Boolean type expected for 'condition' input, found {condition:?}"))
        };
        if condition {
            Ok(state.set_state(self.on_then, 0))
        } else {
            Ok(state.set_state(self.on_else, 0))
        }
    }
}

impl Eval for If {
    fn eval(&self, state: &mut State) -> Res<()> {
        match state.mark() {
            0 => self.push_condition(state),
            1 => self.push_then_or_else(state),
            _ => panic!(),
        }
    }
}

impl Connect for If {
    fn connect(&mut self, port: usize, id: usize) {
        match port {
            0 => self.condition = id,
            1 => self.on_then = id,
            2 => self.on_else = id,
            _ => panic!(),
        }
    }
}