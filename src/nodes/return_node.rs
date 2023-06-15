use crate::{Eval, State, Res, Connect};

pub struct Return(pub usize);

impl Return {
    fn push_return_state(&self, state: &mut State) -> Res<()> {
        state.set_state(state.id(), 1);
        state.push_state();
        state.set_state(self.0, 0);
        Ok(())
    }

    fn pop(&self, state: &mut State) -> Res<()> {
        state.pop_function()?;
        state.pop_state()?;
        Ok(())
    }
}

impl Eval for Return {
    fn eval(&self, state: &mut State) -> Res<()> {
        match state.mark() {
            0 => self.push_return_state(state),
            1 => self.pop(state),
            _ => panic!()
        }
    }
}

impl Connect for Return {
    fn connect(&mut self, port: usize, id: usize) {
        match port {
            0 => self.0 = id,
            _ => panic!(),
        }
    }
}
