use crate::{Eval, State, Res, Connect};

pub struct Argument(pub usize);

impl Eval for Argument {
    fn eval(&self, state: &mut State) -> Res<()> {
        debug_assert_eq!(state.mark(), 0);
        state.pop_state()?;
        Ok(state.load_argument(self.0))
    }
}

impl Connect for Argument {
    fn connect(&mut self, port: usize, id: usize) {
        match port {
            0 => self.0 = id,
            _ => panic!(),
        }
    }
}
