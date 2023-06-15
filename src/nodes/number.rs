use crate::{Eval, State, Res, Value};

pub struct Number(pub f64);

impl Eval for Number {
    fn eval(&self, state: &mut State) -> Res<()> {
        debug_assert_eq!(state.mark(), 0);
        state.pop_state()?;
        Ok(state.set_value(Value::Number(self.0)))
    }
}
