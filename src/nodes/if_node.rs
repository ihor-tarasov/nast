use crate::{Eval, State, Res, Value, Build};

#[derive(Default, Debug)]
pub struct If {
    condition: usize,
    on_then: usize,
    on_else: usize,
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

impl Build for If {
    fn push_input(&mut self, name: &String, id: usize, builder: &crate::Builder) -> Res<()> {
        match name.as_str() {
            "condition" => Ok(self.condition = id),
            _ => Build::push_input(self, name, id, builder)
        }
    }

    fn push_flow(&mut self, name: &String, id: usize, builder: &crate::Builder) -> Res<()> {
        match name.as_str() {
            "then" => Ok(self.on_then = id),
            "else" => Ok(self.on_else = id),
            _ => Build::push_flow(self, name, id, builder)
        }
    }
}
