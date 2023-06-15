use crate::{Eval, State, Res, Value, Connect};

pub struct Number(pub f64);

impl Eval for Number {
    fn eval(&self, state: &mut State) -> Res<()> {
        debug_assert_eq!(state.mark(), 0);
        state.pop_state()?;
        Ok(state.set_value(Value::Number(self.0)))
    }
}

impl Connect for Number {
    fn connect(&mut self, _port: usize, _id: usize) {
        panic!()
    }
}
