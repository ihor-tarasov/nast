use crate::{Eval, State, Res, Connect};

pub struct Start(pub usize);

impl Eval for Start {
    fn eval(&self, state: &mut State) -> Res<()> {
        debug_assert_eq!(state.mark(), 0);
        Ok(state.set_state(self.0, 0))
    }
}

impl Connect for Start {
    fn connect(&mut self, port: usize, id: usize) {
        match port {
            0 => self.0 = id,
            _ => panic!(),
        }
    }
}
