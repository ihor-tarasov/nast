use crate::{Eval, State, Res};

pub struct Call {
    pub start_id: usize,
    pub arguments: Vec<usize>,
}

impl Call {
    fn push_argument(&self, state: &mut State) -> Res<()> {
        if state.mark() != 0 {
            state.push_value();
        }
        state.set_state(state.id(), state.mark() + 1);
        state.push_state();
        state.set_state(self.arguments[state.mark() - 1], 0);
        Ok(())
    }

    fn push_function(&self, state: &mut State) -> Res<()> {
        state.push_value();
        let arguments = state.get_values(self.arguments.len());
        state.push_function(arguments);
        state.set_state(self.start_id, 0);
        Ok(())
    }
}

impl Eval for Call {
    fn eval(&self, state: &mut State) -> Res<()> {
        if state.mark() < self.arguments.len() {
            self.push_argument(state)
        } else if state.mark() == self.arguments.len() {
            self.push_function(state)
        } else {
            panic!()
        }
    }
}
